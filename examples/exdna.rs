use biorust::dnadir::dna;

fn main() {
    let seq = dna::Sequence::new("ACGTATAC");
    println!("{}", seq.len());
    println!("{}", seq.reverse_complement());
    println!("{}", seq.complement());
    println!("{}", seq.mrna());
    println!("{}", seq.gc_content());
    println!("{:?}", seq.translatef());
    println!("{:?}", seq.translates());
    println!("{:?}", seq.translatefstring());
}


