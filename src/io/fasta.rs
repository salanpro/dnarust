use crate::aminoacidsdir::amino;
use crate::dnadir::dna;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Write};
use std::path::Path;

#[derive(Debug, Clone)]
pub struct FastaRecord {
    pub id: String,
    pub description: Option<String>,
    pub sequence: dna::Sequence,
}

impl FastaRecord {
    pub fn new(id: String, sequence: dna::Sequence) -> Self {
        Self {
            id,
            description: None,
            sequence,
        }
    }

    pub fn new_wd(id: String, description: String, sequence: dna::Sequence) -> Self {
        Self {
            id,
            description: Some(description),
            sequence,
        }
    }

    pub fn lenght(&self) -> usize {
        dna::Sequence::lenght(&self.sequence)
    }

    pub fn is_empty(&self) -> bool {
        dna::Sequence::is_empty(&self.sequence)
    }

    pub fn complement(&self) -> String {
        dna::Sequence::complement(&self.sequence)
    }

    pub fn reverse_complement(&self) -> String {
        dna::Sequence::reverse_complement(&self.sequence)
    }

    pub fn mrna(&self) -> String {
        dna::Sequence::to_rna(&self.sequence)
    }

    pub fn gc_content(&self) -> f32 {
        dna::Sequence::gc_content(&self.sequence)
    }

    pub fn translate_as_vector(&self) -> Vec<amino::Aminoacid> {
        dna::Sequence::translate_as_vector(&self.sequence)
    }

    pub fn translate_as_string(&self) -> String {
        dna::Sequence::translate_as_string(&self.sequence)
    }

    pub fn translate_as_string3(&self) -> String {
        dna::Sequence::translate_as_string3(&self.sequence)
    }

    pub fn header(&self) -> String {
        if let Some(desc) = &self.description {
            format!(">{} {}", self.id, desc)
        } else {
            format!(">{}", self.id)
        }
    }

    pub fn seq(&self) -> &str {
        &self.sequence.seq
    }
}


pub fn read_records<P: AsRef<Path>>(path: P) -> io::Result<Vec<FastaRecord>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let mut records = Vec::new();
    let mut current_id = String::new();
    let mut current_desc: Option<String> = None;
    let mut current_seq = String::new();
    
    for line in reader.lines() {
        let line = line?;
        let line = line.trim();
        
        if line.is_empty() {
            continue;
        }
        
        if line.starts_with('>') {
            if !current_id.is_empty() {
                records.push(FastaRecord {
                    id: current_id.clone(),
                    description: current_desc.clone(),
                    sequence: dna::Sequence::new(current_seq.clone()),
                });
                current_seq.clear();
            }
            
            let header = &line[1..];
            let mut parts = header.splitn(2, ' ');
            current_id = parts.next().unwrap_or("").to_string();
            current_desc = parts.next().map(|s| s.to_string());
        } else {
            current_seq.push_str(line);
        }
    }
    
    if !current_id.is_empty() {
        records.push(FastaRecord {
            id: current_id,
            description: current_desc,
            sequence: dna::Sequence::new(current_seq),
        });
    }
    
    Ok(records)
}


pub fn write_records<P: AsRef<Path>>(path: P, records: &[FastaRecord]) -> io::Result<()> {
    write_records_with_width(path, records, 80)
}

pub fn write_records_with_width<P: AsRef<Path>>(
    path: P,
    records: &[FastaRecord],
    line_width: usize,
) -> io::Result<()> {
    let mut file = File::create(path)?;

    for record in records {
        writeln!(file, "{}", record.header())?;

        let seq = &record.sequence.seq;
        for chunk in seq.as_bytes().chunks(line_width) {
            writeln!(file, "{}", std::str::from_utf8(chunk).unwrap())?;
        }
    }

    Ok(())
}

pub fn write_record<P: AsRef<Path>>(path: P, record: &FastaRecord) -> io::Result<()> {
    write_records(path, &[record.clone()])
}
