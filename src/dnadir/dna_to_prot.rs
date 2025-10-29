use phf::phf_map;
use super::dna::Sequence;

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

impl Aminoacid {
    pub fn to_one_letter(&self) -> char {
        match self {
            Aminoacid::Ala => 'A',
            Aminoacid::Arg => 'R',
            Aminoacid::Asn => 'N',
            Aminoacid::Asp => 'D',
            Aminoacid::Cys => 'C',
            Aminoacid::Gln => 'Q',
            Aminoacid::Glu => 'E',
            Aminoacid::Gly => 'G',
            Aminoacid::His => 'H',
            Aminoacid::Ile => 'I',
            Aminoacid::Leu => 'L',
            Aminoacid::Lys => 'K',
            Aminoacid::Met => 'M',
            Aminoacid::Phe => 'F',
            Aminoacid::Pro => 'P',
            Aminoacid::Ser => 'S',
            Aminoacid::Thr => 'T',
            Aminoacid::Trp => 'W',
            Aminoacid::Tyr => 'Y',
            Aminoacid::Val => 'V',
            Aminoacid::Stop => '*',
        }
    }
}


impl Aminoacid {
    pub fn from_one_letter(letter: char) -> Option<Self> {
        match letter {
            'A' => Some(Aminoacid::Ala),
            'R' => Some(Aminoacid::Arg),
            'N' => Some(Aminoacid::Asn),
            'D' => Some(Aminoacid::Asp),
            'C' => Some(Aminoacid::Cys),
            'Q' => Some(Aminoacid::Gln),
            'E' => Some(Aminoacid::Glu),
            'G' => Some(Aminoacid::Gly),
            'H' => Some(Aminoacid::His),
            'I' => Some(Aminoacid::Ile),
            'L' => Some(Aminoacid::Leu),
            'K' => Some(Aminoacid::Lys),
            'M' => Some(Aminoacid::Met),
            'F' => Some(Aminoacid::Phe),
            'P' => Some(Aminoacid::Pro),
            'S' => Some(Aminoacid::Ser),
            'T' => Some(Aminoacid::Thr),
            'W' => Some(Aminoacid::Trp),
            'Y' => Some(Aminoacid::Tyr),
            'V' => Some(Aminoacid::Val),
            '*' => Some(Aminoacid::Stop),
            _ => None, 
        }
    }
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

    // pub fn fromone(&self, protein: &str) -> Vec<Aminoacid>{
    //     from_one_to_three(protein)
    // }
    
    
}
