use crate::aminoacidsdir::amino::Aminoacid;
use crate::proteindir::proteins::Protein;

pub fn molecular_weight(amino: Aminoacid) -> f64 {
    match amino {
        Aminoacid::Ala => 89.09,
        Aminoacid::Arg => 174.20,
        Aminoacid::Asn => 132.12,
        Aminoacid::Asp => 133.10,
        Aminoacid::Cys => 121.16,
        Aminoacid::Gln => 146.15,
        Aminoacid::Glu => 147.13,
        Aminoacid::Gly => 75.07,
        Aminoacid::His => 155.16,
        Aminoacid::Ile => 131.18,
        Aminoacid::Leu => 131.18,
        Aminoacid::Lys => 146.19,
        Aminoacid::Met => 149.21,
        Aminoacid::Phe => 165.19,
        Aminoacid::Pro => 115.13,
        Aminoacid::Ser => 105.09,
        Aminoacid::Thr => 119.12,
        Aminoacid::Trp => 204.23,
        Aminoacid::Tyr => 181.19,
        Aminoacid::Val => 117.15,
        Aminoacid::Stop => 0.0,
    }
}

impl Protein {


    pub fn weight_from_char(c: char) -> Option<f64> {
    Aminoacid::from_one_letter(c).map(|aa| molecular_weight(aa))
}
    pub fn protein_weight(&self) -> f64 {
        let fsum: f64 = self.from_string_to_vector()
            .iter()
            .map(|amino| molecular_weight(*amino))
            .sum();
        return fsum - (self.lenght() as f64 - 1.) * 18.015
    }

}