use biorust::dnadir::dna;

fn main() {
    let seq = dna::Sequence::new("ACGTATAC");
    println!("{}", seq.lenght());
    println!("{}", seq.reverse_complement());
    println!("{}", seq.complement());
    println!("{}", seq.mrna());
    println!("{}", seq.gc_content());
    println!("{:?}", seq.translate_as_vector());
    println!("{:?}", seq.translate_as_string());
    println!("{:?}", seq.translate_as_string3());
    
}


