use std::collections::HashMap;
use std::fmt::format;
use image::{GrayImage, ImageFormat};
use rand::{Rng,rngs::ThreadRng};
use crate::algo::encrypt_image;
use crate::image_utils::{compose_image, decompose};
use crate::init::dkv;

fn image_to_array(path : &str) -> Vec<u8> {
    image::open(path).unwrap()
        .to_luma8().to_vec()
}
fn image_to_matrix(path : &str) -> [[u8;256];256] {
    let mut matrix = [[0u8;256];256] ;
    let image = image::open(
     format!("{}.bmp",path).as_mut_str()
    ).unwrap().to_luma8() ;
    let (w,h) = image.dimensions() ;
    for i in 0..w {
        for j in 0..h {
            matrix[i as usize][j as usize] = image.get_pixel(i,j).0[0]
        }}
    matrix
}
fn histogram_vals(path : &str) -> Vec<i32> {
    let encrypted_image : Vec<u8> = image_to_array(path) ;
    let mut histo : HashMap<u8,i32> = HashMap::new() ;
    for i in 0..255 {
        histo.insert(i,0);
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
            );
        let entropy_test = entropy_test("encrypted.bmp");
        arr_result[i] = entropy_test ;
        mean += arr_result[i]
    }
    mean/100.0
}
fn random_pixel_change(path : &str) {
    let mut image_matrix = image_to_matrix(path) ;
    let mut rnd = rand::thread_rng();
    let index_pix: u8 = rnd.gen() ;
    image_matrix[index_pix  as usize][index_pix  as usize ]  -= 1 ;
    let mut image_to_write = GrayImage::new(256, 256) ;
    for x in 0..256 {
        for y in 0..256 {
            image_to_write.put_pixel(
                x,
                y,
                image::Luma(image_matrix[x as usize][y as usize].to_be_bytes())
            );
        }
    }
    image_to_write.save_with_format(format!("modified_{}.bmp",path), ImageFormat::Bmp).expect("error while saving the image");
}
fn hamming_distance (x : i32, y : i32) -> i32 {
    let mut counter : i32 = 0; 
    let mut z = x ^ y ;
    while z != 0  {
        counter += z & 1;
        z >>= 1;
    }
    counter
    
}

pub fn ks_ps (c1 : &str , c2 : &str) -> f32 {
    let mut ks = 0f32;
    let c1_matrix = image_to_array(c1);
    let c2_matrix = image_to_array(c2);
    let factor : f32 = (100.00/((c2_matrix.len() as f32 ) * 8.0)) ;
    for i in 0..c1_matrix.len() {
        ks += (hamming_distance(c1_matrix[i] as i32 , c2_matrix[i] as i32)) as f32
    }
    ks * factor
}
fn npcr_uaci (c1 : &str , c2 : &str) -> (f32,f32){
    let factor = (100.0/(f32::powi(255.0,2)));
    let (mut npcr,mut uaci ) = (0f32, 0f32) ;
    let c1_matrix = image_to_matrix(format!("{}",c1).as_mut_str());
    let c2_matrix = image_to_matrix(format!("{}",c2).as_mut_str());
    for i in 0..256 {
        for j in 0..256 {
            if c1_matrix[i][j] != c2_matrix[i][j] {
                npcr += 1.0 ;
            }
            uaci += (f32::abs(c1_matrix[i][j] as f32 - c2_matrix[i][j] as f32))/ 255.0
        }
    }
    (npcr * factor ,uaci * factor)
}
pub fn robustness_analysis (image_name : &str) -> (f32 , f32 , f32) {
    let (mut npcr_avg , mut uaci_avg, mut ps_avg) = (0f32,0f32,0f32) ;
    for i in 0..100 {
        random_pixel_change(image_name);
        let m_k : [u8;8] = [101,20,30,40,50,60,128,99];
        let dkv = dkv(&m_k) ;
        let (blocks,w,h) = decompose(
            format!("modified_{}.bmp",image_name).as_mut_str()
        );
        let encrypted_blocks = encrypt_image(blocks,dkv);
        compose_image(encrypted_blocks.clone(),w,h,
                      format!("modified_encrypted_{}",image_name).as_mut_str());
        let (npcr,uaci) = npcr_uaci(
            format!("encrypted_{}",image_name).as_mut_str() ,
            format!("modified_encrypted_{}",image_name).as_mut_str()) ;
        let ps = ks_ps(
            format!("encrypted_{}.bmp",image_name).as_mut_str() ,
            format!("modified_encrypted_{}.bmp",image_name).as_mut_str()
        );
        println!("npcr = {} uaci = {} PS= {}",npcr , uaci , ps);
        npcr_avg += npcr ;
        uaci_avg += uaci ;
        ps_avg += ps

    }
    (npcr_avg/100.0 , uaci_avg/100.0 ,ps_avg/100.0 )
}
pub fn key_sensitivity_analysis(path : &str) -> f32{
    let mut mean : f32 = 0.0 ;
    let mut arr_result : [f32;100] = [0f32;100] ;
    for i in 0..100 {
        let mut rnd = rand::thread_rng() ;
        let _init_array : [f32;32] = rnd.gen();
        let mut dkv : [u8;64] = [0u8;64] ;
        let mut dkv2 = dkv ;
        dkv2[63] += 1 ;
        rnd.fill(&mut dkv) ;
        compose_image(
            encrypt_image(
                decompose(path).0 , dkv ) ,
            decompose(path).1 , decompose(path).2
            , "c1"
        );
        compose_image(
            encrypt_image(
                decompose(path).0 , dkv2 ) ,
            decompose(path).1 , decompose(path).2
            , "c2"
        );
        arr_result[i] = ks_ps("c1.bmp" , "c2.bmp") ;
        mean += arr_result[i]
    }
    mean/100.0
}

// correlation
// choose N random pixels
/*
 Matlab code :
 lena = double(imread('lena.bmp'))
 // horizontal
 x =  lena(:,1:end-1)
 y =  lena(:,2:end)
 rxy = corr2(x,y) -> h_corr value = 0.9430 /  0.0045
 scatter(x(:),y(:)) -> plot
 // vertical
 x =  lena(1:end-1,:)
 y =  lena(2:end,:)
 rxy = corr2(x,y) -> v_corr value =  0.9712 / -0.0025
 scatter(x(:),y(:)) -> plot
 // diagonal
 x = lena(1:end-1,1:end-1)
 y = lena(2:end,2:end)
 rxy = corr2(x,y) -> d_corr value = 0.9203 /   9.9148e-04

 scatter(x(:),y(:)) -> plot
 */
