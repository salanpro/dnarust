// src/dnadir/dna.rs

use std::fmt;

#[derive(Debug, Clone)]
pub struct Sequence {
    pub seq: String,
}

impl fmt::Display for Sequence {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.seq)
    }
}

impl Sequence {
    ///Create a new Sequence (Intended for using the Coding Strand)
    ///
    ///# Args
    ///The sequence can be `String` or `&str`, or any type that implements `Into<String>`
    ///
    ///# Examples
    ///
    ///```
    ///use biorust::dnadir::dna;
    ///
    ///let seq1 = dna::Sequence::new("ATGC");
    ///
    ///let seq2 = dna::Sequence::new(String::from("ATGC"));
    ///```
    ///
    pub fn new(seq: impl Into<String>) -> Self {
        Self { seq: seq.into() }
    }

    ///Returns the lenght of the DNA base sequence
    ///# Examples
    ///```
    ///use biorust::dnadir::dna;
    ///
    ///let seq = dna::Sequence::new("ATGC");
    ///assert_eq!(seq.lenght(), 4);
    ///
    ///let seq2 = dna::Sequence::new("ATGCATGC");
    ///assert_eq!(seq2.lenght(), 8);
    ///```
    ///
    pub fn lenght(&self) -> usize {
        self.seq.len()
    }

    pub fn is_empty(&self) -> bool {
        self.seq.is_empty()
    }

    ///(Intended to be the Template strand)
    ///
    ///Returns the complement of a sequence assuming the
    ///sequence has the direction 5' -> 3', then it returns
    ///the complement in the direction 3' -> 5'
    ///
    ///# Example
    ///```
    ///use biorust::dnadir::dna;
    ///
    ///let seq = dna::Sequence::new("ATGCATGC");
    ///assert_eq!(seq.complement(), "TACGTACG");
    ///```
    ///
    pub fn complement(&self) -> String {
        self.seq
            .chars()
            .map(|c| match c {
                'A' => 'T',
                'T' => 'A',
                'G' => 'C',
                'C' => 'G',
                _ => 'N',
            })
            .collect()
    }
    ///Returns the reverse complement of a sequence assuming the
    ///sequence has the direction 5' -> 3', then it returns
    ///the complement in the direction 5' -> 3'
    ///
    ///# Example
    ///```
    ///use biorust::dnadir::dna;
    ///
    ///let seq = dna::Sequence::new("ATGCATGC");
    ///assert_eq!(seq.reverse_complement(), "GCATGCAT");
    ///```
    ///
    pub fn reverse_complement(&self) -> String {
        self.complement().chars().rev().collect()
    }

    ///Returns the RNA sequence of a DNA sequence
    ///assuming the sequence has the direction 5' -> 3', then it returns
    ///the RNA sequence with the direction 5' -> 3'
    ///# Example
    ///```
    ///use biorust::dnadir::dna;
    ///
    ///let seq = dna::Sequence::new("TCGTTCAGT");
    ///assert_eq!(seq.to_rna(), "UCGUUCAGU");
    ///```
    pub fn to_rna(&self) -> String {
        self.seq.replace("T", "U")
    }

    

    ///Returns the reverse RNA sequence of a DNA sequence
    ///assuming the sequence has the direction 5' -> 3', then it returns
    ///the RNA sequence with the direction 3' -> 5'
    ///# Example
    ///```
    ///use biorust::dnadir::dna;
    ///
    ///let seq = dna::Sequence::new("TCGTTCAGT");
    ///assert_eq!(seq.to_reverse_rna(), "UGACUUGCU");
    ///```
    pub fn to_reverse_rna(&self) -> String{
        self.to_rna().chars().rev().collect()
    }

    ///Returns the percentage of Guanine `G` and Cystosine `C`
    ///bases in a DNA or RNA sequence
    ///# Example
    ///```
    ///use biorust::dnadir::dna;
    ///
    ///let seq = dna::Sequence::new("TCGTTCAGT");
    ///assert_eq!(seq.gc_content(), 44.44);
    ///```
    pub fn gc_content(&self) -> f32 {
        let gc = self.seq.chars().filter(|c| *c == 'G' || *c == 'C').count();
        let result = gc as f32 / self.seq.len() as f32 * 100.;
        (result*100.).round()/100.
    }
}
