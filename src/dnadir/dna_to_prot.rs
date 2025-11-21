use crate::dnadir::dna::Sequence;
use crate::aminoacidsdir::amino::Aminoacid;
use crate::aminoacidsdir::amino::CODON_TABLE;


impl Aminoacid {
    pub fn codons(codon: &str) -> Option<Self> {
        CODON_TABLE.get(codon).copied()
    }
}

pub fn polypeptide_chain(seq: &str) -> Vec<Aminoacid> {
    let mut amino_chainf = Vec::with_capacity(seq.len());

    for codon in seq.as_bytes().chunks(3) {
    if codon.len() < 3 { break; }

    let codon_str = match std::str::from_utf8(codon) {
        Ok(x) => x,
        Err(_) => continue,
    };

    if let Some(aa) = Aminoacid::codons(codon_str) {
        if aa == Aminoacid::Stop { break; }
        amino_chainf.push(aa);
    }
}
    amino_chainf
}


pub fn polypeptide_chain_single(seq: &str) -> String {
    polypeptide_chain(seq)
        .iter()
        .map(|aa| aa.to_one_letter())
        .collect()
}

pub fn polypeptide_chain_three(seq: &str) -> String {
    polypeptide_chain(seq)
        .iter()
        .map(|aa| format!("{:?}", aa)) 
        .collect::<Vec<String>>()
        .join("-") 
}




impl Sequence {
    pub fn translate_as_vector(&self) -> Vec<Aminoacid> {
        polypeptide_chain(&self.to_rna())
    }
    pub fn translate_as_string(&self) -> String{
        polypeptide_chain_single(&self.to_rna())
    }
    pub fn translate_as_string3(&self) -> String {
        polypeptide_chain_three(&self.to_rna())
    }
    
}
