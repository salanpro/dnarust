use biorust::dnadir::dna::Sequence;
use biorust::io::fasta;

fn main() -> std::io::Result<()> {
    let seq = Sequence::new("ATGGCCTAGCGATAA");
    let seq2 = Sequence::new("GGATGCACGAGCA");

    let record1 = fasta::FastaRecord::new("ID_1".to_string(), seq.clone());
    let record2 = fasta::FastaRecord::new_wd("ID_2".to_string(), "Desc_2".to_string(), seq2.clone());

    fasta::write_record("/home/salan/Desktop/fastatest/test.fasta", &record1)?;
    fasta::write_record("/home/salan/Desktop/fastatest/test2.fasta", &record2)?;
    fasta::write_records("/home/salan/Desktop/fastatest/testd.fasta", &[record1, record2],
    )?;

    Ok(())
}
