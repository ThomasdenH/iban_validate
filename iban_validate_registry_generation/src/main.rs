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
    bytes::complete::{tag, take},
    character::complete::{alpha1, digit1},
    multi::many1,
    sequence::{preceded, separated_pair},
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
    for (country_code, maybe_range) in country_codes.iter().zip(bank_identifier_position.iter()) {
        if let Some((start, end)) = maybe_range {
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
    let bank_identifier_position: Vec<_> = contents
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
    for (country_code, maybe_range) in country_codes.iter().zip(bank_identifier_position.iter()) {
        if let Some((start, end)) = maybe_range {
            writeln!(&mut s, "\"{}\" => Some({}..{}),", country_code, start, end)?;
        } else {
            writeln!(&mut s, "\"{}\" => None,", country_code)?;
        }
    }
    Ok(s)
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
                many1(separated_pair(digit1, tag("!"), alpha1)),
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
            format!("Some(\"{}\")", bank[i])
        };
        let branch = if branch[i].is_empty() {
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
