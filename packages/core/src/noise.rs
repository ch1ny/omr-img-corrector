use oics::{
    core::{Mat, MatTraitConst},
    prelude::MatTraitManual,
};

use rand::Rng;
use rand_distr::{Distribution, Normal};

#[allow(dead_code)]
pub fn add_gaussian_noise(src: &Mat, mean: f64, std_dev: f64) -> Mat {
    let mut rng = rand::thread_rng();
    let normal = Normal::new(mean, std_dev).unwrap();

    // 将噪声向量转换为 Mat，与原始图像大小相同
    let mut noise_mat = src.clone();
    for i in 0..noise_mat.rows() {
        let mut_row = noise_mat.at_row_mut::<u8>(i).unwrap();
        for j in 0..src.cols() {
            // 从正态分布中生成随机噪声
            let noise_val = normal.sample(&mut rng);
            // 在每个像素处添加噪声
            let pixel = src.at_2d::<u8>(i, j).unwrap();
            let noisy_pixel = pixel.saturating_add(noise_val as u8);
            mut_row[j as usize] = noisy_pixel;
        }
    }

    return noise_mat;
}

#[allow(dead_code)]
pub fn add_salt_and_pepper_noise(src: &Mat, prob: f64) -> Mat {
    let mut rng = rand::thread_rng();

    let prob = prob % 1.0;

    // 将噪声向量转换为 Mat，与原始图像大小相同
    let mut noise_mat = src.clone();
    for i in 0..noise_mat.rows() {
        let mut_row = noise_mat.at_row_mut::<u8>(i).unwrap();
        for j in 0..src.cols() {
            let random = rng.gen_range(0.0..1.0);
            if random < prob {
                mut_row[j as usize] = if random < 0.5 { 0 } else { 255 };
            }
        }
    }

    return noise_mat;
}
