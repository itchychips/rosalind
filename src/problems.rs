use crate::nucleotides::Dna;
use crate::nucleotides::Rna;
use crate::nucleotides::Nucleotide;

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
    let dna = Dna::from(input);
    print!("{} ", dna.count_nucleotide(Nucleotide::Adenine));
    print!("{} ", dna.count_nucleotide(Nucleotide::Cytosine));
    print!("{} ", dna.count_nucleotide(Nucleotide::Guanine));
    print!("{}\n", dna.count_nucleotide(Nucleotide::Thymine));
}

pub fn transcribing_dna_into_rna(input: &String)
{
    let dna = Dna::from(input);
    let rna = Rna::from(dna);
    let string = String::from(rna);
    println!("{}", string);
}

pub fn complementing_a_strand_of_dna(input: &String)
{
    let dna = Dna::from(input);
    let reverse_complement = dna.reverse_complement();
    let string = String::from(reverse_complement);
    println!("{}", string);
}
