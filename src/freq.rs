// Frequency analyzer

use itertools::Itertools;
use std::collections::HashMap;
use std::process::exit;
use std::io::{BufRead, StdinLock};

pub enum Sorted {
    No,
    Alpha,
    Freq,
}

// The frequency counters
fn word_freq<F>(lines: &mut StdinLock, sf: F) -> HashMap<String, usize>
where
    F: Fn(char) -> bool,
{
    let mut dictionary = HashMap::new();
    let mut line = String::new();

    while lines.read_line(&mut line).unwrap() > 0 {
        line.split(&sf)
            .filter(|s| !s.is_empty())
            .map(|s| s.to_string())
            .for_each(|s| *dictionary.entry(s).or_insert(0usize) += 1);
        line.clear();
    }

    dictionary
}

fn word_freq_nums(s: &mut StdinLock) -> HashMap<String, usize> {
    word_freq(s, |c: char| !c.is_alphanumeric())
}

fn word_freq_no_nums(s: &mut StdinLock) -> HashMap<String, usize> {
    word_freq(s, |c: char| !c.is_alphabetic())
}

// Preparing for output
fn sort_by_freq(c: HashMap<String, usize>) -> () {
    let counts = c
        .iter()
        .sorted_by(|a, b| Ord::cmp(&b.1, &a.1));

    for (k, v) in counts {
        if let Err(_) = try_println!("{} {}", k, v) {
            exit(0);
        }
    }
}

fn sort_by_alpha(c: HashMap<String, usize>) -> () {
    let mut arr: Vec<String> = c.iter().map(|(k, v)| format!("{} {}", k, v)).collect();
    arr.sort();
    
    for i in arr {
        if let Err(_) = try_println!("{}", i) {
            exit(0);
        }
    }
}

fn no_sort(c: HashMap<String, usize>) -> () {
    for (k, v) in c {
        if let Err(_) = try_println!("{} {}", k, v) {
            exit(0);
        }
    }
}

// The main dispatch function
pub fn get_freqs(s: &mut StdinLock, nums: bool, sort: Sorted) -> () {
    let count = match nums {
        true => word_freq_nums(s),
        false => word_freq_no_nums(s),
    };
    match sort {
        Sorted::No => no_sort(count),
        Sorted::Alpha => sort_by_alpha(count),
        Sorted::Freq => sort_by_freq(count),
    }
}
