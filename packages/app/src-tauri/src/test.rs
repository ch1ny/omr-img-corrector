use oics::{
    self,
    core::{self, Scalar},
    imgcodecs,
    imgproc,
    // prelude::MatTraitConstManual,
    transfer::{self, TransformableMatrix},
    types::{ImageFormat, RotateClipStrategy},
};
use rand::Rng;
use serde::Serialize;
use std::{
    collections::HashMap,
    path::Path,
    sync::{Arc, Mutex},
    thread,
    time::Instant,
};

fn get_most_possible_angle(vec: (Vec<f64>, Vec<f64>)) -> usize {
    // 处理垂直投影数据
    let vertical_vec = vec.0;
    let mut vertical_possibles: (f64, Vec<usize>) = (vertical_vec[0], vec![0]);
    for (index, val) in vertical_vec.iter().enumerate() {
        if *val > vertical_possibles.0 {
            vertical_possibles.0 = *val;
            vertical_possibles.1 = vec![index];
        } else if *val == vertical_possibles.0 {
            vertical_possibles.1.push(index);
        }
    }

    // 处理水平投影数据
    let horizontal_vec = vec.1;
    let mut horizontal_possibles: (f64, Vec<usize>) = (horizontal_vec[0], vec![0]);
    for (index, val) in horizontal_vec.iter().enumerate() {
        if *val > horizontal_possibles.0 {
            horizontal_possibles.0 = *val;
            horizontal_possibles.1 = vec![index];
        } else if *val == horizontal_possibles.0 {
            horizontal_possibles.1.push(index);
        }
    }

    // 唯一结果且相等
    if vertical_possibles.1.len() == 1
        && horizontal_possibles.1.len() == 1
        && vertical_possibles.1[0] == horizontal_possibles.1[0]
    {
        return vertical_possibles.1[0];
    }

    let mut candidate_hashmap = HashMap::new();
    for deg_index in vertical_possibles.1 {
        candidate_hashmap.entry(deg_index).or_insert((
            vertical_vec[deg_index as usize],
            horizontal_vec[deg_index as usize],
        ));
    }
    for deg_index in horizontal_possibles.1 {
        candidate_hashmap.entry(deg_index).or_insert((
            vertical_vec[deg_index as usize],
            horizontal_vec[deg_index as usize],
        ));
    }

    let mut sdp = 0.0;
    let mut most_possible_angle: Option<usize> = None;
    for (angle, (vsd, hsd)) in candidate_hashmap {
        let cur_sdp = vsd.powf(2.0) + hsd.powf(2.0);
        if sdp < cur_sdp {
            sdp = cur_sdp;
            most_possible_angle = Some(angle);
        }
    }

    match most_possible_angle {
        Some(value) => value,
        None => vertical_vec.len() / 2,
    }
}

fn find_target_angle(
    max_angle: u16,
    step: f64,
    thresh_image: TransformableMatrix,
    threads: usize,
) -> f64 {
    let max_angle = (max_angle as f64 / step) as u16;
    let min_angle = -(max_angle as i32);
    let range = min_angle..(max_angle as i32);

    let standard_deviations = if threads <= 1 {
        // 单线程
        let mut standard_deviations = (
            Vec::with_capacity(range.len()),
            Vec::with_capacity(range.len()),
        );

        for deg in range {
            let rotated_image = transfer::rotate_mat(
                &thresh_image,
                deg as f64 * step,
                1.0,
                imgproc::WARP_POLAR_LINEAR,
                core::BORDER_CONSTANT,
                Scalar::new(255.0, 255.0, 255.0, 0.0), // b g r
                RotateClipStrategy::DEFAULT,
            )
            .expect("旋转图像时发生错误！");

            let projection_standard_deviations =
                transfer::get_projection_standard_deviations(&rotated_image)
                    .expect("计算投影标准差时发生错误");

            standard_deviations.0.push(projection_standard_deviations.0);
            standard_deviations.1.push(projection_standard_deviations.1);
        }

        standard_deviations
    } else {
        // 多线程
        let mut handles = Vec::with_capacity(threads);
        let index = Arc::new(Mutex::new(0));

        let arc_standard_deviations =
            Arc::new(Mutex::new((vec![0.0; range.len()], vec![0.0; range.len()])));
        let arc_thresh_image = Arc::new(thresh_image);
        for _ in 0..threads {
            let ref_standard_deviations = Arc::clone(&arc_standard_deviations);
            let ref_index = Arc::clone(&index);
            let ref_thresh_image = Arc::clone(&arc_thresh_image);

            let handle = thread::spawn(move || loop {
                let angle = {
                    let mut index_locker = ref_index.lock().unwrap();
                    let current_index = *index_locker;
                    if current_index == (max_angle * 2) as i32 {
                        break;
                    }
                    *index_locker = current_index + 1;
                    current_index + min_angle
                };

                let rotated_image = transfer::rotate_mat(
                    &ref_thresh_image,
                    angle as f64,
                    1.0,
                    imgproc::WARP_POLAR_LINEAR,
                    core::BORDER_CONSTANT,
                    Scalar::new(255.0, 255.0, 255.0, 0.0), // b g r
                    RotateClipStrategy::DEFAULT,
                )
                .expect("旋转图像时发生错误！");

                let projection_standard_deviations =
                    transfer::get_projection_standard_deviations(&rotated_image)
                        .expect("计算投影标准差时发生错误");

                {
                    let mut rsd = ref_standard_deviations.lock().unwrap();
                    rsd.0[(angle - min_angle) as usize] = projection_standard_deviations.0;
                    rsd.1[(angle - min_angle) as usize] = projection_standard_deviations.1;
                }
            });
            handles.push(handle);
        }

        for handle in handles {
            handle.join().unwrap();
        }

        let sd = arc_standard_deviations.lock().unwrap().to_owned();
        sd
    };

    return ((get_most_possible_angle(standard_deviations) as f64) - (max_angle as f64)) * step;
}

#[allow(dead_code)]
#[derive(Serialize, Clone)]
struct TestMistakeResult {
    arithmetic_mean: f64,
    standard_deviation: f64,
    max_mistake: f64,
}

#[allow(dead_code)]
#[derive(Serialize, Clone)]
struct TestDurationResults {
    projection: u128,
    hough: u128,
    fft: u128,
}

#[allow(dead_code)]
#[derive(Serialize, Clone)]
struct TestMistakeResults {
    projection: TestMistakeResult,
    hough: TestMistakeResult,
    fft: TestMistakeResult,
}

#[allow(dead_code)]
#[derive(Serialize, Clone)]
pub struct TestResult {
    test_id: usize,
    durations: TestDurationResults,
    mistakes: TestMistakeResults,
}

#[allow(dead_code)]
#[derive(Serialize, Clone)]
enum TestProcessMethod {
    Projection,
    Hough,
    FFT,
}
#[allow(dead_code)]
#[derive(Serialize, Clone)]
struct TestProcessPayload {
    test_id: usize,
    method: TestProcessMethod,
    processed_count: usize,
    total_count: usize,
    processed_progress: f64,
}

const DATA_SET_DIR_PATH: &str = "./resources/test/dataset";

#[cfg(debug_assertions)]
const OUTPUT_DIR_PATH: &str = "../../../dataset/result";
#[cfg(not(debug_assertions))]
const OUTPUT_DIR_PATH: &str = "./resources/test/result";

#[tauri::command]
pub fn run_test(
    test_id: usize,
    projection_max_angle: u16,
    projection_angle_step: f64,
    projection_max_width: i32,
    projection_max_height: i32,
    hough_min_line_length: f64,
    hough_max_line_gap: f64,
    fft_canny_threshold_lower: f64,
    fft_canny_threshold_higher: f64,
    fft_min_line_length: f64,
    fft_max_line_gap: f64,
    window: tauri::Window,
) -> usize {
    thread::spawn(move || {
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

        let mut original_images_vec = vec![];
        for entry in walkdir::WalkDir::new(DATA_SET_DIR_PATH) {
            let this_entry = entry.unwrap();
            if !this_entry.metadata().unwrap().is_file() {
                continue;
            }

            let filepath = this_entry.path().display();
            let input_file_path = &filepath.to_string();
            let file_name = String::from(this_entry.file_name().to_str().unwrap());

            let random_angle = random.gen_range(-10.0..10.0);
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
            original_images_vec.push((original_image, random_angle, file_name));
        }

        let total_file_count = original_images_vec.len();

        // Projection :: Start
        let mut processed_count: usize = 0;
        window
            .emit(
                "run_test_progress_event",
                TestProcessPayload {
                    method: TestProcessMethod::Projection,
                    test_id,
                    processed_count,
                    total_count: total_file_count,
                    processed_progress: processed_count as f64 / total_file_count as f64,
                },
            )
            .unwrap();
        for (original_image, random_angle, file_name) in &original_images_vec {
            let projection_start = instant.elapsed().as_millis();

            let thresh_image = {
                let mut gray_image =
                    transfer::transfer_rgb_image_to_gray_image(&original_image).unwrap();
                let gray_image = gray_image
                    .shrink_to(projection_max_width, projection_max_height)
                    .unwrap();

                transfer::transfer_gray_image_to_thresh_binary(&gray_image).unwrap()
            };

            let projection_angle =
                find_target_angle(projection_max_angle, projection_angle_step, thresh_image, 1);

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
                    Path::new(OUTPUT_DIR_PATH)
                        .join("projection")
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
            processed_count += 1;

            window
                .emit(
                    "run_test_progress_event",
                    TestProcessPayload {
                        method: TestProcessMethod::Projection,
                        test_id,
                        processed_count,
                        total_count: total_file_count,
                        processed_progress: processed_count as f64 / total_file_count as f64,
                    },
                )
                .unwrap();
        }
        // Projection :: End

        // Hough :: Start
        let mut processed_count: usize = 0;
        window
            .emit(
                "run_test_progress_event",
                TestProcessPayload {
                    method: TestProcessMethod::Hough,
                    test_id,
                    processed_count,
                    total_count: total_file_count,
                    processed_progress: processed_count as f64 / total_file_count as f64,
                },
            )
            .unwrap();
        for (original_image, random_angle, file_name) in &original_images_vec {
            let hough_start = instant.elapsed().as_millis();
            // let min_line_length = original_image.get_mat().size().unwrap().width as f64 * 0.1;
            // let max_line_gap = min_line_length * 0.1;
            let hough_angle = oics::hough::get_angle_with_hough(
                &transfer::transfer_rgb_image_to_gray_image(&original_image).unwrap(),
                hough_min_line_length,
                hough_max_line_gap,
                &file_name,
                &(String::from(OUTPUT_DIR_PATH) + &"/edges/"),
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
                Path::new(OUTPUT_DIR_PATH)
                    .join("hough")
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

            processed_count += 1;
            window
                .emit(
                    "run_test_progress_event",
                    TestProcessPayload {
                        method: TestProcessMethod::Hough,
                        test_id,
                        processed_count,
                        total_count: total_file_count,
                        processed_progress: processed_count as f64 / total_file_count as f64,
                    },
                )
                .unwrap();
        }
        // Hough :: End

        // FFT :: Start
        let mut processed_count: usize = 0;
        window
            .emit(
                "run_test_progress_event",
                TestProcessPayload {
                    method: TestProcessMethod::FFT,
                    test_id,
                    processed_count,
                    total_count: total_file_count,
                    processed_progress: processed_count as f64 / total_file_count as f64,
                },
            )
            .unwrap();
        for (original_image, random_angle, file_name) in &original_images_vec {
            let fft_start = instant.elapsed().as_millis();

            let gray_image = transfer::transfer_rgb_image_to_gray_image(&original_image).unwrap();
            let fft_angle = oics::fft::get_angle_with_fft(
                &gray_image,
                fft_canny_threshold_lower,
                fft_canny_threshold_higher,
                fft_min_line_length,
                fft_max_line_gap,
                &file_name,
                &(String::from(OUTPUT_DIR_PATH) + &"/fft_lined/"),
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
                Path::new(OUTPUT_DIR_PATH)
                    .join("fft")
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

            processed_count += 1;

            window
                .emit(
                    "run_test_progress_event",
                    TestProcessPayload {
                        method: TestProcessMethod::FFT,
                        test_id,
                        processed_count,
                        total_count: total_file_count,
                        processed_progress: processed_count as f64 / total_file_count as f64,
                    },
                )
                .unwrap();
        }
        // FFT :: End

        // 基准测试
        let projection_average_duration = {
            let len = projection_run_time_array.len();
            projection_run_time_array.iter().sum::<u128>() / (len as u128)
        };
        let hough_average_duration = {
            let len = hough_run_time_array.len();
            hough_run_time_array.iter().sum::<u128>() / (len as u128)
        };
        let fft_average_duration = {
            let len = fft_run_time_array.len();
            fft_run_time_array.iter().sum::<u128>() / (len as u128)
        };

        // 误差测试
        let projection_mistake_result = TestMistakeResult {
            arithmetic_mean: oics::calculate::get_arithmetic_mean(&projection_deviation_array),
            standard_deviation: oics::calculate::get_standard_deviation(
                &projection_deviation_array,
            ),
            max_mistake: {
                let mut largest = projection_deviation_array[0];
                for val in projection_deviation_array {
                    if val > largest {
                        largest = val;
                    }
                }

                largest
            },
        };
        let hough_mistake_result = TestMistakeResult {
            arithmetic_mean: oics::calculate::get_arithmetic_mean(&hough_deviation_array),
            standard_deviation: oics::calculate::get_standard_deviation(&hough_deviation_array),
            max_mistake: {
                let mut largest = hough_deviation_array[0];
                for val in hough_deviation_array {
                    if val > largest {
                        largest = val;
                    }
                }

                largest
            },
        };
        let fft_mistake_result = TestMistakeResult {
            arithmetic_mean: oics::calculate::get_arithmetic_mean(&fft_deviation_array),
            standard_deviation: oics::calculate::get_standard_deviation(&fft_deviation_array),
            max_mistake: {
                let mut largest = fft_deviation_array[0];
                for val in fft_deviation_array {
                    if val > largest {
                        largest = val;
                    }
                }

                largest
            },
        };

        let result = TestResult {
            test_id,
            durations: TestDurationResults {
                projection: projection_average_duration,
                hough: hough_average_duration,
                fft: fft_average_duration,
            },
            mistakes: TestMistakeResults {
                projection: projection_mistake_result,
                hough: hough_mistake_result,
                fft: fft_mistake_result,
            },
        };

        window.emit("test_result", result).unwrap();
    });

    test_id
}
