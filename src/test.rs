use img_enc::algo::{decrypt_image, encrypt_image};
use img_enc::image_utils::{compose_image, decompose};
use img_enc::init::dkv;

fn main() {
    let m_k: [u8; 8] = [0xb1, 0xca, 0x52, 0x9a, 0x69, 0xab, 0xca, 0x12];
    let dkv = dkv(&m_k , "lena.bmp");
    let (blocks, w, h) = decompose("lena.bmp");
    let encrypted_blocks = encrypt_image(blocks, dkv);
    compose_image(encrypted_blocks.0.clone(), w, h, "encrypted_lena".into());
    let (blocks, w, h) = decompose("encrypted_lena.bmp");
    let decrypted_blocks = decrypt_image(blocks, dkv , encrypted_blocks.0 , encrypted_blocks.1);
    compose_image(decrypted_blocks.clone(), w, h, "decrypted_lena".into())
}