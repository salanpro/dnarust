# Dnarust
>I'm still thinking about the name of the lib
>
A personal project which consists in a basic library for genomics and bioinformatics

# Features
- [1. DNA](#1-dna)
- [2. Proteins](#2-proteins)

See [To-do list](#to-do-list) for current and futures features.

## 1. DNA
```rust
let seq = dna::Sequence::new("ACGTATAC");
    println!("{}", seq.lenght()); //Lenght of the DNA chain
    println!("{}", seq.reverse_complement()); //Reverse complement
    println!("{}", seq.complement()); //Complement
    println!("{}", seq.mrna()); //Transcription to mRNA
    println!("{}", seq.gc_content()); //Content of G & C
    println!("{:?}", seq.translate_as_vector()); //Translate the DNA as a vector of aminoacids using 3-letter-code
    println!("{}", seq.translate_as_string()); //Translate the DNA as a string of aminoacids using 1-letter-code
    println!("{}", seq.translate_as_string3()); //Translate the DNA as a string of aminoacids using 3-letter-code
```
> #### Output
> ```bash
> 8
>GTATACGT
>TGCATATG
>ACGUAUAC
>37.5
>[Thr, Tyr]
>TY
>Thr-Tyr
> ```


## 2. Proteins
```rust
let protein = proteins::Protein::new("ACDEFGHIKLMNPQRSTVWY");
    println!("{}", protein.lenght()); //Lenght of the polypeptide chain
    println!("{:?}", protein.from_string_to_vector()); //Convert the 1-letter-code polypeptide chain to a 3-letter-code vector
    println!("{}", protein.protein_weight()); //Calculate the weight of the protein/polypeptide chain in Daltons
```
> #### Output
>```bash
>20
>[Ala, Cys, Asp, Glu, Phe, Gly, His, Ile, Lys, Leu, Met, Asn, Pro, Gln, Arg, Ser, Thr, Val, Trp, Tyr]
>2395.7550000000006
>```


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
- [ ] Reading FASTA files
- [ ] Writing FASTA files

# Documentation
See [Documentation](./DOCUMENTATION.md)
