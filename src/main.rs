pub mod account;
pub mod grind;
pub mod pattern;

use clap::{App, Arg};

fn main() -> Result<(), String> {
    let matches = App::new("Aptos Keygrind")
        .about("CLI tool to grind addresses for Aptos")
        .arg_required_else_help(true)
        .arg(Arg::with_name("suffix")
            .short('s')
            .long("suffix")
            .value_name("PATTERN:COUNT")
            .number_of_values(1)
            .multiple_occurrences(true)
            .multiple_values(true)
            .validator(pattern::validate_pattern)
            .help("Saves the specified number of private keys where the address starts with the pattern\nExample: --suffix AAA:1\nPATTERN type is hex string, case insensitive\nCOUNT type is u64"))
        .arg(Arg::with_name("prefix")
            .short('p')
            .long("prefix")
            .value_name("PATTERN:COUNT")
            .number_of_values(1)
            .multiple_occurrences(true)
            .multiple_values(true)
            .validator(pattern::validate_pattern)
            .help("Saves the specified number of private keys where the address ends with the pattern\nExample: --prefix AAA:1\nPATTERN type is hex string, case insensitive\nCOUNT type is u64"))
        .arg(Arg::with_name("prefix-suffix")
            .short('b')
            .long("prefix-suffix")
            .value_name("PREFIX:SUFFIX:COUNT")
            .number_of_values(1)
            .multiple_occurrences(true)
            .multiple_values(true)
            .validator(pattern::validate_pattern_prefix_and_suffix)
            .help("Saves the specified number of private keys where the address starts with the prefix and ends with the suffix\nExample: --prefix-suffix AAA:BBB:1\nPATTERN type is hex string, case insensitive\nCOUNT type is u64"))
        .arg(Arg::with_name("output-dir")
            .short('o')
            .long("output-dir")
            .value_name("PATH")
            .number_of_values(1)
            .default_value("./")
            .help("Directory to save generated keys"))
        .get_matches();

    grind::run(&matches)?;

    Ok(())
}
