use biorust::proteindir::proteins;


fn main (){

    let promax = proteins::Protein::new("TYA");

    println!("{}", promax.len())

}
