use biorust::proteindir::proteins;


fn main (){

    let protein = proteins::Protein::new("ACDEFGHIKLMNPQRSTVWY");

    println!("{}", protein.lenght());
    println!("{:?}", protein.from_string_to_vector());
    println!("{}", protein.protein_weight());

}
