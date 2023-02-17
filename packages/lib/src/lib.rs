pub use opencv::{highgui, imgcodecs};

pub mod transfer;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut image = transfer::TransformableMat::default();
        let result = image.load_mat("./01234.jpg", imgcodecs::IMREAD_COLOR);
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
