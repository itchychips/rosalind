#![allow(dead_code)]

use std::collections::HashMap;

use log::{trace};

pub fn count_nucleotides_hashmap(input: &String) -> HashMap<char, i64> {
    let mut output = HashMap::new();

    for ch in input.chars() {
        let count = output.get(&ch).unwrap_or(&0);
        let count = count + 1;
        output.insert(ch, count);
    }
    output
}

pub struct NucleotideCounts {
    pub a: i64,
    pub c: i64,
    pub g: i64,
    pub t: i64,
}

impl NucleotideCounts {
    pub fn new() -> NucleotideCounts {
        NucleotideCounts {
            a: 0,
            c: 0,
            g: 0,
            t: 0,
        }
    }
}

pub fn count_nucleotides(input: &String) -> NucleotideCounts {
    let mut output = NucleotideCounts::new();

    for ch in input.chars() {
        match ch {
            'A' => output.a += 1,
            'C' => output.c += 1,
            'G' => output.g += 1,
            'T' => output.t += 1,
            unknown => {
                trace!("Skipping character: '{}'", unknown);
            },
        }
    }
    output
}
