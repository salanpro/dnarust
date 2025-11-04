use phf::phf_map;



#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Aminoacid {
    Ala, Arg, Asn, Asp, Cys, Gln, Glu, Gly, His,
    Ile, Leu, Lys, Met, Phe, Pro, Ser, Thr, Trp,
    Tyr, Val, Stop,
}
pub static CODON_TABLE: phf::Map<&'static str, Aminoacid> = phf_map! {
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


