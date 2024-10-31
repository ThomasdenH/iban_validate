#![doc = include_str!("../README.md")]

use std::{
    fs::File,
    io::{Read, Write},
    ops::Range,
};

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

#[derive(Debug)]
struct RegistryRecord<'a> {
    country_code: &'a str,
    bban: &'a str,
    iban_electronic: &'a str,
    iban_print: &'a str,
    bank_identifier_position: Option<Range<usize>>,
    bank_identifier_pattern: Option<Vec<&'a str>>,
    bank_identifier_example: Option<&'a str>,
    branch_identifier_position: Option<Range<usize>>,
    branch_identifier_example: Option<&'a str>,
    iban_structure: Vec<(&'a str, &'a str)>,
}

impl<'a> RegistryRecord<'a> {
    /// Fix all errors, inconsistencies and missing entries in the registry.
    ///
    /// This method is immediately also a collection of the errors contained in
    /// the registry. For the most part, this is just a bank or branch item
    /// that does not match the IBAN, which is not wrong, it just mean we can't
    /// use it for testing.
    fn fix_inconsistencies(&mut self) {
        match self.country_code {
            "AL" => {
                // These seem to incorrectly include the branch as well as the
                // national check digit. Correct them manually.
                assert_eq!(self.bank_identifier_pattern, Some(vec!["8"]));
                assert_eq!(self.bank_identifier_example, Some("212-1100-9"));
                self.bank_identifier_pattern = Some(vec!["3"]);
                self.bank_identifier_example = Some("212");

                // Correct branch range that was specified as exclusive where they should have been inclusive.
                self.branch_identifier_position.as_mut().unwrap().end -= 1;
            }
            "BA" => {
                // The BBAN does not match the IBAN. The bank and branch match
                // the BBAN. Manually fix all three to correspond to IBAN.
                assert_eq!(self.bban, "1990440001200279");
                assert_eq!(self.bank_identifier_example, Some("199"));
                assert_eq!(self.branch_identifier_example, Some("044"));
                self.bban = "1290079401028494";
                self.bank_identifier_example = Some("129");
                self.branch_identifier_example = Some("007");
            }
            "BI" => {
                // Pretty print format is incorrect, fix.
                assert_eq!(self.iban_print, "BI42 10000 10001 00003320451 81");
                self.iban_print = "BI42 1000 0100 0100 0033 2045 181";
            }
            "BR" => {
                // The BBAN differs by one letter. Fix.
                assert_eq!(self.bban, "00360305000010009795493P1");
                self.bban = "00360305000010009795493C1";
            }
            "CR" => {
                // The BBAN removes the leading '0'. Add it back.
                assert_eq!(self.bban, "15202001026284066");
                self.bban = "015202001026284066";
            }
            "FI" => {
                // Not provided, add manually
                assert!(self.bank_identifier_pattern.is_none());
                self.bank_identifier_pattern = Some(vec!["3"]);

                // The BBAN is not provided, add manually as well.
                assert_eq!(self.bban, "N/A");
                self.bban = "12345600000785";
            }
            "IL" => {
                // This looks like a typo. There is one 0 missing in the BBAN.
                assert_eq!(self.bban, "010800000099999999");
                self.bban = "0108000000099999999";
            }
            "JO" => {
                // Fix the bank position. Perhaps it was indexed into the IBAN
                // instead of the BBAN?
                assert_eq!(self.bank_identifier_position, Some(4..8));
                self.bank_identifier_position = Some(0..4);

                // There is no example of the branch even though there is a range.
                // We will just use the range and set the example manually.
                // https://www.xe.com/nl/ibancalculator/jordan/
                assert!(self.branch_identifier_example.is_none());
                self.branch_identifier_example = Some("0010");

                // Note that the .PDF version of the registry is also
                // incorrect, but differently. The bank position should be 1-4
                // but is 5-8, the branch position should be 5-8 but is empty.
            }
            "LY" => {
                // Incorrect spacing.
                assert_eq!(self.iban_print, "LY83 002 048 000020100120361");
                self.iban_print = "LY83 0020 4800 0020 1001 2036 1";
            }
            "MK" => {
                // The bank identifier does not match the BBAN or IBAN.
                assert_eq!(self.bank_identifier_example, Some("300"));
                self.bank_identifier_example = Some("250");
            }
            "NI" => {
                // Check digit incorrect!
                assert_eq!(self.iban_electronic, "NI04BAPR00000013000003558124");
                assert_eq!(self.iban_print, "NI04 BAPR 0000 0013 0000 0355 8124");
                self.iban_electronic = "NI45BAPR00000013000003558124";
                self.iban_print = "NI45 BAPR 0000 0013 0000 0355 8124";
            }
            "RU" => {
                // Check digit incorrect!
                assert_eq!(self.iban_electronic, "RU1704452522540817810538091310419");
                assert_eq!(self.iban_print, "RU17 0445 2522 5408 1781 0538 0913 1041 9");
                self.iban_electronic = "RU0304452522540817810538091310419";
                self.iban_print = "RU03 0445 2522 5408 1781 0538 0913 1041 9";
            }
            "SE" => {
                // The bank identifier does not match.
                assert_eq!(self.bank_identifier_example, Some("123"));
                self.bank_identifier_example = Some("500");
            }
            "ST" => {
                // The IBAN and BBAN differ from the PDF, but the bank was not
                // updated.
                assert_eq!(self.bank_identifier_example, Some("0001"));
                self.bank_identifier_example = Some("0002");

                // Check digit incorrect!
                assert_eq!(self.iban_electronic, "ST68000200010192194210112");
                assert_eq!(self.iban_print, "ST68 0002 0001 0192 1942 1011 2");
                self.iban_electronic = "ST32000200010192194210112";
                self.iban_print = "ST32 0002 0001 0192 1942 1011 2";
            }
            "SV" => {
                assert_eq!(self.iban_print, "SV 62 CENR 00000000000000700025");
                self.iban_print = "SV62 CENR 0000 0000 0000 0070 0025";
            }
            "VA" => {
                assert_eq!(self.iban_print, "VA59 001 1230 0001 2345 678");
                self.iban_print = "VA59 0011 2300 0012 3456 78";
            }
            _ => {}
        }
    }

    fn check(&mut self) {
        // Test for inconsistencies in the input file. We do this by
        // considering the bank identifier pattern (i.e. "4!n") and comparing
        // its length to the range.
        if let Some(bank_position) = &self.bank_identifier_position {
            let bank_pattern = self
                .bank_identifier_pattern
                .as_ref()
                .expect("we expect the bank pattern to be given if the position is");

            // We compute the length from the pattern, i.e. "4!n" implies a
            // length of 4. Only the numbers have been retained during
            // parsing.
            let bank_identifier_length = bank_pattern
                .iter()
                .map(|len| len.parse::<usize>().unwrap())
                .sum();

            assert_eq!(
                bank_position.end - bank_position.start,
                bank_identifier_length,
                "expect the bank pattern length to be equal to the size of the range"
            );

            // Get the example bank identifier.
            let bank_example: String = self
                .bank_identifier_example
                .expect("expected an example bank identifier")
                .chars()
                // Remove formatting like spaces and dashes.
                .filter(|c| c.is_ascii_alphanumeric())
                .collect();

            // We check that the bank identifier matches the claimed length.
            assert_eq!(bank_example.len(), bank_identifier_length);

            // As a final check, see if the example BBAN and and bank
            // identifier match.
            assert_eq!(self.bban[bank_position.clone()], bank_example);
            assert_eq!(
                self.iban_electronic[bank_position.start + 4..bank_position.end + 4],
                bank_example
            );
        } else {
            // No bank position. We don't expect a length or an example either.
            assert!(self.bank_identifier_example.is_none());
            assert!(self.bank_identifier_pattern.is_none());
        }

        // Branch info
        if let Some(branch_position) = &self.branch_identifier_position {
            let branch_example = self.branch_identifier_example.expect("expected example");
            assert_eq!(
                branch_example.len(),
                branch_position.len(),
                "expected branch example to match position"
            );
        } else {
            assert!(
                self.branch_identifier_example.is_none(),
                "expected no example"
            );
        }
    }
}

struct RegistryReader<'a> {
    records: Vec<RegistryRecord<'a>>,
}

impl<'a> RegistryReader<'a> {
    fn new(records_transposed: &'a [StringRecord]) -> anyhow::Result<Self> {
        let mut records: Vec<RegistryRecord<'a>> = (1..records_transposed[0].len())
            .map(|i| -> anyhow::Result<_> {
                Ok(RegistryRecord {
                    country_code: &records_transposed[2][i],
                    bban: &records_transposed[16][i],
                    iban_electronic: &records_transposed[21][i],
                    iban_print: &records_transposed[22][i],
                    bank_identifier_position: maybe(parse_range)(&records_transposed[10][i])
                        .unwrap()
                        .1
                        .map(|(start, end)| ((start - 1)..end)),
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
                        .1
                        .map(|(start, end)| (start - 1)..end),
                    branch_identifier_example: maybe(not_line_ending)(&records_transposed[15][i])
                        .unwrap()
                        .1,
                    iban_structure: iban_structure(&records_transposed[18][i]).unwrap().1,
                })
            })
            .collect::<Result<_, _>>()
            .unwrap();
        for record in &mut records {
            record.fix_inconsistencies();
            record.check();
        }
        Ok(RegistryReader { records })
    }
}

const FILE_PATH: &str = "./swift_iban_registry.txt";

/// Fix the UTF8 of a file by performing a lossless conversion.
fn fix_utf8(file_name: &str) -> anyhow::Result<()> {
    // The file is invalid utf8, so we will first process it.
    let buf = {
        let mut file = File::open(file_name)?;
        let mut buf = Vec::new();
        file.read_to_end(&mut buf)?;
        buf
    };
    let contents = String::from_utf8_lossy(&buf);
    File::create(file_name)?.write_all(contents.as_bytes())?;
    Ok(())
}

fn main() -> anyhow::Result<()> {
    // The registry file is actually invalid UTF8, so first try to fix it.
    fix_utf8(FILE_PATH)?;

    // By trimming and escaping double quotes we fix entries like `"1-5\n"` (double quotes included).
    let mut reader = ReaderBuilder::new()
        .delimiter(b'\t')
        .double_quote(true)
        .has_headers(false)
        .trim(Trim::All)
        .from_path(FILE_PATH)?;

    let records_transposed: Vec<StringRecord> = reader.records().collect::<Result<_, _>>()?;
    let registry = RegistryReader::new(&records_transposed)?;

    // Generate this file for checking and getting country specific info.
    let mut generated_file = File::create("../iban_validate/src/generated.rs")?;
    writeln!(generated_file, "//! This file is automatically generated by `iban_validate_registry_generation` from the IBAN registry.")?;
    generate_bank_identifier_position_in_bban_match_arm(&mut generated_file, &registry)?;
    writeln!(generated_file)?;
    generate_branch_identifier_position_in_bban_match_arm(&mut generated_file, &registry)?;
    writeln!(generated_file)?;
    generate_format_match_arm(&mut generated_file, &registry)?;

    // Generate this file with test cases.
    let mut generated_file = File::create("../iban_validate/tests/registry_examples_generated.rs")?;
    generate_test_file(&mut generated_file, &registry)?;

    Ok(())
}

fn generate_bank_identifier_position_in_bban_match_arm(
    mut writer: &mut impl Write,
    contents: &RegistryReader,
) -> anyhow::Result<()> {
    writeln!(
        writer,
        "
/// Get the position of the bank in the BBAN.
#[inline]
pub(crate) fn bank_identifier(country_code: &str) -> Option<core::ops::Range<usize>> {{
\t#[allow(clippy::match_same_arms)] // For clarity, identical arms are not combined.
\tmatch country_code {{"
    )?;
    for record in &contents.records {
        if let Some(bank_identifier_position) = &record.bank_identifier_position {
            writeln!(
                &mut writer,
                "\t\t\"{}\" => Some({}..{}),",
                record.country_code, bank_identifier_position.start, bank_identifier_position.end
            )?;
        } else {
            writeln!(&mut writer, "\t\t\"{}\" => None,", record.country_code)?;
        }
    }
    writeln!(writer, "\t\t_ => None,")?;
    writeln!(writer, "\t}}\n}}")?;
    Ok(())
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

/// Generate match arms for the branch range in the IBAN.
fn generate_branch_identifier_position_in_bban_match_arm(
    write: &mut impl Write,
    contents: &RegistryReader,
) -> anyhow::Result<()> {
    writeln!(
        write,
        "/// Get the position of the branch in the BBAN.
#[inline]
pub(crate) fn branch_identifier(country_code: &str) -> Option<core::ops::Range<usize>> {{
\t#[allow(clippy::match_same_arms)] // For clarity, identical arms are not combined.
\tmatch country_code {{"
    )?;

    for record in &contents.records {
        if let Some(branch_position) = record.branch_identifier_position.clone() {
            writeln!(
                write,
                "\t\t\"{}\" => Some({}..{}),",
                record.country_code, branch_position.start, branch_position.end
            )?;
        } else {
            writeln!(write, "\t\t\"{}\" => None,", record.country_code)?;
        }
    }
    writeln!(write, "\t\t_ => None,")?;
    writeln!(write, "\t}}\n}}")?;
    Ok(())
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

/// Parse a pattern that repeatedly contains the form "4!a". Only the length is stored.
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

fn generate_format_match_arm(
    write: &mut impl Write,
    contents: &RegistryReader,
) -> anyhow::Result<()> {
    writeln!(
        write,
        "use crate::countries::CharacterType;

#[inline]
pub(crate) fn country_pattern(country_code: &str) -> Option<&[(usize, CharacterType)]> {{
\tuse CharacterType::*;
\tuse core::borrow::Borrow;
\tmatch country_code {{"
    )?;
    for record in &contents.records {
        // TODO: Maybe combine sequences of the same character. The compiler will probably optimize this anyway though.
        let pos_formatted = record
            .iban_structure
            .iter()
            .map(|(num, t)| format!("({}, {})", num, t.to_ascii_uppercase()))
            .collect::<Vec<_>>()
            .join(", ");
        writeln!(
            write,
            "\t\t\"{}\" => Some([{}].borrow()),",
            record.country_code, pos_formatted
        )?;
    }
    writeln!(write, "\t\t_ => None")?;
    writeln!(write, "\t}}\n}}")?;
    Ok(())
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

fn generate_test_file(write: &mut impl Write, contents: &RegistryReader) -> anyhow::Result<()> {
    writeln!(
        write,
        "//! This file was automatically generated by `iban_validate_registry_generation`.

pub struct RegistryExample<'a> {{
    pub country_code: &'a str,
    pub bank_identifier: Option<&'a str>,
    pub branch_identifier: Option<&'a str>,
    pub bban: &'a str,
    pub iban_electronic: &'a str,
    pub iban_print: &'a str,
}}

pub const EXAMPLES: &[RegistryExample] = &{:#?};",
        contents
            .records
            .iter()
            .map(|record| RegistryExample {
                country_code: record.country_code,
                bank_identifier: record.bank_identifier_example,
                branch_identifier: record.branch_identifier_example,
                bban: record.bban,
                iban_electronic: record.iban_electronic,
                iban_print: record.iban_print,
            })
            .collect::<Vec<_>>()
            .as_slice()
    )?;
    Ok(())
}
