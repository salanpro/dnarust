use std::fs;
use std::io;
use std::path::Path;

    //ENCODER
pub fn encoder<P:AsRef<Path>>(path: P, destination: P) -> io::Result<()>{


    let file:Vec<u8> = fs::read(path)?;

    let mut seq = String::new();
    let nucleotides = ['A', 'T', 'G', 'C'];

    //                  A   T   G   C
    //                  ↑   ↑   ↑   ↑
    //                  0   1   2   3
    //                 00  01  10  11


    for &c in file.iter(){
        let pair1 = (c >> 6) & 0b11;
        let pair2 = (c >> 4) & 0b11;
        let pair3 = (c >> 2) & 0b11;
        let pair4 = c & 0b11;
        
        seq.push(nucleotides[pair1 as usize]);
        seq.push(nucleotides[pair2 as usize]);
        seq.push(nucleotides[pair3 as usize]);
        seq.push(nucleotides[pair4 as usize]);


        }


    let _ = fs::write(destination, seq);
    println!("DNA encoded");

    Ok(())

}

    // DECODER 
pub fn decoder<P:AsRef<Path>>(path: P, destination: P) -> io::Result<()>{

    let dna_text = fs::read_to_string(path)?;
    let mut decoded_bytes = Vec::new();
    
    let chars: Vec<char> = dna_text.chars().collect();
    
    for chunk in chars.chunks(4) {
        let mut byte: u8 = 0;
        
        for (i, &nucleotide) in chunk.iter().enumerate() {
            let value = match nucleotide {
                'A' => 0,
                'T' => 1,
                'G' => 2,
                'C' => 3,
                '\n' | '\r' | ' ' | '\t' => continue,
                _ => panic!("Unknown nucleotide: {}", nucleotide),
            };
            
            let shift = 6 - (i * 2);  
            byte |= value << shift;
        }
        
        decoded_bytes.push(byte);
    }
    
    fs::write(destination, &decoded_bytes)?;
    
    println!("DNA decoded");
    

    Ok(())
}

// pub fn compare(){
//     if encoded == decoded {
//         println!("Same file");
//     } else {
//         println!("Different file");
//     }
//
// }


