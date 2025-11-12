use biorust::io::fasta;

fn main() -> std::io::Result<()> {

    let record1 = fasta::read_records("PATH TO FASTA FILE")?;
    let record2 = fasta::read_records("PATH TO FASTA FILE")?;
    let records = fasta::read_records("PATH TO FASTA FILE")?;


    for record in &record2 {
        println!("ID: {}", record.id);

        if let Some(desc) = &record.description {
            println!("Description: {}", desc);
        }

        println!("{}", record.sequence);
    }

    Ok(())
}
