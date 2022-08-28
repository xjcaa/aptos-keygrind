use std::collections::HashSet;
use std::sync::atomic::{AtomicU64, Ordering};

pub struct PatternMatch {
    pub prefix: String,
    pub suffix: String,
    pub count: AtomicU64,
}

impl PatternMatch {
    pub fn matches(&self, address: &String) -> bool {
        let prefix_match = self.prefix.is_empty() || address.starts_with(&self.prefix);
        let suffix_match = self.suffix.is_empty() || address.ends_with(&self.suffix);
        prefix_match && suffix_match
    }
}

pub fn validate_pattern(v: &str) -> Result<(), String> {
    if v.matches(':').count() != 1 || (v.starts_with(':') || v.ends_with(':')) {
        return Err(String::from("Expected : between PATTERN and COUNT"));
    }
    let args: Vec<&str> = v.split(':').collect();
    validate_hex_pattern(args[0])?;
    let count = args[1].parse::<u64>();
    if count.is_err() || count.unwrap() == 0 {
        return Err(String::from("Expected COUNT to be of type u64"));
    }
    Ok(())
}

fn validate_hex_pattern(pattern: &str) -> Result<(), String> {
    let mut p = String::from(pattern.clone());
    if p.len() % 2 != 0 {
        p = [p, String::from("0")].join("");
    }
    hex::decode(&p).map_err(|err| format!("{}: {:?}", pattern, err))?;
    Ok(())
}

pub fn validate_pattern_prefix_and_suffix(v: &str) -> Result<(), String> {
    if v.matches(':').count() != 2 || (v.starts_with(':') || v.ends_with(':')) {
        return Err(String::from(
            "Expected : between PREFIX and SUFFIX and COUNT",
        ));
    }
    let args: Vec<&str> = v.split(':').collect();
    validate_hex_pattern(args[0])?;
    validate_hex_pattern(args[1])?;
    let count = args[2].parse::<u64>();
    if count.is_err() || count.unwrap() == 0 {
        return Err(String::from("Expected COUNT to be a u64"));
    }
    Ok(())
}

pub fn parse_pattern_args(
    prefix_args: HashSet<String>,
    suffix_args: HashSet<String>,
    prefix_and_suffix_args: HashSet<String>,
) -> Vec<PatternMatch> {
    let mut pattern_matches = Vec::<PatternMatch>::new();
    for prefix in prefix_args {
        let args: Vec<&str> = prefix.split(':').collect();
        pattern_matches.push(PatternMatch {
            prefix: args[0].to_string().to_lowercase(),
            suffix: "".to_string(),
            count: AtomicU64::new(args[1].parse::<u64>().unwrap()),
        });
    }
    for suffix in suffix_args {
        let args: Vec<&str> = suffix.split(':').collect();
        pattern_matches.push(PatternMatch {
            prefix: "".to_string(),
            suffix: args[0].to_string().to_lowercase(),
            count: AtomicU64::new(args[1].parse::<u64>().unwrap()),
        });
    }
    for prefix_suffix in prefix_and_suffix_args {
        let args: Vec<&str> = prefix_suffix.split(':').collect();
        pattern_matches.push(PatternMatch {
            prefix: args[0].to_string().to_lowercase(),
            suffix: args[1].to_string().to_lowercase(),
            count: AtomicU64::new(args[2].parse::<u64>().unwrap()),
        });
    }
    pattern_matches
}

pub fn print_pattern_matches(pattern_matches: &[PatternMatch], num_threads: usize) {
    println!("Searching with {} threads for:", num_threads);
    for pm in pattern_matches {
        let mut msg = Vec::<String>::new();
        if pm.count.load(Ordering::Relaxed) > 1 {
            msg.push("addresses".to_string());
            msg.push("start".to_string());
            msg.push("end".to_string());
        } else {
            msg.push("address".to_string());
            msg.push("starts".to_string());
            msg.push("ends".to_string());
        }
        println!(
            "\t{} {} that {} with '{}' and {} with '{}'",
            pm.count.load(Ordering::Relaxed),
            msg[0],
            msg[1],
            pm.prefix,
            msg[2],
            pm.suffix,
        );
    }
}
