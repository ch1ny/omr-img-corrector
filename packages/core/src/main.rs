#[allow(unused_imports)]
use noise::add_gaussian_noise;
#[allow(unused_imports)]
use oics::{
    self,
    core::{self, Scalar},
    imgcodecs, imgproc,
    transfer::{self, TransformableMatrix},
    types::{ImageFormat, RotateClipStrategy},
};
use rand::Rng;
use std::{path::Path, time::Instant};

mod noise;

const DATA_SET_DIR_PATH: &str = "../../dataset/dataset";
#[allow(dead_code)]
fn run_test(p: bool, h: bool, f: bool) {
    let instant = Instant::now();
    let mut random = rand::thread_rng();

    // 基准测试
    let mut projection_run_time_array: Vec<u128> = vec![];
    let mut hough_run_time_array: Vec<u128> = vec![];
    let mut fft_run_time_array: Vec<u128> = vec![];
    // 准确度测试
    let mut projection_deviation_array: Vec<f64> = vec![];
    let mut hough_deviation_array: Vec<f64> = vec![];
    let mut fft_deviation_array: Vec<f64> = vec![];

    for entry in walkdir::WalkDir::new(DATA_SET_DIR_PATH) {
        let this_entry = entry.unwrap();
        if !this_entry.metadata().unwrap().is_file() {
            continue;
        }

        let filepath = this_entry.path().display();
        let input_file_path = &filepath.to_string();
        let file_name = this_entry.file_name().to_str().unwrap();

        let random_angle = random.gen_range(-10.0..10.0);
        let original_image = transfer::rotate_mat(
            &transfer::TransformableMatrix::new(input_file_path, imgcodecs::IMREAD_COLOR).unwrap(),
            -random_angle,
            1.0,
            imgproc::INTER_LINEAR,
            core::BORDER_CONSTANT,
            Scalar::new(255.0, 255.0, 255.0, 0.0),
            RotateClipStrategy::CONTAIN,
        )
        .unwrap();

        // 添加高斯噪声
        // let original_image = transfer::transfer_rgb_image_to_gray_image(&original_image).unwrap();
        // let original_image = TransformableMatrix::from_matrix(&{
        //     let mut dst = oics::prelude::Mat::default();
        //     imgproc::cvt_color(
        //         &add_gaussian_noise(original_image.get_mat(), 0.0, 20.0),
        //         &mut dst,
        //         imgproc::COLOR_GRAY2RGB,
        //         0,
        //     )
        //     .unwrap();
        //     dst
        // });

        if p {
            let projection_start = instant.elapsed().as_millis();
            let projection_angle =
                oics::projection::get_angle_with_projections(&original_image, 45, 0.2, 0.2, 1);

            let final_image = transfer::rotate_mat(
                &original_image,
                projection_angle,
                1.0,
                imgproc::INTER_LINEAR,
                core::BORDER_CONSTANT,
                Scalar::new(255.0, 255.0, 255.0, 0.0),
                RotateClipStrategy::CONTAIN,
            )
            .unwrap();

            final_image
                .im_write(
                    Path::new("../../dataset/result/projection")
                        .join(file_name)
                        .to_str()
                        .unwrap(),
                    ImageFormat::JPEG,
                    100,
                )
                .unwrap();

            let projection_end = instant.elapsed().as_millis();
            projection_run_time_array.push(projection_end - projection_start);
            projection_deviation_array.push((projection_angle - random_angle).abs());
        }

        if h {
            let hough_start = instant.elapsed().as_millis();
            // let min_line_length = original_image.get_mat().size().unwrap().width as f64 * 0.1;
            // let max_line_gap = min_line_length * 0.1;
            let hough_angle = oics::hough::get_angle_with_hough(
                &transfer::transfer_rgb_image_to_gray_image(&original_image).unwrap(),
                125.0,
                15.0,
                file_name,
                "../../dataset/result/edges/",
            )
            .unwrap();
            transfer::rotate_mat(
                &original_image,
                hough_angle,
                1.0,
                imgproc::INTER_LINEAR,
                core::BORDER_CONSTANT,
                Scalar::new(255.0, 255.0, 255.0, 0.0),
                RotateClipStrategy::CONTAIN,
            )
            .unwrap()
            .im_write(
                Path::new("../../dataset/result/hough")
                    .join(file_name)
                    .to_str()
                    .unwrap(),
                ImageFormat::JPEG,
                100,
            )
            .unwrap();

            let hough_end = instant.elapsed().as_millis();
            hough_run_time_array.push(hough_end - hough_start);
            hough_deviation_array.push((hough_angle - random_angle).abs());
        }

        if f {
            let fft_start = instant.elapsed().as_millis();

            let gray_image = transfer::transfer_rgb_image_to_gray_image(&original_image).unwrap();

            let fft_angle = oics::fft::get_angle_with_fft(
                &gray_image,
                125.0,
                150.0,
                150.0,
                75.0,
                file_name,
                "../../dataset/result/canny/",
            )
            .unwrap();
            transfer::rotate_mat(
                &original_image,
                fft_angle,
                1.0,
                imgproc::INTER_LINEAR,
                core::BORDER_CONSTANT,
                Scalar::new(255.0, 255.0, 255.0, 0.0),
                RotateClipStrategy::CONTAIN,
            )
            .unwrap()
            .im_write(
                Path::new("../../dataset/result/fft")
                    .join(file_name)
                    .to_str()
                    .unwrap(),
                ImageFormat::JPEG,
                100,
            )
            .unwrap();

            let fft_end = instant.elapsed().as_millis();
            fft_run_time_array.push(fft_end - fft_start);
            fft_deviation_array.push((fft_angle - random_angle).abs());
        }
    }

    println!("基准测试");
    if p {
        println!("{}", {
            let len = projection_run_time_array.len();
            projection_run_time_array.iter().sum::<u128>() / (len as u128)
        });
    }
    if h {
        println!("{:?}", {
            let len = hough_run_time_array.len();
            hough_run_time_array.iter().sum::<u128>() / (len as u128)
        });
    }
    if f {
        println!("{:?}", {
            let len = fft_run_time_array.len();
            fft_run_time_array.iter().sum::<u128>() / (len as u128)
        });
    }

    println!("误差测试");
    if p {
        println!(
            "{}, {}, {}",
            oics::calculate::get_arithmetic_mean(&projection_deviation_array),
            oics::calculate::get_standard_deviation(&projection_deviation_array),
            {
                let mut largest = projection_deviation_array[0];
                for val in projection_deviation_array {
                    if val > largest {
                        largest = val;
                    }
                }

                largest
            }
        );
    }
    if h {
        println!(
            "{}, {}, {}",
            oics::calculate::get_arithmetic_mean(&hough_deviation_array),
            oics::calculate::get_standard_deviation(&hough_deviation_array),
            {
                let mut largest = hough_deviation_array[0];
                for val in hough_deviation_array {
                    if val > largest {
                        largest = val;
                    }
                }

                largest
            }
        );
    }
    if f {
        println!(
            "{}, {}, {}",
            oics::calculate::get_arithmetic_mean(&fft_deviation_array),
            oics::calculate::get_standard_deviation(&fft_deviation_array),
            {
                let mut largest = fft_deviation_array[0];
                for val in fft_deviation_array {
                    if val > largest {
                        largest = val;
                    }
                }

                largest
            }
        );
    }
}

fn main() {
    run_test(true, true, false);
}

// 测试
#[cfg(test)]
mod tests {
    use oics::{
        self,
        core::{self, Scalar},
        imgcodecs, imgproc, omr,
        transfer::{self, TransformableMatrix},
        types::{ImageFormat, RotateClipStrategy},
    };
    use rand::Rng;
    use std::path::Path;

    #[allow(unused_imports)]
    use crate::{
        noise::{add_gaussian_noise, add_salt_and_pepper_noise},
        DATA_SET_DIR_PATH,
    };

    #[test]
    fn lib_omr() {
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
                core::BORDER_CONSTANT,
                Scalar::new(255.0, 255.0, 255.0, 0.0),
                RotateClipStrategy::DEFAULT,
            )
            .unwrap();

            let original_image =
                transfer::transfer_rgb_image_to_gray_image(&original_image).unwrap();
            let original_image = TransformableMatrix::from_matrix(&{
                let mut dst = oics::prelude::Mat::default();
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
}
