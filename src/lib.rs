use wasm_bindgen::prelude::wasm_bindgen;
use web_sys::console::log_1 as log;
use base64::{ encode, decode };
use image::load_from_memory;
use std::io::Cursor;
use image::ImageOutputFormat::Png;

#[wasm_bindgen]
pub fn grayscale(encoded_file: &str) -> String {
    log(&"GrayScale Called".into());
    
    let base64_to_vector = decode(encoded_file).unwrap();
    
    log(&"Image Decoded".into());
    
    let mut img = load_from_memory(&base64_to_vector).unwrap();
    
    log(&"Image Loaded".into());
    
    img = img.grayscale();

    log(&"GrayScale Effect Applied".into());

    let mut buffer = vec![];
    img.write_to(&mut Cursor::new(&mut buffer), Png).unwrap();

    log(&"GrayScaled Image Written".into());

    let encoded_img = encode(&buffer);
    let data_url = format!("data:image/png;base64,{}", encoded_img);

    data_url
}