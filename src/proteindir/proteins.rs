use crate::aminoacidsdir::{amino::Aminoacid};

pub struct Protein{
    pub seq: String
}


impl Protein {
    pub fn new(seq: impl Into<String>) -> Self {
        Self { seq: seq.into() }
    }

    pub fn lenght(&self) -> usize {
        self.seq.len()
    }

    pub fn is_empty(&self) -> bool{
        self.seq.is_empty()
    
    }

    pub fn from_string_to_vector(&self) -> Vec<Aminoacid>{
    self.seq.chars().filter_map(|c| Aminoacid::from_one_letter(c)).collect()
}

    }


