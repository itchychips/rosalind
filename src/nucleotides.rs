#![allow(dead_code)]

use std::collections::HashMap;

use log::trace;

/// Count nucleotides using a hashmap.
///
/// # Arguments
///
/// * `input` - A string of an input nucleotides.
///
/// # Remarks
///
/// The implementation is quite naive.  While nucleotides for DNA are A, C, G, and T (and RNA
/// replaces T with U), the hashmap will record the count of any character.
pub fn count_nucleotides_hashmap(input: &String) -> HashMap<char, i64> {
    let mut output = HashMap::new();

    for ch in input.chars() {
        let count = output.get(&ch).unwrap_or(&0);
        let count = count + 1;
        output.insert(ch, count);
    }
    output
}

/// Data for nucleotide counts.
///
/// # Remarks
///
/// Each count uses an i64.  Be wary if you need to track larger counts.
///
/// Additionally, this is DNA-specific.  You could just use T for U for RNA, but it's hacky either
/// way.
pub struct NucleotideCounts {
    /// Adenine count.
    pub a: i64,
    /// Cytosine count.
    pub c: i64,
    /// Guanine count.
    pub g: i64,
    /// Thymine count.
    pub t: i64,
}

impl NucleotideCounts {
    /// Create new nucleotide counts data.  All counts start at 0.
    pub fn new() -> NucleotideCounts {
        NucleotideCounts {
            a: 0,
            c: 0,
            g: 0,
            t: 0,
        }
    }
}

/// Count nucleotides
///
/// # Arguments
///
/// * `input` - A string of input nucleotides.
///
/// # Remarks
///
/// The implementation will throw away any characters not recognized, most namely for uracil (U).
/// Set logger to level of trace for output of any skipped character with codes.
pub fn count_nucleotides(input: &String) -> NucleotideCounts {
    let mut output = NucleotideCounts::new();

    for ch in input.chars() {
        match ch {
            'A' => output.a += 1,
            'C' => output.c += 1,
            'G' => output.g += 1,
            'T' => output.t += 1,
            unknown => {
                trace!("Skipping character with code: '{}' (code: {})", unknown.escape_debug(), unknown.escape_unicode());
            },
        }
    }
    output
}
