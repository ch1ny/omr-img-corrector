pub use opencv::{core, highgui, imgcodecs, imgproc, prelude};

pub mod calculate;

pub mod constants;

pub mod fft;

pub mod hough;

pub mod omr;

pub mod projection;

pub mod transfer;

pub mod types;

#[cfg(test)]
mod tests {
    use crate::constants::ImReadFlags;

    use super::*;

    #[test]
    fn it_works() {
        let mut image = transfer::TransformableMatrix::default();
        let result = image.load_mat("./01234.jpg", ImReadFlags::from(ImReadFlags::Color));
        match result {
            Err(_) => assert!(false),
            Ok(original_image) => {
                match transfer::transfer_rgb_image_to_gray_image(&original_image) {
                    Err(_) => assert!(false),
                    Ok(gray_image) => {
                        match transfer::transfer_gray_image_to_thresh_binary(&gray_image) {
                            Err(_) => assert!(false),
                            Ok(_) => {
                                assert!(true);
                            }
                        }
                    }
                }
            }
        }
    }
}
