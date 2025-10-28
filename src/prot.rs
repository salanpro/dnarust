use phf::phf_map;
use crate::seq::Sequence;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Aminoacid {
    Ala, Arg, Asn, Asp, Cys, Gln, Glu, Gly, His,
    Ile, Leu, Lys, Met, Phe, Pro, Ser, Thr, Trp,
    Tyr, Val, Stop,
}

static CODON_TABLE: phf::Map<&'static str, Aminoacid> = phf_map! {
    "UUU" => Aminoacid::Phe, "UUC" => Aminoacid::Phe,
    "UUA" => Aminoacid::Leu, "UUG" => Aminoacid::Leu,
    "UCU" => Aminoacid::Ser, "UCC" => Aminoacid::Ser, "UCA" => Aminoacid::Ser, "UCG" => Aminoacid::Ser, 
    "CCU" => Aminoacid::Pro, "CCC" => Aminoacid::Pro, "CCA" => Aminoacid::Pro, "CCG" => Aminoacid::Pro,
    "ACU" => Aminoacid::Thr, "ACC" => Aminoacid::Thr, "ACA" => Aminoacid::Thr, "ACG" => Aminoacid::Thr,
    "GCU" => Aminoacid::Ala, "GCC" => Aminoacid::Ala, "GCA" => Aminoacid::Ala, "GCG" => Aminoacid::Ala,
    "UAU" => Aminoacid::Tyr, "UAC" => Aminoacid::Tyr,
    "CAU" => Aminoacid::His, "CAC" => Aminoacid::His,
    "CAA" => Aminoacid::Gln, "CAG" => Aminoacid::Gln,
    "AAU" => Aminoacid::Asn, "AAC" => Aminoacid::Asn,
    "AAA" => Aminoacid::Lys, "AAG" => Aminoacid::Lys,
    "GAU" => Aminoacid::Asp, "GAC" => Aminoacid::Asp,
    "GAA" => Aminoacid::Glu, "GAG" => Aminoacid::Glu,
    "UGU" => Aminoacid::Cys, "UGC" => Aminoacid::Cys,
    "UGG" => Aminoacid::Trp,
    "CGU" => Aminoacid::Arg, "CGC" => Aminoacid::Arg, "CGA" => Aminoacid::Arg, "CGG" => Aminoacid::Arg,
    "AGA" => Aminoacid::Arg, "AGG" => Aminoacid::Arg,
    "AGU" => Aminoacid::Ser, "AGC" => Aminoacid::Ser,
    "GGU" => Aminoacid::Gly, "GGC" => Aminoacid::Gly, "GGA" => Aminoacid::Gly, "GGG" => Aminoacid::Gly,
 
    "UAA" => Aminoacid::Stop, "UAG" => Aminoacid::Stop, "UGA" => Aminoacid::Stop,
};


impl Aminoacid {
    pub fn codons(codon: &str) -> Option<Self> {
        CODON_TABLE.get(codon).copied()
    }
}

pub fn polypeptide_chain(seq: &str) -> Vec<Aminoacid> {
    let mut amino_chain = Vec::with_capacity(seq.len());

    for codon in seq.as_bytes().chunks(3) {
    if codon.len() < 3 { break; }

    let codon_str = match std::str::from_utf8(codon) {
        Ok(x) => x,
        Err(_) => continue,
    };

    if let Some(aa) = Aminoacid::codons(codon_str) {
        if aa == Aminoacid::Stop { break; }
        amino_chain.push(aa);
    }
}
    amino_chain
}

impl Sequence {
    pub fn translate(&self) -> Vec<Aminoacid> {
        polypeptide_chain(&self.mrna())
    }
}

