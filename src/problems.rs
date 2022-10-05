use crate::nucleotides;

/// Solve problem counting_dna_nucleotides.
///
/// # Arguments
///
/// * `input` - A string that should denote DNA nucleotides (A, C, G, and T).
///
/// # Remarks
///
/// This only prints the solution to output.  `Use nucleotides::count_nucleotides(&String)` if you
/// want to use the solution generation directly.
pub fn counting_dna_nucleotides(input: &String)
{
    let nucleotide_counts = nucleotides::count_nucleotides(&input);
    print!("{} ", nucleotide_counts.a);
    print!("{} ", nucleotide_counts.c);
    print!("{} ", nucleotide_counts.g);
    print!("{}\n", nucleotide_counts.t);
}
