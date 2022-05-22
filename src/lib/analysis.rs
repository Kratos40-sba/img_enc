use std::collections::HashMap;
use rand::{Rng,rngs::ThreadRng};
use crate::algo::{decrypt_image, encrypt_image};
use crate::image_utils::{compose_image, decompose};

fn image_to_array(path : &str) -> Vec<u8> {
    image::open(path).unwrap()
        .to_luma8().to_vec()
}
fn histogram_vals(path : &str) -> Vec<i32> {
    let encrypted_image : Vec<u8> = image_to_array(path) ;
    let mut histo : HashMap<u8,i32> = HashMap::new() ;
    for i in 0..255 {
        histo.insert(i,0)
    }
    for i in 0..encrypted_image.len() {
        if histo.contains_key(&encrypted_image[i]) {
            *histo.get_mut(&encrypted_image[i]).unwrap() += 1
        }
    }
    let hist_values : Vec<i32> = histo.into_iter()
        .map(|(_k ,v)|v).collect();
    hist_values
}
fn chi_square_test(path : &str) -> f32 {
    let hist_values: Vec<i32> = histogram_vals(path);
    let mut result : f32 = 0.0 ;
    for i in 0..255 {
        result += f32::powi(hist_values[i] as f32 - 256.0, 2) / 256.0
    }
    result
}
fn entropy_test(path:&str) -> f32 {
    let hist_values: Vec<i32> = histogram_vals(path);
    let mut result : f32 = 0.0 ;
    let sum : i32 = hist_values.clone().into_iter().sum() ;
    for i in 0..255 {
        if hist_values[i] != 0 {
            result += (hist_values[i] as f32 / sum as f32) * f32::log2(sum as f32 / hist_values[i] as f32);
        }
    }
    result
}
pub fn uniformity_analysis(path : &str) -> f32 {
    let mut mean : f32 = 0.0 ;
    let mut arr_result : [f32;100] = [0f32;100] ;
    for i in 0..100 {
        let mut rnd : ThreadRng = rand::thread_rng() ;
        let _init_array : [f32;32] = rnd.gen() ;
        let mut dkv : [u8;64] = [0u8;64] ;
        rnd.fill(&mut dkv) ;
        compose_image(
            encrypt_image(
                decompose(path).0 ,
                dkv) , decompose(path).1 , decompose(path).2 , "encrypted");
        let chi_test : f32 = chi_square_test("encrypted.bmp");
        arr_result[i] = chi_test ;
        mean += arr_result[i]
    }
    mean/100.0
}
pub fn entropy_analysis(path : &str) -> f32 {
    let mut mean : f32 = 0.0 ;
    let mut arr_result : [f32;100] = [0f32;100] ;
    for i in 0..100 {
        let mut rnd = rand::thread_rng() ;
        let _init_array : [f32;32] = rnd.gen();
        let mut dkv : [u8;64] = [0u8;64] ;
        rnd.fill(&mut dkv) ;
        compose_image(
            encrypt_image(
                decompose(path).0 , dkv ) ,
            decompose(path).1 , decompose(path).2
                , "encrypted"
            )
        let entropy_test = entropy_test("encrypted.bmp");
        arr_result[i] = entropy_test ;
        mean += arr_result[i]
    }
    mean/100.0
}
// correlation
// choose N random pixels
// vertical x = m[i][j] / y = m[i][j-1] or m[i][j+1]
