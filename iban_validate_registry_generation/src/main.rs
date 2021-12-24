//! This file can generate the repeticive country-specific code from the IBAN registry.
//!
//! The code could be better, but honestly, it's not worth it. The txt format is undocumented,
//! seems to be ill-structured. (There is the following pattern: "_\n", which I suspect may be
//! due to one item having an accidental newline and the formatter adding " to escape it.)
//!
//! Furthermore, the registry itself has errors. Albania for example, has a bank code position 1-3,
//! implying a length of 3. Instead, the length is 8 and this is also the length of the example.
//! Based on a [press release](https://www.bankofalbania.org/Press/Press_Releases/IBAN_International_Bank_Account_Number.html),
//! the length is taken for truth.

use std::{fmt::Write, fs::File, io::Read};

use nom::{
    bytes::complete::{tag, take, take_while},
    character::complete::{alpha1, digit1},
    multi::many1,
    sequence::{preceded, separated_pair, terminated},
    IResult,
};

fn main() -> anyhow::Result<()> {
    let mut registry = File::open("./iban_validate_registry_generation/swift_iban_registry.txt")?;
    let mut contents = String::new();
    registry.read_to_string(&mut contents)?;

    println!(
        "BANK_IDENTIFIER:\n{}",
        generate_bank_identifier_position_in_bban_match_arm(&contents)?
    );

    println!(
        "BRANCH_IDENTIFIER:\n{}",
        generate_branch_identifier_position_in_bban_match_arm(&contents)?
    );

    println!(
        "FORMAT_VALIDATION:\n{}",
        generate_format_match_arm(&contents)?
    );

    println!("TEST_FILE:\n{}", generate_test_file(&contents)?);

    Ok(())
}

fn generate_bank_identifier_position_in_bban_match_arm(contents: &str) -> anyhow::Result<String> {
    let mut s = String::new();
    let country_codes = read_line(contents, COUNTRY_CODE_INDEX);
    // - The bank identifier has an error in the input file: one range is terminated by an \n and
    //   surrounded by ". I believe this is an error (an accidental newline that is escaped by the
    //   generator of the file. Current solution: just fix the input file.
    // - The identifier for Albania is a bit strange. I think 1-8 is the bank identifier position,
    //   but then part of that is the branch position. Solution: the length is taken instead of
    //   the range as denoted in the file. See: https://www.bankofalbania.org/Press/Press_Releases/IBAN_International_Bank_Account_Number.html
    let bank_identifier_position: Vec<_> = contents
        .lines()
        .nth(10)
        .unwrap()
        .split('\t')
        .skip(1)
        .map(|s| {
            let mut split = s.split('-');
            if let Ok(start) = split.next().unwrap().parse::<usize>() {
                let end: usize = split.next().unwrap().parse().unwrap();
                Some((start - 1, end))
            } else {
                None
            }
        })
        .collect();
    let bank_identifier_pattern = read_line(contents, 11);
    for ((country_code, maybe_range), identifier_pattern) in country_codes
        .iter()
        .zip(bank_identifier_position.iter())
        .zip(bank_identifier_pattern.iter())
    {
        if let Some((start, mut end)) = maybe_range {
            // Test for inconsistencies in the input file and leave a comment.
            // Namely, decduce the pattern length and check if it matches the range.
            if *identifier_pattern != "N/A" && !identifier_pattern.is_empty() {
                let bank_identifier_length =
                    if let Ok((_, bank_identifier_length)) = parse_pattern(identifier_pattern) {
                        bank_identifier_length
                            .into_iter()
                            .map(|(len, _type)| len.parse::<usize>().unwrap())
                            .sum()
                    } else {
                        parse_malformed_pattern(identifier_pattern)
                            .unwrap()
                            .1
                            .into_iter()
                            .map(|len| len.parse::<usize>().unwrap())
                            .sum()
                    };

                if end - start != bank_identifier_length {
                    writeln!(&mut s, "// The bank identifier length ({}) does not match the range ({}..{}) in the registry. Using length as truth.", bank_identifier_length, start, end)?;
                    end = start + bank_identifier_length;
                }
            }
            writeln!(&mut s, "\"{}\" => Some({}..{}),", country_code, start, end)?;
        } else {
            writeln!(&mut s, "\"{}\" => None,", country_code)?;
        }
    }
    Ok(s)
}

const COUNTRY_CODE_INDEX: usize = 2;

fn read_line(contents: &str, index: usize) -> Vec<&str> {
    contents
        .lines()
        .nth(index)
        .unwrap()
        .split('\t')
        .skip(1)
        .collect()
}

fn generate_branch_identifier_position_in_bban_match_arm(contents: &str) -> anyhow::Result<String> {
    let mut s = String::new();
    let country_codes = read_line(contents, COUNTRY_CODE_INDEX);
    let branch_identifier_position: Vec<_> = contents
        .lines()
        .nth(12)
        .unwrap()
        .split('\t')
        .skip(1)
        .map(|s| {
            let mut split = s.split('-');
            if let Ok(start) = split.next().unwrap().parse::<usize>() {
                let end: usize = split.next().unwrap().parse().unwrap();
                Some((start - 1, end))
            } else {
                None
            }
        })
        .collect();

    // Load the examples
    let branch_identifier = read_line(contents, 15);


    for ((country_code, maybe_range), branch_identifier) in country_codes.iter().zip(branch_identifier_position.iter()).zip(branch_identifier.iter()) {
        if let Some((start, mut end)) = maybe_range {
            // Just do some sanity check. That actually fails sometimes...

            if branch_identifier.is_empty() {
                // This happens for Jordan. The correct thing to do seems to be
                // to assume that there just isn't an example.
                // Note that the .PDF version of the registry is incorrect.
                // The bank position should be 1-4 but is 5-8, the branch
                // position should be 5-8 but is empty.
                writeln!(&mut s, "// The registry doesn't provide an example.")?;
            } else if branch_identifier.len() != end - start {
                if branch_identifier.len() == (end - 1) - start {
                    // Assume that the end of the range is accidentally exclusive, unlike the other entries.
                    writeln!(&mut s, "// The registry branch example (\"{}\") does not have the length as expected from the position range ({}..{}).\n// Assume the example is correct, see generation code for details.", branch_identifier, start, end)?;
                    end -= 1;
                } else {
                    panic!("The registry branch example (\"{}\") does not have the length as expected from the position range ({}..{}) and it can't be fixed.", branch_identifier, start, end);
                }
            }

            writeln!(&mut s, "\"{}\" => Some({}..{}),", country_code, start, end)?;
        } else {
            writeln!(&mut s, "\"{}\" => None,", country_code)?;
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

fn generate_format_match_arm(contents: &str) -> anyhow::Result<String> {
    let mut s = String::new();
    let country_codes = read_line(contents, COUNTRY_CODE_INDEX);
    let bank_identifier_position: Vec<_> = contents
        .lines()
        .nth(18)
        .unwrap()
        .split('\t')
        .skip(1)
        .map(|s| -> IResult<&str, Vec<(&str, &str)>> {
            // Parse string.
            preceded(
                // Skip country code and check digits
                take(5_usize),
                parse_pattern,
            )(s)
        })
        .map(|res| {
            res.map(|(s, pars)| {
                assert_eq!(s, "");
                pars
            })
        })
        .collect::<Result<Vec<_>, _>>()
        .unwrap();
    for (country_code, pos) in country_codes.iter().zip(bank_identifier_position.iter()) {
        // TODO: Maybe combine sequences of the same character
        let pos_formatted = pos
            .iter()
            .map(|(num, t): &(&str, &str)| format!("({}, {})", num, t.to_ascii_uppercase()))
            .collect::<Vec<_>>()
            .join(", ");
        writeln!(
            &mut s,
            "\"{}\" => Some([{}].borrow()),",
            country_code, pos_formatted
        )?;
    }
    Ok(s)
}

fn generate_test_file(contents: &str) -> anyhow::Result<String> {
    let mut s = String::new();
    let country_codes = read_line(contents, COUNTRY_CODE_INDEX);
    let bank = read_line(contents, 14);
    let branch = read_line(contents, 15);
    let bbans = read_line(contents, 16);
    let iban_electronic = read_line(contents, 21);
    let iban_print = read_line(contents, 22);
    for i in 0..country_codes.len() {
        let bank = if bank[i].is_empty() {
            "None".to_string()
        } else {
            // For Albania, the bank string contains extra '-'. This is not deducible from the IBAN.
            // It would be an option to hardcode if the positions are fixed (are they?). Instead,
            // we'll remove them here and assume that is also correct.
            let bank: String = bank[i]
                .chars()
                .filter(|c| c.is_ascii_alphanumeric())
                .collect();
            format!("Some(\"{}\")", bank)
        };
        let branch = if branch[i].is_empty() || branch[i] == "N/A" {
            "None".to_string()
        } else {
            format!("Some(\"{}\")", branch[i])
        };
        writeln!(
            &mut s,
            "(\"{}\", {}, {}, \"{}\", \"{}\", \"{}\"),",
            country_codes[i], bank, branch, bbans[i], iban_electronic[i], iban_print[i]
        )?;
    }
    Ok(s)
}
