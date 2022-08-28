use clap::ArgMatches;
use std::collections::HashSet;
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::Arc;
use std::thread;
use std::time::Instant;

use crate::account;
use crate::pattern;

pub fn run(matches: &ArgMatches) -> Result<(), String> {
    let suffix_args = if matches.is_present("suffix") {
        matches
            .values_of_t_or_exit::<String>("suffix")
            .into_iter()
            .map(|s| s)
            .collect()
    } else {
        HashSet::new()
    };

    let prefix_args = if matches.is_present("prefix") {
        matches
            .values_of_t_or_exit::<String>("prefix")
            .into_iter()
            .map(|s| s)
            .collect()
    } else {
        HashSet::new()
    };

    let prefix_and_suffix_args = if matches.is_present("prefix-suffix") {
        matches
            .values_of_t_or_exit::<String>("prefix-suffix")
            .into_iter()
            .map(|s| s)
            .collect()
    } else {
        HashSet::new()
    };

    let output_directory = String::from(matches.value_of("output-dir").unwrap());

    let pattern_matches =
        pattern::parse_pattern_args(prefix_args, suffix_args, prefix_and_suffix_args);
    let num_threads = num_cpus::get();

    pattern::print_pattern_matches(&pattern_matches, num_threads);

    let pattern_matches_thread_safe = Arc::new(pattern_matches);
    let attempts = Arc::new(AtomicU64::new(1));
    let found = Arc::new(AtomicU64::new(0));
    let start = Instant::now();
    let done = Arc::new(AtomicBool::new(false));

    let thread_handles: Vec<_> = (0..num_threads)
        .map(|_| {
            let done = done.clone();
            let attempts = attempts.clone();
            let found = found.clone();
            let pattern_matches_thread_safe = pattern_matches_thread_safe.clone();
            let output_directory = output_directory.clone();

            thread::spawn(move || loop {
                if done.load(Ordering::Relaxed) {
                    break;
                }
                let attempts = attempts.fetch_add(1, Ordering::Relaxed);
                if attempts % 1_000_000 == 0 {
                    println!(
                        "Searched {} keypairs in {}s. {} matches found.",
                        attempts,
                        start.elapsed().as_secs(),
                        found.load(Ordering::Relaxed),
                    );
                }
                let acc = account::Account::generate();
                let mut total_matches_found = 0;
                for i in 0..pattern_matches_thread_safe.len() {
                    if pattern_matches_thread_safe[i].count.load(Ordering::Relaxed) == 0 {
                        total_matches_found += 1;
                        continue;
                    }

                    if pattern_matches_thread_safe[i].matches(&acc.address) {
                        let _found = found.fetch_add(1, Ordering::Relaxed);
                        pattern_matches_thread_safe[i]
                            .count
                            .fetch_sub(1, Ordering::Relaxed);
                        acc.write_key(Some(output_directory.clone()));
                    }
                }
                if total_matches_found == pattern_matches_thread_safe.len() {
                    done.store(true, Ordering::Relaxed);
                }
            })
        })
        .collect();

    for thread_handle in thread_handles {
        thread_handle.join().unwrap();
    }

    Ok(())
}
