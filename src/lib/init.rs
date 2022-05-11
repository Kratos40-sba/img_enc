use sha2::{Digest, Sha512};

// generate DKv
// Dkv generates by using an master key
pub fn dkv (master_key : &[u8;8]) -> [u8;64] {
    let mut hasher = Sha512::new() ;
    hasher.update(master_key);
    let result = hasher.finalize() ;
    <[u8; 64]>::try_from(result.as_slice()).unwrap()
}
// reshaping from array to matrix and from matrix to array
pub fn array_to_matrix (array : &[u8;64]) -> [[u8;8];8] {
    let mut matrix : [[u8;8];8] = [[0u8;8];8];
    let mut counter = 0 ;
    for i in 0..8  {
        for j in 0..8 {
            matrix[i][j] = array[counter];
            counter += 1 ;
        }
    }
    matrix
}
pub fn matrix_to_array (matrix : &[[u8;8];8]) -> [u8;64] {
    let mut array : [u8;64] = [0u8;64];
    let mut counter = 0 ;
    for i in matrix {
        for j in i {
            array[counter] = *j;
            counter += 1
        }
    }
    array
}
// xor between 2 matrix's
pub fn xor(a : [[u8;8];8] , b : [[u8;8];8]) -> [[u8;8];8] {
    let mut result = [[0u8;8];8];
    for i in 0..8 {
        for j in 0..8 {
            result[i][j] = a[i][j] ^ b[i][j]
        }
    }
    result
}
// generate im1 & im2
pub fn im1_im2 (dkv : [u8;64]) -> ([[u8;8];8],[[u8;8];8]) {
    let mut first_hasher = Sha512::new();
    first_hasher.update(&dkv);
    let mut result = first_hasher.finalize() ;
    let im1 = array_to_matrix(&<[u8; 64]>::try_from(result.as_slice()).unwrap());
    let mut second_hasher = Sha512::new();
    second_hasher.update(matrix_to_array(&im1));
    result = second_hasher.finalize() ;
    let im2 = array_to_matrix(&<[u8; 64]>::try_from(result.as_slice()).unwrap());
    (im1,im2)
}
// generate G & inv_G

pub fn g_inv_g (dkv : [u8;64]) -> ([[i32;8];8],[[i32;8];8]) {
    let mut g = [[0i32; 8]; 8];
    let mut inv_g = [[0i32; 8]; 8];
    let m: [[u8; 4]; 4] = [
        [dkv[0], dkv[8], dkv[16], dkv[24]],
        [dkv[1], dkv[9], dkv[17], dkv[25]],
        [dkv[2], dkv[10], dkv[18], dkv[26]],
        [dkv[3], dkv[11], dkv[19], dkv[27]],
    ];
    let id_m: [[i32; 4]; 4] = [[1, 0, 0, 0], [0, 1, 0, 0], [0, 0, 1, 0], [0, 0, 0, 1]];
    for i in 0..4 {
        for j in 0..4 {
            g[i][j] = m[i][j] as i32;
        }
    }
    for (i, i2) in (4..8).zip(0..4) {
        for (j, j2) in (4..8).zip(0..4) {
            g[i][j] = m[i2][j2] as i32;
        }
    }
    for i in 0..4 {
        for (j, j2) in (4..8).zip(0..4) {

            g[i][j] = (m[i][j2] as i32) + id_m[i][j2]
        }
    }
    for (i,i2) in (4..8).zip(0..4) {
        for j in 0..4 {
            g[i][j] = (m[i2][j] as i32 )- id_m[i2][j]
        }
    }
    // inv_g
    for i in 0..4 {
        for j in 0..4 {
            inv_g[i][j] = m[i][j] as i32;
        }
    }
    for (i, i2) in (4..8).zip(0..4) {
        for (j, j2) in (4..8).zip(0..4) {
            inv_g[i][j] = m[i2][j2] as i32;
        }
    }
    for i in 0..4 {
        for (j, j2) in (4..8).zip(0..4) {
            inv_g[i][j] = (-1*m[i][j2] as i32 ) - id_m[i][j2]
        }
    }
    for (i,i2) in (4..8).zip(0..4) {
        for j in 0..4 {
            inv_g[i][j] = (-1 * m[i2][j] as i32 ) + id_m[i2][j]
        }
    }
    (g, inv_g)
}

// generate s_box & inv_s_box
pub fn s_box_inv_s_box (dkv : [u8;64]) -> ([u8;256],[u8;256]) {
    let mut ks : [u8;8] = [0u8;8];
    let mut r : [u8;4] = [0u8;4];
    let mut t : [u8;4] = [0u8;4];
    let mut s_box : [u8;256] = [0u8;256];
    let mut inv_s_box : [u8;256] = [0u8;256];
    let temp  = array_to_matrix(&dkv);
    for i in 0..8 {
        for j in 0..8 {
            ks[i] = temp[i][j] ^ ks[i]
        }
    }
    let mut ir = 0 ;
    let mut it = 0 ;
    for k in &ks {
        if k % 2 == 0 && ir < 4 {
            r[ir] = *k;
            // change the lsb to 0
            r[ir] = r[ir] ^ (r[ir] & 1);
            ir += 1 ;
        }else if it < 4 {
            t[it] = *k;
            // change lsb to 1
            t[it] |= 1;
            it += 1 ;
        }
    }
    for i in 0..256 {
        s_box[i] = i as u8
    }
    for j in 1..256 {
        for i in 0..4 {
            s_box[j] = (s_box[j].overflowing_mul((s_box[j - 1].overflowing_mul( r[i])).0.overflowing_add( t[i]).0)).0
        }
    }
    for i in 0..256 {
        inv_s_box[s_box[i] as usize] = i as u8  ;
    }

    (s_box,inv_s_box)

}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn dkv_test (){
        let m_k : [u8;8] = [0xb1, 0xcc, 0x58, 0x91, 0x44, 0xab, 0xca, 0x12];
        let dkv = dkv(&m_k) ;
        assert_eq!(dkv.len(),64)
    }
    #[test]
    fn test_array_to_matrix_and_vesa_versa() {
        let mut arr : [u8;64] = [0u8;64] ;
        for i in 0..64 {
            arr[i] = i as u8;
        }
        let matrix = array_to_matrix(&arr);
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
        let array = matrix_to_array(&matrix);
        assert_eq!(matrix,expected_matrix);
        assert_eq!(array,arr);
    }
    #[test]
    fn test_xor () {
        let zeros = [[0u8;8];8];
        let ones  = [[1u8;8];8];
        let res1 = xor(zeros,zeros);
        let res2 = xor(ones,ones);
        let res3 = xor(ones,zeros);
        assert_eq!(res1,zeros);
        assert_eq!(res2,zeros);
        assert_eq!(res3,ones);

    }
    #[test]
    fn print_im1_im2 () {
        let m_k : [u8;8] = [0xb1, 0xcc, 0x58, 0x91, 0x44, 0xab, 0xca, 0x12];
        let dkv = dkv(&m_k) ;
        let (im1,im2) = im1_im2(dkv);
        dbg!(im1);
        dbg!(im2);
    }


}