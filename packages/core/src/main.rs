use oics::{
    core::{self, Scalar},
    imgcodecs, imgproc, transfer,
};
use std::{env, time::Instant};

fn main() {
    let start_time = Instant::now();

    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        panic!("参数数目有误！");
    }

    let time_0 = start_time.elapsed().as_millis();

    let mut original_image = transfer::TransformableMat::default();
    original_image
        .load_mat(&args[1], imgcodecs::IMREAD_COLOR)
        .expect("读取图片时发生错误");

    let time_1 = start_time.elapsed().as_millis();

    let gray_image =
        transfer::transfer_rgb_image_to_gray_image(&original_image).expect("RGB图转灰度图失败");

    let time_2 = start_time.elapsed().as_millis();

    let thresh_image = transfer::transfer_gray_image_to_thresh_binary(&gray_image)
        .expect("灰度图二值化阈值处理失败");

    let time_3 = start_time.elapsed().as_millis();

    let mut results = vec![];
    let mut times = (vec![], vec![]);
    for deg in -45..45 {
        let dr = Instant::now();

        let rotated_image = transfer::rotate_mat(
            &thresh_image,
            deg as f64,
            imgproc::WARP_INVERSE_MAP,
            core::BORDER_CONSTANT,
            Scalar::new(255.0, 255.0, 255.0, 0.0), // b g r
        )
        .expect("旋转图像时发生错误！");

        let md = dr.elapsed().as_millis();

        let projection_standard_deviations =
            transfer::get_projection_standard_deviations(&rotated_image)
                .expect("计算投影标准差时发生错误");

        times.1.push(dr.elapsed().as_millis() - md);
        times.0.push(md);
        results.push(projection_standard_deviations);
    }

    println!("{:?}", results);

    println!("读取参数 => {}", time_0);
    println!("读取图片 => {}", time_1 - time_0);
    println!("转灰度图 => {}", time_2 - time_1);
    println!("二值化处理 => {}", time_3 - time_2);

    println!("旋转图像 => {:?}", times.0);
    println!("计算标准差 => {:?}", times.1);

    println!("总耗时 => {}", start_time.elapsed().as_millis());
}
