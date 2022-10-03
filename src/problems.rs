use crate::nucleotides;

pub fn counting_dna_nucleotides(input: &String)
{
    let nucleotide_counts = nucleotides::count_nucleotides(&input);
    print!("{} ", nucleotide_counts.a);
    print!("{} ", nucleotide_counts.c);
    print!("{} ", nucleotide_counts.g);
    print!("{}\n", nucleotide_counts.t);
}
