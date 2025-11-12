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
    pub fn new(seq: impl Into<String>) -> Self {
        Self { seq: seq.into() }
    }

    pub fn lenght(&self) -> usize {
        self.seq.len()
    }

    pub fn is_empty(&self) -> bool {
        self.seq.is_empty()
    }
    
    ///3' -> 5'
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
    ///5' -> 3'
    pub fn reverse_complement(&self) -> String {
        self.seq
            .chars()
            .rev()
            .map(|c| match c {
                'A' => 'T',
                'T' => 'A',
                'G' => 'C',
                'C' => 'G',
                _ => 'N',
            })
            .collect()
    }

    ///5' -> 3'
    pub fn mrna(&self) -> String {
        self.seq
            .chars()
            .map(|c| match c {
                'A' => 'A',
                'T' => 'U',
                'G' => 'G',
                'C' => 'C',
                _ => 'N',
            })
            .collect()
    }

    pub fn gc_content(&self) -> f32 {
        let gc = self.seq.chars().filter(|c| *c == 'G' || *c == 'C').count();
        gc as f32 / self.seq.len() as f32 * 100.
    }
}
