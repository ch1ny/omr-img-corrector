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

            for test_iter_idx in -450..450 {
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

                let distance = (original_image_rotate_angle - result_angle).abs();

                test_log
                    .write_all(
                        format!(
                            "------------------\nFile: {}\nrotate: {}deg\ndistance: {}deg\ntime_cost: {}ms\nneed_check: {}\nerror_type: {}\n------------------\n",
                            file_name,
                            original_image_rotate_angle,
                            distance,
                            algorithm_end - algorithm_start,
                            need_check,
                            if need_check {
                                "NOT_BELIEVED"
                            } else if distance > 0.5 {
                                "ERROR"
                            } else if distance > 0.4 {
                                "NOT_SO_RIGHT"
                            } else { "SUCCESS" }
                        )
                        .as_bytes(),
                    )
                    .unwrap();

                total_times += 1;
                total_mistake += distance;
                total_time_cost += algorithm_end - algorithm_start;

                println!("Test {} {}deg DONE", file_name, original_image_rotate_angle);
            }
        }

        println!("Average Error = {}deg", total_mistake / total_times as f64);
        println!(
            "Average Run Time = {}ms",
            total_time_cost / total_times as u128
        );
    }

    mod multi_thread {
        use crate::omr;
        use once_cell::sync::Lazy;
        use std::{collections::VecDeque, sync::Mutex, thread};

        type Job = Box<dyn FnOnce() + 'static + Send>;
        // 线程池最大并行任务数
        static MAX_WORKERS_COUNT: Mutex<usize> = Mutex::new(usize::MAX);
        // 执行中的任务数
        static RUNNING_WORKERS_COUNT: Mutex<usize> = Mutex::new(0);
        // 等待中的线程队列
        static WAITING_WORKERS_QUEUE: Mutex<Lazy<VecDeque<Job>>> =
            Mutex::new(Lazy::new(|| VecDeque::<Job>::new()));
        // 总任务数
        static TOTAL_COUNT: Mutex<usize> = Mutex::new(0);

        static MULTI_THREAD_INSTANT: Lazy<std::time::Instant> =
            Lazy::new(|| std::time::Instant::now());
        #[allow(dead_code)]
        // #[test]
        fn multi_thread_bench_test() {
            let mut io_file_paths = vec![];
            for entry in walkdir::WalkDir::new(super::DATA_SET_DIR_PATH) {
                let this_entry = entry.unwrap();
                if !this_entry.metadata().unwrap().is_file() {
                    continue;
                }

                let filepath = this_entry.path().display();
                let input_file_path = &filepath.to_string();
                let file_name = this_entry.file_name().to_str().unwrap();

                let output_file_path = String::from(
                    std::path::Path::new("../../dataset/result/projection")
                        .join(file_name)
                        .to_str()
                        .unwrap(),
                );

                if file_name.starts_with("image") {
                    io_file_paths.push((String::from(input_file_path), output_file_path));
                }
            }

            let algorithm_start = MULTI_THREAD_INSTANT.elapsed().as_millis();
            println!(">> Start at {}", algorithm_start);
            for (input_file, output_file) in io_file_paths {
                request_task(move || {
                    omr::correct_default(&input_file, &output_file, 45, 0.2, 248, 230, 150.0, 50.0)
                        .unwrap();

                    let mut total_count = TOTAL_COUNT.lock().unwrap();
                    if *total_count == 99 {
                        let algorithm_end = MULTI_THREAD_INSTANT.elapsed().as_millis();
                        println!(">> End at {}", algorithm_end);
                        std::process::exit(0);
                    }
                    *total_count += 1;
                });
            }

            loop {}
        }

        fn before_execute() {
            loop {
                let max_workers_count = MAX_WORKERS_COUNT.lock().unwrap();
                let mut running_workers_count = RUNNING_WORKERS_COUNT.lock().unwrap();
                if *running_workers_count >= *max_workers_count {
                    // 如果当前正在执行的任务数不小于最大任务数，不做任何处理
                    return;
                }
                let mut waiting_workers_queue = WAITING_WORKERS_QUEUE.lock().unwrap();
                // 从等待队列中取出第一个任务
                let next_worker = waiting_workers_queue.pop_front();
                drop(max_workers_count);
                drop(waiting_workers_queue);
                match next_worker {
                    None => {
                        return;
                    }
                    Some(bx) => {
                        // 如果存在待执行的任务则立即执行
                        *running_workers_count += 1;
                        drop(running_workers_count);
                        execute(bx);
                    }
                };
            }
        }

        fn execute<F>(f: F)
        where
            F: FnOnce() + 'static + Send,
        {
            thread::spawn(move || {
                f();
                // 任务执行完毕后使执行中任务计数减一
                let mut running_workers_count = RUNNING_WORKERS_COUNT.lock().unwrap();
                *running_workers_count -= 1;
                drop(running_workers_count);
                // 准备执行下一个任务
                before_execute();
            });
        }

        pub fn request_task<F>(f: F)
        where
            F: FnOnce() + 'static + Send,
        {
            let mut running_workers_count = RUNNING_WORKERS_COUNT.lock().unwrap();
            let max_workers_count = MAX_WORKERS_COUNT.lock().unwrap();
            if *running_workers_count < *max_workers_count {
                // 若当前执行中任务小于最大执行任务则立即执行
                *running_workers_count += 1;
                drop(running_workers_count);
                drop(max_workers_count);
                execute(f);
            } else {
                // 否则将任务添加到等待队列尾部
                let mut waiting_workers_queue = WAITING_WORKERS_QUEUE.lock().unwrap();
                waiting_workers_queue.push_back(Box::new(f));
            }
        }
    }
}
