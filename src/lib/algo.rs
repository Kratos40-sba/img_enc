use std::collections::VecDeque;
use crate::init::{array_to_matrix, g_inv_g, im1_im2, matrix_to_array, s_box_inv_s_box, xor};

// encrypt block .
fn encrypt_block(sub_image : [[u8;8];8] , dkv : [u8;64] , matrix : [[u8;8];8]) -> [[u8;8];8] {
    let y = xor(sub_image, matrix);
    let (s_box,_) = s_box_inv_s_box(dkv);
    let mut y_arr = matrix_to_array(&y);
    for i in 0..64 {
        y_arr[i] = s_box[y_arr[i] as usize];
    }
    let z = array_to_matrix(&y_arr);
    let (g,_) = g_inv_g(dkv);
    let encrypted_sub_image = mat_mul(g,z);
    encrypted_sub_image

}
// decrypt block .
fn decrypt_block(encrypted_block : [[u8;8];8] , dkv : [u8;64] , matrix : [[u8;8];8]) -> [[u8;8];8] {

    let (_ , inv_s_box) = s_box_inv_s_box(dkv);
    let (_,inv_g) = g_inv_g(dkv) ;
    let d = mat_mul(inv_g , encrypted_block);
    let mut d_array = matrix_to_array(&d);
    for i in 0..64 {
        d_array[i] = inv_s_box[d_array[i] as usize];
    }
    let e = array_to_matrix(&d_array);
    let decrypted_block = xor(e,matrix);
    decrypted_block
}
// encrypt image .
pub fn encrypt_image( blocks : Vec<[[u8;8];8]> , dkv : [u8;64] ) -> Vec<[[u8;8];8]> {
    let blocks_size = blocks.len() ;
    let (im1,im2) = im1_im2(dkv) ;
    let mut w: Vec<[[u8;8];8]> = Vec::new() ;
    let mut w_t : Vec<[[u8;8];8]> = Vec::new() ;
    for i in 0..blocks_size {
        if i == 0 {
            w.push(encrypt_block(blocks[i], dkv, im1));
        }else {
            w.push(encrypt_block(blocks[i], dkv, w[i-1]));
        }
    }
    for t in w {
        w_t.push(transpose(t))
    }
    for i in (0..blocks_size).rev() {
        if i == blocks_size - 1 {
            w_t[i] = encrypt_block(
                *w_t.get(i).expect("block does not exist"),
                dkv,
            im2)
        }else {
            w_t[i] = encrypt_block(
                *w_t.get(i+1).expect("block does not exist") ,
                dkv ,
                *w_t.get(i).expect("block does not exist")
            )

        }
    }
        w_t
    }



// decrypt image
pub fn decrypt_image(blocks: Vec<[[u8;8];8]>, dkv : [u8;64]) -> Vec<[[u8;8];8]>{
    let blocks_size = blocks.len() ;
    let (im1,im2) = im1_im2(dkv) ;
    let  mut w_t : Vec<[[u8;8];8]> = Vec::new() ;
    let  mut w : VecDeque<[[u8;8];8]> = VecDeque::new() ;
    let mut decrypted_image : Vec<[[u8;8];8]> = Vec::new() ;
    // first round of decryption process
    for i in (0..blocks_size).rev() {
        if i == blocks_size - 1 {
            w.push_front(decrypt_block(
                *blocks.get(i).expect("block does not exist "),
                dkv ,
                im2
            ))
        } else {
            w.push_front(decrypt_block(
                *blocks.get(i).expect("block does not exist"),
                dkv ,
                *blocks.get(i+1).expect("block does not exist")
            ))
        }
    }
    for t in Vec::from(w) {
        w_t.push(transpose(t))
    }
    for i in 0..blocks_size {
        if i == 0 {
            decrypted_image.push(decrypt_block(
                *w_t.get(i).expect("block does not exist") ,
                dkv ,
                im1
            ))
        }else {
            decrypted_image.push(decrypt_block(
                *w_t.get(i).expect("block does not exist") ,
                dkv ,
                *w_t.get(i-1).expect("block does not exist")
            ))
        }
    }

    decrypted_image
}
// matrix multiplication yes . (first)
fn mat_mul(a : [[i32;8];8], b : [[u8;8];8]) -> [[u8;8];8] {
    let mut res : [[u8;8];8] = [[0u8;8];8] ;
    for i in 0..8{
        for j in 0..8{
            for k in 0..8{
                res[i][j] = res[i][j].overflowing_add(a[i][k].overflowing_mul(b[k][j] as i32).0 as u8).0
            }
        }
    }
    res
}
fn _mat_mul (a : [[i32;8];8] , b : [[u8;8];8] ) -> [[u8;8];8] {
    let mut res = [[0u8;8];8];
    let mut b_i32 = [[0i32;8];8];
    let mut temp = [[0i32;8];8];
    for i in 0..8 {
        for j in 0..8 {
            b_i32[i][j] = b[i][j] as i32
        }
    }
    for i in 0..8 {
        for j in 0..8 {
            for k in 0..8 {
                temp[i][j] += a[i][k] * b_i32[k][j]
            }
        }
    }
    for i in 0..8 {
        for j in 0..8 {
            res[i][j] = temp[i][j] as u8
        }
    }
    res
}
fn transpose(mat : [[u8;8];8]) -> [[u8;8];8] {
    let mut mat_t = [[0u8;8];8];
    for i in 0..8 {
        for j in 0..8 {
            mat_t[i][j] = mat[j][i]
        }
    }
    mat_t
}
#[cfg(test)]
mod tests {
    use crate::init::{dkv, im1_im2};
    use super::*;
    #[test]
    fn test_encrypting_decrypting_one_block () {
        let m_k : [u8;8] = [0xb1, 0xcc, 0x58, 0x91, 0x44, 0xab, 0xca, 0x12];
        let dkv = dkv(&m_k) ;
        let expected_matrix : [[u8;8];8] = [
            [0,1,2,3,4,5,6,7],
            [8,9,10,11,12,13,14,15],
            [16,17,18,19,20,21,22,23],
            [24,25,26,27,28,29,30,31],
            [32,33,34,35,36,37,38,39],
            [40,41,42,43,44,45,46,47],
            [48,49,50,51,52,53,54,55],
            [56,57,58,59,60,61,62,63]
        ];
        let (_,im2) = im1_im2(dkv);
        let encrypted_block = encrypt_block(expected_matrix,dkv,im2);
        let decrypted_block = decrypt_block(encrypted_block,dkv,im2);
        assert_eq!(decrypted_block,expected_matrix);
        dbg!(decrypted_block);
    }
    #[test]
    fn test_transpose() {
        let expected_matrix : [[u8;8];8] = [
            [0,1,2,3,4,5,6,7],
            [8,9,10,11,12,13,14,15],
            [16,17,18,19,20,21,22,23],
            [24,25,26,27,28,29,30,31],
            [32,33,34,35,36,37,38,39],
            [40,41,42,43,44,45,46,47],
            [48,49,50,51,52,53,54,55],
            [56,57,58,59,60,61,62,63]
        ];
        let w_t = transpose(expected_matrix);
        assert_ne!(w_t,expected_matrix);
        dbg!(expected_matrix);
        dbg!(w_t);
    }
}