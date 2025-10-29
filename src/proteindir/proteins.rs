pub struct Protein{
    pub seq: String
}

impl Protein {
    pub fn new(seq: impl Into<String>) -> Self {
        Self { seq: seq.into() }
    }

    pub fn len(&self) -> usize {
        self.seq.len()
    }

    pub fn is_empty(&self) -> bool{
        self.seq.is_empty()

    }
}
