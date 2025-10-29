use crate::dnadir::dna::Sequence;
use crate::aminoacidsdir::amino::Aminoacid;
use crate::aminoacidsdir::amino::CODON_TABLE;


impl Aminoacid {
    pub fn codons(codon: &str) -> Option<Self> {
        CODON_TABLE.get(codon).copied()
    }
}

pub fn polypeptide_chainf(seq: &str) -> Vec<Aminoacid> {
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


pub fn polypeptide_chains(seq: &str) -> String {
    polypeptide_chainf(seq)
        .iter()
        .map(|aa| aa.to_one_letter())
        .collect()
}

pub fn translatefstring(seq: &str) -> String {
    polypeptide_chainf(seq)
        .iter()
        .map(|aa| format!("{:?}", aa)) 
        .collect::<Vec<String>>()
        .join("-") 
}

pub fn from_one_to_three(protein: &str) -> Vec<Aminoacid>{
    protein.chars().filter_map(|c| Aminoacid::from_one_letter(c)).collect()
}




impl Sequence {
    pub fn translatef(&self) -> Vec<Aminoacid> {
        polypeptide_chainf(&self.mrna())
    }

    pub fn translates(&self) -> String{
        polypeptide_chains(&self.mrna())
    }
    pub fn translatefstring(&self) -> String {
        translatefstring(&self.mrna())
    }

        
    
}
