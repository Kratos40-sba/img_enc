use image::{GrayImage, ImageFormat};


// image to vec<[[u8;8];8]  .
pub fn decompose (path : &str) -> ( Vec<[[u8;8];8]> , u32 , u32) {
    let mut blocks : Vec<[[u8;8];8]> = Vec::new();
    let mut block  : [[u8;8];8] = [[0u8;8];8];
    let img = image::open(path).expect("error while opening file").to_luma8();
    let (w,h) = img.dimensions() ;
    for i in (0..h).step_by(8) {
        for j in (0..w).step_by(8) {
            for (x, sx) in (j..j + 8).zip(0..64) {
                for (y, sy) in (i..i + 8).zip(0..64) {
                    block[sy][sx] = img.get_pixel(x,y).0[0]
                }
            }
            blocks.push(block);
        }
    }
    ( blocks , w, h )
}
// vec<[[u8;8];8]> to image .
pub fn compose_image (decomposed_image : Vec<[[u8;8];8]> , w : u32 , h : u32 , image_name : &str) {
    let mut image_to_write = GrayImage::new(w, h) ;
    let mut counter = 0 ;
    for i in (0..h).step_by(8) {
        for j in (0..w).step_by(8){
            for (x, sx) in (j..j + 8).zip(0..64) {
                for (y, sy) in (i..i + 8).zip(0..64) {
                  image_to_write.put_pixel(
                      x,
                      y,
                      image::Luma(decomposed_image[counter][sy][sx].to_be_bytes())
                  );
                }
            }
            counter += 1 ;
        }
    }
    image_to_write.save_with_format(format!("{}.bmp",image_name), ImageFormat::Bmp).expect("error while saving the image");
}