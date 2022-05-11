use img_enc::image_utils::* ;
use img_enc::init::dkv;
use img_enc::algo::{decrypt_image, encrypt_image};
fn main() {
    let m_k : [u8;8] = [101,20,30,40,50,60,128,99];
    let dkv = dkv(&m_k) ;
    let (blocks,w,h) = decompose("Lena_512_rust.bmp");
    let encrypted_blocks = encrypt_image(blocks,dkv);
    compose_image(encrypted_blocks.clone(),w,h,"decrypted_lena");
    let decrypted_blocks = decrypt_image(encrypted_blocks,dkv);
    compose_image(decrypted_blocks,w,h,"encrypted_lena")

}