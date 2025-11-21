use biorust::dnadir::dna;

fn main() {
    let seq = dna::Sequence::new("ACGTATAC");
    // let seq = dna::Sequence::new("TCGTTCAGT");
    
    println!("Seq: {}", seq);
    println!("Lenght: {}", seq.lenght());
    println!("Complement: {}", seq.complement());
    println!("Reverse complement: {}", seq.reverse_complement());
    println!("Rna: {}", seq.to_rna());
    println!("Reverse Rna: {}", seq.to_reverse_rna());
    println!("GC %: {}", seq.gc_content());
    println!("Translation: {}", seq.translate_as_string());
    println!("{:?}", seq.translate_as_vector());
    println!("{}", seq.translate_as_string3());
}
