#![doc = include_str!("../README.md")]

use std::fmt::Write;

use csv::{ReaderBuilder, StringRecord, Trim};

use nom::{
    branch::alt,
    bytes::complete::{tag, take, take_while},
    character::complete::{alpha1, digit1, not_line_ending},
    combinator::{eof, map, map_res},
    multi::many1,
    sequence::{preceded, separated_pair, terminated},
    IResult,
};
use std::str::FromStr;

struct RegistryRecord<'a> {
    country_code: &'a str,
    bban: &'a str,
    iban_electronic: &'a str,
    iban_print: &'a str,
    bank_identifier_position: Option<(usize, usize)>,
    bank_identifier_pattern: Option<Vec<&'a str>>,
    bank_identifier_example: Option<&'a str>,
    branch_identifier_position: Option<(usize, usize)>,
    branch_identifier_example: Option<&'a str>,
    iban_structure: Vec<(&'a str, &'a str)>,
}

struct RegistryReader<'a> {
    records: Vec<RegistryRecord<'a>>,
}

impl<'a> RegistryReader<'a> {
    fn new(records_transposed: &'a [StringRecord]) -> anyhow::Result<Self> {
        let records: Vec<RegistryRecord<'a>> = (1..records_transposed[0].len())
            .map(|i| -> anyhow::Result<_> {
                Ok(RegistryRecord {
                    country_code: &records_transposed[2][i],
                    bban: &records_transposed[16][i],
                    iban_electronic: &records_transposed[21][i],
                    iban_print: &records_transposed[22][i],
                    bank_identifier_position: maybe(parse_range)(&records_transposed[10][i])
                        .unwrap()
                        .1,
                    bank_identifier_pattern: maybe(potentially_malformed_pattern)(
                        &records_transposed[11][i],
                    )
                    .unwrap()
                    .1,
                    bank_identifier_example: maybe(not_line_ending)(&records_transposed[14][i])
                        .unwrap()
                        .1,
                    branch_identifier_position: maybe(parse_range)(&records_transposed[12][i])
                        .unwrap()
                        .1,
                    branch_identifier_example: maybe(not_line_ending)(&records_transposed[15][i])
                        .unwrap()
                        .1,
                    iban_structure: iban_structure(&records_transposed[18][i]).unwrap().1,
                })
            })
            .collect::<Result<_, _>>()
            .unwrap();
        Ok(RegistryReader { records })
    }
}

fn main() -> anyhow::Result<()> {
    // By trimming and escaping double quotes we fix entries like `"1-5\n"` (double quotes included).
    let mut reader = ReaderBuilder::new()
        .delimiter(b'\t')
        .double_quote(true)
        .has_headers(false)
        .trim(Trim::All)
        .from_path("./swift_iban_registry.txt")?;

    let records_transposed: Vec<StringRecord> = reader.records().collect::<Result<_, _>>()?;
    let registry = RegistryReader::new(&records_transposed)?;

    println!(
        "BANK_IDENTIFIER:\n{}",
        generate_bank_identifier_position_in_bban_match_arm(&registry)?
    );

    println!(
        "BRANCH_IDENTIFIER:\n{}",
        generate_branch_identifier_position_in_bban_match_arm(&registry)?
    );

    println!(
        "FORMAT_VALIDATION:\n{}",
        generate_format_match_arm(&registry)?
    );

    println!("TEST_FILE:\n{}", generate_test_file(&registry)?);

    Ok(())
}

fn generate_bank_identifier_position_in_bban_match_arm(
    contents: &RegistryReader,
) -> anyhow::Result<String> {
    let mut s = String::new();
    for record in &contents.records {
        if let Some((mut start, mut end)) = record.bank_identifier_position {
            // Convert from one-indexed inclusive-inclusive to zero-indexed inclusive-exclusive.
            start -= 1;

            // The info for Jordan is just incorrect. Adjust manually.
            if record.country_code == "JO" {
                writeln!(
                    &mut s,
                    "// Jordan has an incorrect bank identifier range in the registry."
                )?;
                start = 0;
                end = 4;
            }

            // Test for inconsistencies in the input file and leave a comment.
            // Namely, deduce the pattern length and check if it matches the range.
            if let Some(bank_identifier_pattern) = &record.bank_identifier_pattern {
                let bank_identifier_length = bank_identifier_pattern
                    .iter()
                    .map(|len| len.parse::<usize>().unwrap())
                    .sum();

                if end - start != bank_identifier_length {
                    writeln!(&mut s, "// The bank identifier length ({}) does not match the range ({}..{}) in the registry. Using length as truth.", bank_identifier_length, start, end)?;
                    end = start + bank_identifier_length;
                }

                // As a final check, see if the example BBAN and and bank identifier match.
                // Skip some countries since the examples don't match.
                // For ST, weirdly the PDF BBAN does match the bank_identifier, but the txt doesn't.
                let bank_identifier_example: String = record
                    .bank_identifier_example
                    .unwrap_or_else(|| {
                        panic!(
                            "expected a bank identifier example for country {}",
                            record.country_code
                        )
                    })
                    .chars()
                    // Remove formatting like spaces and dashes
                    .filter(|c| c.is_ascii_alphanumeric())
                    .collect();
                if matches!(record.country_code, "MK" | "SE" | "ST") {
                    assert_eq!(bank_identifier_example.len(), bank_identifier_length);
                } else {
                    // Sometimes the BBAN is just different so we should use the BBAN and not the IBAN. Sometimes the BBAN removes leading zeros or
                    // has weird formatting. Just check both and be happy if one matches.
                    assert!(record.bban[start..end] == bank_identifier_example
                        || record.iban_electronic[start + 4..end + 4] == bank_identifier_example,
                        "the example bank code does not match the example bban/iban for country {}. Expected {} or {} but found {}", record.country_code, &record.bban[start..end], &record.iban_electronic[start + 4..end + 4], &bank_identifier_example);
                }
            }
            writeln!(
                &mut s,
                "\"{}\" => Some({}..{}),",
                record.country_code, start, end
            )?;
        } else {
            // No range given.
            writeln!(&mut s, "\"{}\" => None,", record.country_code)?;
        }
    }
    Ok(s)
}

/// Parse using the inner function but accept an empty string or "N/A" as `None`.
fn maybe<'a, T>(
    f: impl FnMut(&'a str) -> IResult<&'a str, T>,
) -> impl FnMut(&'a str) -> IResult<&'a str, Option<T>> {
    alt((map(alt((eof, tag("N/A"))), |_| None), map(f, Some)))
}

fn parse_range(input: &str) -> IResult<&str, (usize, usize)> {
    separated_pair(
        map_res(digit1, usize::from_str),
        tag("-"),
        map_res(digit1, usize::from_str),
    )(input)
}

#[test]
fn test_maybe_parse_range() {
    let mut maybe_parse_range = maybe(parse_range);
    assert_eq!(maybe_parse_range(""), Ok(("", None)));
    assert_eq!(maybe_parse_range("N/A"), Ok(("", None)));
    assert_eq!(maybe_parse_range("1-4"), Ok(("", Some((1, 4)))));
}

fn generate_branch_identifier_position_in_bban_match_arm(
    contents: &RegistryReader,
) -> anyhow::Result<String> {
    let mut s = String::new();
    for record in &contents.records {
        if let Some((mut start, mut end)) = record.branch_identifier_position {
            // Convert from one-indexed inclusive-inclusive to zero-indexed inclusive-exclusive.
            start -= 1;

            // Just do some sanity check. That actually fails sometimes...

            if let Some(branch_identifier_example) = record.branch_identifier_example {
                if branch_identifier_example.len() != end - start {
                    if branch_identifier_example.len() == (end - 1) - start {
                        // Assume that the end of the range is accidentally exclusive, unlike the other entries.
                        writeln!(&mut s, "// The registry branch example (\"{}\") does not have the length as expected from the position range ({}..{}).\n// Assume the example is correct, see generation code for details.", branch_identifier_example, start, end)?;
                        end -= 1;
                    } else {
                        panic!("The registry branch example (\"{}\") does not have the length as expected from the position range ({}..{}) and it can't be fixed.", branch_identifier_example, start, end);
                    }
                }
            } else {
                // This happens for Jordan. The correct thing to do seems to be
                // to assume that there just isn't an example.
                // Note that the .PDF version of the registry is incorrect.
                // The bank position should be 1-4 but is 5-8, the branch
                // position should be 5-8 but is empty.
                // The bank position is also incorrect in the .txt, the fix is
                // hardcoded in the bank identifier function.
                writeln!(&mut s, "// The registry doesn't provide an example.")?;
            }
            writeln!(
                &mut s,
                "\"{}\" => Some({}..{}),",
                record.country_code, start, end
            )?;
        } else {
            writeln!(&mut s, "\"{}\" => None,", record.country_code)?;
        }
    }
    Ok(s)
}

fn parse_pattern(contents: &str) -> IResult<&str, Vec<(&str, &str)>> {
    many1(separated_pair(digit1, tag("!"), alpha1))(contents)
}

/// Parse a misformed pattern. Now we're desperate: just find the numbers in the input and ignore the rest.
fn parse_malformed_pattern(contents: &str) -> IResult<&str, Vec<&str>> {
    many1(terminated(
        digit1,
        take_while(|s: char| !s.is_ascii_digit()),
    ))(contents)
}

fn potentially_malformed_pattern(contents: &str) -> IResult<&str, Vec<&str>> {
    alt((
        map(parse_pattern, |a: Vec<(&str, &str)>| {
            a.iter().map(|a| a.0).collect()
        }),
        parse_malformed_pattern,
    ))(contents)
}

fn iban_structure(contents: &str) -> IResult<&str, Vec<(&str, &str)>> {
    preceded(
        // Skip country code and check digits
        take(5_usize),
        parse_pattern,
    )(contents)
}

fn generate_format_match_arm(contents: &RegistryReader) -> anyhow::Result<String> {
    let mut s = String::new();
    for record in &contents.records {
        // TODO: Maybe combine sequences of the same character. The compiler will probably optimize this anyway though.
        let pos_formatted = record
            .iban_structure
            .iter()
            .map(|(num, t)| format!("({}, {})", num, t.to_ascii_uppercase()))
            .collect::<Vec<_>>()
            .join(", ");
        writeln!(
            &mut s,
            "\"{}\" => Some([{}].borrow()),",
            record.country_code, pos_formatted
        )?;
    }
    Ok(s)
}

#[derive(Debug)]
#[allow(dead_code)] // Allow since it is used for printing
struct RegistryExample<'a> {
    country_code: &'a str,
    bank_identifier: Option<&'a str>,
    branch_identifier: Option<&'a str>,
    bban: &'a str,
    iban_electronic: &'a str,
    iban_print: &'a str,
}

fn generate_test_file(contents: &RegistryReader) -> anyhow::Result<String> {
    let mut s = String::new();
    for record in &contents.records {
        writeln!(
            &mut s,
            "{:#?},",
            RegistryExample {
                country_code: record.country_code,
                bank_identifier: record.bank_identifier_example,
                branch_identifier: record.branch_identifier_example,
                bban: record.bban,
                iban_electronic: record.iban_electronic,
                iban_print: record.iban_print
            }
        )?;
    }
    Ok(s)
}
