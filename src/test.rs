use img_enc::image_utils::* ;
use img_enc::init::dkv;
use img_enc::algo::{decrypt_image, encrypt_image};
fn main() {
    let m_k : [u8;8] = [10,20,30,40,50,60,70,80];
    let dkv = dkv(&m_k) ;
    let (blocks,w,h) = decompose("Pepper.bmp");
    let encrypted_blocks = encrypt_image(blocks,dkv);
    compose_image(encrypted_blocks.clone(),w,h,"dec_p".into());
    let decrypted_blocks = decrypt_image(encrypted_blocks,dkv);
    compose_image(decrypted_blocks,w,h,"enc_p".into());
}