use oics::{
    core::{self, Scalar},
    imgcodecs, imgproc,
    transfer::{self, TransformableMat},
    types::{ImageFormat, RotateClipStrategy},
};
use std::{
    collections::HashMap,
    env,
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

fn find_target_angle(max_angle: u16, thresh_image: TransformableMat, threads: usize) -> f64 {
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
                deg as f64,
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
                    RotateClipStrategy::CONTAIN,
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

    return (get_most_possible_angle(standard_deviations) as f64) - (max_angle as f64);
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        panic!("参数数目有误！");
    }

    let instant = Instant::now();
    let mut original_image = transfer::TransformableMat::default();
    original_image
        .load_mat(&args[1], imgcodecs::IMREAD_COLOR)
        .expect("读取图片时发生错误");

    let thresh_image = {
        let gray_image =
            transfer::transfer_rgb_image_to_gray_image(&original_image).expect("RGB图转灰度图失败");

        transfer::transfer_gray_image_to_thresh_binary(&gray_image)
            .expect("灰度图二值化阈值处理失败")
    };

    // let mut standard_deviations = (vec![], vec![]);
    // for deg in -250..250 {
    //     let rotated_image = transfer::rotate_mat(
    //         &thresh_image,
    //         (deg as f64) / 10.0,
    //         1.0,
    //         imgproc::WARP_POLAR_LINEAR,
    //         core::BORDER_CONSTANT,
    //         Scalar::new(255.0, 255.0, 255.0, 0.0), // b g r
    //         RotateClipStrategy::CONTAIN,
    //     )
    //     .expect("旋转图像时发生错误！");

    //     let projection_standard_deviations =
    //         transfer::get_projection_standard_deviations(&rotated_image)
    //             .expect("计算投影标准差时发生错误");

    //     standard_deviations.0.push(projection_standard_deviations.0);
    //     standard_deviations.1.push(projection_standard_deviations.1);
    // }
    // let target_angle = ((get_most_possible_angle(standard_deviations) as f64) - 250.0) / 10.0;

    let target_angle = find_target_angle(45, thresh_image, 1);

    let duration = instant.elapsed().as_millis();
    println!("耗时 => {}ms", duration);

    let final_image = transfer::rotate_mat(
        &original_image,
        target_angle,
        1.0,
        imgproc::WARP_POLAR_LINEAR,
        core::BORDER_CONSTANT,
        Scalar::new(0.0, 0.0, 0.0, 0.0),
        RotateClipStrategy::CONTAIN,
    )
    .expect("旋转图像时发生错误");

    final_image
        .im_write("C:/Users/10563/Desktop/result.jpg", ImageFormat::JPEG, 100)
        .expect("图像输出失败");
}
