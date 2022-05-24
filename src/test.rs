use img_enc::algo::{decrypt_image, encrypt_image};
use img_enc::analysis::{key_sensitivity_analysis, ks_ps, robustness_analysis};
use img_enc::image_utils::{compose_image, decompose};
use img_enc::init::dkv;

fn main() {
  /*
  let (npcr,uaci,ps) = robustness_analysis("lena");
  println!("NPCR = {} \n UACI = {}\n PS = {}",npcr , uaci , ps);
   */
  let ks = key_sensitivity_analysis("lena.bmp");
  println!("Key Sensitivity mean = {}" , ks);
}
/*
 let m_k : [u8;8] = [101,20,30,40,50,60,128,99];
    let dkv = dkv(&m_k) ;
    let (blocks,w,h) = decompose("Lena_512_rust.bmp");
    let encrypted_blocks = encrypt_image(blocks,dkv);
    compose_image(encrypted_blocks.clone(),w,h,"decrypted_lena");
    let decrypted_blocks = decrypt_image(encrypted_blocks,dkv);
    compose_image(decrypted_blocks,w,h,"encrypted_lena")
 */
/*
    let uniformity_analysis = uniformity_analysis("lena.bmp");
    println!("mean in uniformity analysis is = {}",uniformity_analysis);
    // 252.70805
    let entropy_analysis = entropy_analysis("lena.bmp");
    println!("mean in entropy analysis is = {}",entropy_analysis);
    // 7.991547
 */
/*
 random_pixel_change("lena.bmp");
    let m_k : [u8;8] = [101,20,30,40,50,60,128,99];
    let dkv = dkv(&m_k) ;
    let (blocks,w,h) = decompose("modified_lena.bmp");
    let encrypted_blocks = encrypt_image(blocks,dkv);
    compose_image(encrypted_blocks.clone(),w,h,"modified_encrypted_lena");

 */
/*
    let (npcr , uaci ) = npcr_uaci("encrypted_lena.bmp","modified_encrypted_lena.bmp");
    println!("NPCR = {} \nUACI = {} " , npcr , uaci);

      let dkv1:[u8;64] = [160, 56, 20, 77, 161, 56, 20, 77, 49, 229, 205, 52, 49, 230,
        205, 52, 165, 235, 219, 77, 165, 235, 220, 77, 138, 218, 83,
        209, 138, 218, 83, 210, 159, 56, 20, 77, 160, 56, 20, 77, 49,
        228, 205, 52, 49, 229, 205, 52, 165, 235, 218, 77, 165, 235,
        219, 77, 138, 218, 83, 208, 138, 218, 83, 209];
    let dkv2 :[u8;64] = [160, 56, 20, 77, 161, 56, 20, 77, 49, 229, 205, 52, 49, 230,
    205, 52, 165, 235, 219, 77, 165, 235, 220, 77, 138, 218, 83,
    209, 138, 218, 83, 210, 159, 56, 20, 77, 160, 56, 20, 77, 49,
    228, 205, 52, 49, 229, 205, 52, 165, 235, 218, 77, 165, 235,
    219, 77, 138, 218, 83, 208, 138, 218, 83, 210] ;
    let (blocks,w,h) = decompose("lena.bmp");
    let encrypted_blocks = encrypt_image(blocks.clone(),dkv1);
    let encrypted_blocks2 = encrypt_image(blocks,dkv2);
    compose_image(encrypted_blocks,w,h,"c_k1");
    compose_image(encrypted_blocks2,w,h,"c_k2");
    let ks = ks_ps("c_k1.bmp","c_k2.bmp");
    println!("ks = {} \n",ks);
    let ps = ks_ps("encrypted_lena.bmp","modified_encrypted_lena.bmp");
    println!("PS = {} \n",ps);
 */