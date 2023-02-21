use oics::{
    core::{self, Scalar},
    imgcodecs, imgproc, transfer,
    types::ImageFormat,
};
use std::{collections::HashMap, env};

#[allow(dead_code)]
fn get_most_possible_angle(vec: (Vec<f64>, Vec<f64>)) -> usize {
    // 处理垂直投影数据
    let vertical_vec = vec.0;
    let mut vertical_possibles: (f64, Vec<usize>) = (vertical_vec[0], vec![0]);
    for index in 0..vertical_vec.len() {
        let val = vertical_vec[index];
        if val > vertical_possibles.0 {
            vertical_possibles.0 = val;
            vertical_possibles.1 = vec![index];
        } else if val == vertical_possibles.0 {
            vertical_possibles.1.push(index);
        }
    }

    // 处理水平投影数据
    let horizontal_vec = vec.1;
    let mut horizontal_possibles: (f64, Vec<usize>) = (horizontal_vec[0], vec![0]);
    for index in 0..horizontal_vec.len() {
        let val = horizontal_vec[index];
        if val > horizontal_possibles.0 {
            horizontal_possibles.0 = val;
            horizontal_possibles.1 = vec![index];
        } else if val == horizontal_possibles.0 {
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

    let mut sdp: Option<f64> = None;
    let mut most_possible_angle: Option<usize> = None;
    for (angle, (vsd, hsd)) in candidate_hashmap {
        let cur_sdp = vsd.powf(2.0) + hsd.powf(2.0);
        match sdp {
            Some(biggest_sdp) => {
                if biggest_sdp < cur_sdp {
                    sdp = Some(cur_sdp);
                    most_possible_angle = Some(angle);
                }
            }
            None => {
                sdp = Some(cur_sdp);
                most_possible_angle = Some(angle);
            }
        }
    }

    match most_possible_angle {
        Some(value) => value,
        None => vertical_vec.len() / 2,
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        panic!("参数数目有误！");
    }

    let mut original_image = transfer::TransformableMat::default();
    original_image
        .load_mat(&args[1], imgcodecs::IMREAD_COLOR)
        .expect("读取图片时发生错误");

    let gray_image =
        transfer::transfer_rgb_image_to_gray_image(&original_image).expect("RGB图转灰度图失败");

    let thresh_image = transfer::transfer_gray_image_to_thresh_binary(&gray_image)
        .expect("灰度图二值化阈值处理失败");

    let mut standard_deviations = (vec![], vec![]);
    for deg in -45..45 {
        let rotated_image = transfer::rotate_mat(
            &thresh_image,
            deg as f64,
            1.0,
            imgproc::WARP_INVERSE_MAP,
            core::BORDER_CONSTANT,
            Scalar::new(255.0, 255.0, 255.0, 0.0), // b g r
        )
        .expect("旋转图像时发生错误！");

        let projection_standard_deviations =
            transfer::get_projection_standard_deviations(&rotated_image)
                .expect("计算投影标准差时发生错误");

        standard_deviations.0.push(projection_standard_deviations.0);
        standard_deviations.1.push(projection_standard_deviations.1);
    }

    let target_angle = get_most_possible_angle(standard_deviations) as f64 - 45.0;

    let final_image = transfer::rotate_mat(
        &original_image,
        target_angle,
        1.0,
        imgproc::WARP_INVERSE_MAP,
        core::BORDER_CONSTANT,
        Scalar::new(255.0, 255.0, 255.0, 0.0),
    )
    .expect("旋转图像时发生错误");

    final_image
        .im_write("C:/Users/10563/Desktop/result.jpg", ImageFormat::JPEG, 100)
        .expect("图像输出失败");
}
