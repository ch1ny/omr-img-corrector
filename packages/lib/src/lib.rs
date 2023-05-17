pub use opencv::{
    core, highgui, imgcodecs, imgproc, prelude, types as opencv_types, Result as OpenCV_Result,
};

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
    use crate::{
        omr,
        transfer::{self, TransformableMatrix},
        types::{ImageFormat, RotateClipStrategy},
    };
    use opencv::{
        core::{Scalar, BORDER_CONSTANT},
        imgcodecs, imgproc,
    };
    use rand::Rng;
    use std::{io::Write, path::Path};

    const DATA_SET_DIR_PATH: &str = "../../dataset/dataset";
    #[allow(dead_code)]
    // #[test]
    fn crate_omr_correct_default_test() {
        let mut random = rand::thread_rng();
        let mut total_times: u32 = 0;
        let mut total_mistake = 0.0f64;
        let mut total_time_cost: u128 = 0;
        let instant = std::time::Instant::now();

        for entry in walkdir::WalkDir::new(DATA_SET_DIR_PATH) {
            let this_entry = entry.unwrap();
            if !this_entry.metadata().unwrap().is_file() {
                continue;
            }

            let filepath = this_entry.path().display();
            let input_file_path = &filepath.to_string();
            let file_name = this_entry.file_name().to_str().unwrap();

            let random_angle = random.gen_range(-45.0..45.0);
            // let random_angle = 0.0;
            let original_image = transfer::rotate_mat(
                &transfer::TransformableMatrix::new(input_file_path, imgcodecs::IMREAD_COLOR)
                    .unwrap(),
                -random_angle,
                1.0,
                imgproc::INTER_LINEAR,
                BORDER_CONSTANT,
                Scalar::new(255.0, 255.0, 255.0, 0.0),
                RotateClipStrategy::DEFAULT,
            )
            .unwrap();

            let original_image =
                transfer::transfer_rgb_image_to_gray_image(&original_image).unwrap();
            let original_image = TransformableMatrix::from_matrix(&{
                let mut dst = opencv::prelude::Mat::default();
                imgproc::cvt_color(
                    // 添加高斯噪声
                    // &add_gaussian_noise(original_image.get_mat(), 0.0, 255.0),
                    // 添加椒盐噪声
                    // &add_salt_and_pepper_noise(original_image.get_mat(), 0.01),
                    &original_image.get_mat(),
                    &mut dst,
                    imgproc::COLOR_GRAY2RGB,
                    0,
                )
                .unwrap();
                dst
            });

            original_image
                .im_write("./tmp.jpg", ImageFormat::JPEG, 100)
                .unwrap();

            let algorithm_start = instant.elapsed().as_millis();

            let (result_angle, need_check) = omr::correct_default(
                &"./tmp.jpg",
                Path::new("../../dataset/result/projection")
                    .join(file_name)
                    .to_str()
                    .unwrap(),
                45,
                0.2,
                248,
                230,
                150.0,
                50.0,
            )
            .unwrap();

            let algorithm_end = instant.elapsed().as_millis();

            if !need_check {
                if (random_angle - result_angle).abs() >= 0.4 {
                    println!("{}", (random_angle - result_angle).abs());
                }
                assert!(
                    // 99.9% 不会超过 0.4; 近似 100% 不会超过 0.5(测试中出现过一次超过0.5°的情况)
                    (random_angle - result_angle).abs() < 0.5,
                    "{}, {}",
                    file_name,
                    (random_angle - result_angle).abs()
                );

                total_times += 1;
                total_mistake += (random_angle - result_angle).abs();
                total_time_cost += algorithm_end - algorithm_start;
            } else {
                println!("{} => {}", file_name, (random_angle - result_angle).abs());
            }
        }

        println!("Average Error = {}deg", total_mistake / total_times as f64);
        println!(
            "Average Run Time = {}ms",
            total_time_cost / total_times as u128
        );
    }

    #[allow(dead_code)]
    #[test]
    fn crate_omr_correct_default_all_situation_test() {
        let mut total_times: u32 = 0;
        let mut total_mistake = 0.0f64;
        let mut total_time_cost: u128 = 0;
        let instant = std::time::Instant::now();
        let mut test_log = std::fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open("./test_logs/crate_omr_correct_default_all_situation_test.log")
            .unwrap();

        for entry in walkdir::WalkDir::new(DATA_SET_DIR_PATH) {
            let this_entry = entry.unwrap();
            if !this_entry.metadata().unwrap().is_file() {
                continue;
            }

            let filepath = this_entry.path().display();
            let input_file_path = &filepath.to_string();
            let file_name = this_entry.file_name().to_str().unwrap();

            for test_iter_idx in -450..-449 {
                let original_image_rotate_angle = test_iter_idx as f64 * 0.1;

                let original_image = transfer::rotate_mat(
                    &transfer::TransformableMatrix::new(input_file_path, imgcodecs::IMREAD_COLOR)
                        .unwrap(),
                    -original_image_rotate_angle,
                    1.0,
                    imgproc::INTER_LINEAR,
                    BORDER_CONSTANT,
                    Scalar::new(255.0, 255.0, 255.0, 0.0),
                    RotateClipStrategy::DEFAULT,
                )
                .unwrap();

                let original_image =
                    transfer::transfer_rgb_image_to_gray_image(&original_image).unwrap();
                let original_image = TransformableMatrix::from_matrix(&{
                    let mut dst = opencv::prelude::Mat::default();
                    imgproc::cvt_color(
                        // 添加高斯噪声
                        // &add_gaussian_noise(original_image.get_mat(), 0.0, 255.0),
                        // 添加椒盐噪声
                        // &add_salt_and_pepper_noise(original_image.get_mat(), 0.01),
                        &original_image.get_mat(),
                        &mut dst,
                        imgproc::COLOR_GRAY2RGB,
                        0,
                    )
                    .unwrap();
                    dst
                });

                original_image
                    .im_write("./tmp.jpg", ImageFormat::JPEG, 100)
                    .unwrap();

                let algorithm_start = instant.elapsed().as_millis();

                let (result_angle, need_check) = omr::correct_default(
                    &"./tmp.jpg",
                    Path::new("../../dataset/result/projection")
                        .join(file_name)
                        .to_str()
                        .unwrap(),
                    45,
                    0.2,
                    248,
                    230,
                    150.0,
                    50.0,
                )
                .unwrap();

                let algorithm_end = instant.elapsed().as_millis();

                test_log
                    .write_all(
                        format!(
                            "------------------\nFile: {}\nrotate: {}deg\ndistance: {}deg\ntime_cost: {}ms\nneed_check: {}\n------------------\n",
                            file_name,
                            original_image_rotate_angle,
                            (original_image_rotate_angle - result_angle).abs(),
                            algorithm_end - algorithm_start,
                            need_check
                        )
                        .as_bytes(),
                    )
                    .unwrap();

                if !need_check {
                    total_times += 1;
                    total_mistake += (original_image_rotate_angle - result_angle).abs();
                    total_time_cost += algorithm_end - algorithm_start;
                }
            }
        }

        println!("Average Error = {}deg", total_mistake / total_times as f64);
        println!(
            "Average Run Time = {}ms",
            total_time_cost / total_times as u128
        );
    }
}
