use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
    thread,
};

use opencv::{core::Scalar, imgproc};

use crate::{
    transfer::{
        get_projection_standard_deviations, rotate_mat, transfer_gray_image_to_thresh_binary,
        transfer_rgb_image_to_gray_image, TransformableMatrix,
    },
    types::RotateClipStrategy,
};

pub fn get_angle_with_projections(
    src_img: &TransformableMatrix,
    max_angle: u16,
    step: f64,
    resize_scale: f64,
    threads: usize,
) -> f64 {
    let scaled_img = {
        let mut cloned_img = src_img.clone();
        cloned_img.resize_self(resize_scale).unwrap().to_owned()
    };
    // 二值化图像
    let thresh_image = {
        let gray_image = transfer_rgb_image_to_gray_image(&scaled_img).unwrap();
        transfer_gray_image_to_thresh_binary(&gray_image).unwrap()
    };

    // 查找目标角度
    let projection_angle = {
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
                let rotated_image = rotate_mat(
                    &thresh_image,
                    deg as f64 * step,
                    1.0,
                    imgproc::WARP_POLAR_LINEAR,
                    opencv::core::BORDER_CONSTANT,
                    Scalar::new(255.0, 255.0, 255.0, 0.0), // b g r
                    RotateClipStrategy::DEFAULT,
                )
                .expect("旋转图像时发生错误！");

                let projection_standard_deviations =
                    get_projection_standard_deviations(&rotated_image)
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

                    let rotated_image = rotate_mat(
                        &ref_thresh_image,
                        angle as f64,
                        1.0,
                        imgproc::WARP_POLAR_LINEAR,
                        opencv::core::BORDER_CONSTANT,
                        Scalar::new(255.0, 255.0, 255.0, 0.0), // b g r
                        RotateClipStrategy::DEFAULT,
                    )
                    .expect("旋转图像时发生错误！");

                    let projection_standard_deviations =
                        get_projection_standard_deviations(&rotated_image)
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

        // return
        ((
            // 获取最有可能的角度
            {
                // 处理垂直投影数据
                let vertical_vec = standard_deviations.0;
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
                let horizontal_vec = standard_deviations.1;
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
                    vertical_possibles.1[0]
                } else {
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
            } as f64
        ) - (max_angle as f64))
            * step
    };

    return projection_angle;
}
