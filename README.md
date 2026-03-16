# Dnarust
>I'm still thinking about the name of the lib
>
A personal project which consists in a basic library for genomics and bioinformatics

# Features
- [1. DNA](#1-dna)
- [2. Proteins](#2-proteins)
- [3. FASTA](#3-fasta)
- [4. File CODEC](#4-file-codec)

See [To-do list](#to-do-list) for current and futures features.

## 1. DNA
```rust
use biorust::dnadir::dna;

fn main() {
    let seq = dna::Sequence::new("ACGTATAC");
    
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
```
> #### Output
> ```bash
>Seq: ACGTATAC
>Lenght: 8
>Complement: TGCATATG
>Reverse complement: GTATACGT
>Rna: ACGUAUAC
>Reverse Rna: CAUAUGCA
>GC %: 37.5
>Translation: TY
>[Thr, Tyr]
>Thr-Tyr
> ```


## 2. Proteins
```rust
use biorust::proteindir::proteins;

fn main (){

    let protein = proteins::Protein::new("ACDEFGHIKLMNPQRSTVWY");

    println!("{}", protein.lenght());
    println!("{:?}", protein.from_string_to_vector());
    println!("{}", protein.protein_weight());

}
```
> #### Output
>```bash
>20
>[Ala, Cys, Asp, Glu, Phe, Gly, His, Ile, Lys, Leu, Met, Asn, Pro, Gln, Arg, Ser, Thr, Val, Trp, Tyr]
>2395.755
>```

## 3. FASTA
### FASTA READER
```rust
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
```
### FASTA WRITER
```rust
use biorust::dnadir::dna::Sequence;
use biorust::io::fasta;

fn main() -> std::io::Result<()> {
    let seq = Sequence::new("ATGGCCTAGCGATAA");
    let seq2 = Sequence::new("GGATGCACGAGCA");

    let record1 = fasta::FastaRecord::new("ID_1".to_string(), seq.clone());
    let record2 = fasta::FastaRecord::new_wd("ID_2".to_string(), "Desc_2".to_string(), seq2.clone());

    fasta::write_record("ouput1.fasta", &record1)?;
    fasta::write_record("ouput2.fasta", &record2)?;
    fasta::write_records("ouput3.fasta", &[record1, record2],
    )?;

    Ok(())
}
```
## 4. File CODEC
```rust
use biorust::io::files;


fn main()-> std::io::Result<()> {

    let input_encoder = "assets/images.jpg";
    let filename_encoder = "assets/prueba.dna";
    let _ = files::encoder(input_encoder, filename_encoder);

    let input_decoder = "assets/prueba.dna";
    let filename_decoder = "assets/recovered.jpg";
    let _ = files::decoder(input_decoder, filename_decoder);




    Ok(())
}
```

 # To do list
 #### DNA
- [x] The length of a DNA chain
- [x] Convert DNA squence to its complement
- [x] Convert DNA squence to its reverse complement
- [x] Convert DNA to mRNA
- [x] Calculate GC content
- [x] Translate DNA to polypeptide chain with **full amino acid names** (e.g., Ala, Arg, Asn)
- [x] Translate DNA to polypeptide chain with **single-letter amino acid codes** (e.g., A, R, N)
- [x] Convert between single-letter and three-letter amino acid codes
#### Proteins
- [x] Calculate molecular weight of the protein
- [ ] Isoelectric point (pI)
- [ ] Find synonymous codons for each amino acid
- [ ] Generate silent mutations (synonymous codon substitutions)

#### Primers designing
- [ ] Optimal melting temperature (Tm)
- [ ] The Gibbs free energy change (Δ*G*)
- [ ] Avoid long repeats
- [ ] 40 < GC% < 60

#### FASTA
- [x] Reading FASTA files
- [x] Writing FASTA files

# Documentation
See [Documentation](./DOCUMENTATION.md)
