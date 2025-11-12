use biorust::io::fasta;

fn main() -> std::io::Result<()> {

    let record1 = fasta::read_records("/home/salan/Desktop/fastatest/test.fasta")?;
    let record2 = fasta::read_records("/home/salan/Desktop/fastatest/test2.fasta")?;
    let records = fasta::read_records("/home/salan/Desktop/fastatest/testd.fasta")?;


    for record in &record2 {
        println!("ID: {}", record.id);

        if let Some(desc) = &record.description {
            println!("Description: {}", desc);
        }

        println!("{}", record.sequence);
    }

    Ok(())
}
