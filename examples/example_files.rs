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
