// cSpell: disable
use opencv::{
    core::{
        dft, log, magnitude, merge, min_max_loc, no_array, split, Mat, Point, Rect, Scalar, Vec4i,
        CV_32F, CV_8UC1, DFT_COMPLEX_INPUT, DFT_COMPLEX_OUTPUT, DFT_SCALE,
    },
    imgcodecs,
    imgproc::{self, canny, cvt_color, hough_lines_p},
    prelude::{MatExprTraitConst, MatTraitConst, MatTraitConstManual},
    types::{VectorOfMat, VectorOfi32},
};
use std::f64::consts::PI;

use crate::transfer::TransformableMatrix;

fn new_mat() -> Mat {
    Mat::zeros(0, 0, CV_32F).unwrap().to_mat().unwrap()
}

fn mul_image(image: &Mat, mul: f64) -> opencv::Result<Mat> {
    let mut clone: Mat = image.clone();
    image.convert_to(&mut clone, CV_32F, mul, 0.0)?;
    Ok(clone)
}

fn mul_add_image(image: &Mat, mul: f64, add: f64) -> opencv::Result<Mat> {
    let mut clone = image.clone();
    image.convert_to(&mut clone, CV_32F, mul, add)?;
    Ok(clone)
}

pub fn rev(filter: &Mat) -> opencv::Result<Mat> {
    mul_add_image(&mul_add_image(filter, -1.0, 0.0)?, 1.0, 1.0)
}

fn log_image(image: &Mat) -> opencv::Result<Mat> {
    let mut clone = image.clone();
    log(&image, &mut clone)?;
    Ok(clone)
}

fn fft_complex(image: &Mat) -> opencv::Result<(Mat, Mat)> {
    let vec_of_mat = VectorOfMat::from(vec![
        image.clone(),
        Mat::zeros(image.rows(), image.cols(), CV_32F)?.to_mat()?,
    ]);
    let mut image_complex = new_mat();
    merge(&vec_of_mat, &mut image_complex)?;

    let mut image_dft = image.clone();
    dft(
        &image_complex,
        &mut image_dft,
        DFT_COMPLEX_OUTPUT | DFT_COMPLEX_INPUT | DFT_SCALE,
        0,
    )?;

    let mut vec_of_mat = VectorOfMat::new();
    split(&image_dft, &mut vec_of_mat)?;

    Ok((
        fft_shift(&vec_of_mat.get(0)?)?,
        fft_shift(&vec_of_mat.get(1)?)?,
    ))
}

fn fft_shift(image: &Mat) -> opencv::Result<Mat> {
    let clone = image.clone();
    let cx = image.cols() / 2;
    let cy = image.rows() / 2;
    let mut q0 = Mat::roi(&clone, Rect::new(0, 0, cx, cy))?;
    let mut q1 = Mat::roi(&clone, Rect::new(cx, 0, cx, cy))?;
    let mut q2 = Mat::roi(&clone, Rect::new(0, cy, cx, cy))?;
    let mut q3 = Mat::roi(&clone, Rect::new(cx, cy, cx, cy))?;

    let mut tmp = q0.clone();
    q0.copy_to(&mut tmp)?;
    q3.copy_to(&mut q0)?;
    tmp.copy_to(&mut q3)?;

    q1.copy_to(&mut tmp)?;
    q2.copy_to(&mut q1)?;
    tmp.copy_to(&mut q2)?;

    Ok(clone)
}

fn correction(image: &Mat) -> opencv::Result<Mat> {
    let mut min = 0.0;
    let mut max = 0.0;
    min_max_loc(
        image,
        Some(&mut min),
        Some(&mut max),
        None,
        None,
        &no_array(),
    )?;
    // println!("{}, {}", min, max);
    let mut clone = image.clone();
    image.convert_to(&mut clone, CV_32F, 1.0, -min)?;

    let mut clone2 = clone.clone();
    clone.convert_to(&mut clone2, CV_32F, 1.0 / (max - min), 0.0)?;
    Ok(clone2)
}

fn fft_magnitude(fft: &(Mat, Mat)) -> opencv::Result<Mat> {
    let mut image_magnitude = new_mat();
    magnitude(&fft.0, &fft.1, &mut image_magnitude)?;
    let image: Mat = (correction(&image_magnitude) as opencv::Result<Mat>)?;
    let image: Mat = mul_image(&image, 255.0)?;
    Ok(image)
}

fn fft_magnitude_log(fft: &(Mat, Mat)) -> opencv::Result<Mat> {
    let image_magnitude = fft_magnitude(&fft)? as Mat;
    let image = mul_add_image(&image_magnitude, 1.0, 1.0 / 255.0)? as Mat;
    let image = &log_image(&image)? as &Mat;
    let image = correction(&image)?;
    Ok(image)
}

pub fn get_fft_image(gray_tm: &TransformableMatrix) -> opencv::Result<(Mat, Mat)> {
    let image_file = {
        let gray_tm_mat = gray_tm.get_mat();
        let mut clone = gray_tm_mat.clone();
        gray_tm_mat.convert_to(&mut clone, CV_32F, 1.0 / 255.0, 0.0)?;
        clone
    };

    let fft: (Mat, Mat) = fft_complex(&image_file)?;

    let mut magnitude_image = Mat::default();
    fft_magnitude(&fft)?.convert_to(&mut magnitude_image, CV_8UC1, 255.0, 0.0)?;

    let mut magnitude_log_image = Mat::default();
    fft_magnitude_log(&fft)?.convert_to(&mut magnitude_log_image, CV_8UC1, 255.0, 0.0)?;

    Ok((magnitude_image, magnitude_log_image))
}

/// ### 利用傅里叶变换查找偏转角
///
pub fn get_angle_with_fft(
    // img_src: &str,
    gray_tm: &TransformableMatrix,
    canny_threshold_1: f64,
    canny_threshold_2: f64,
    min_line_length: f64,
    max_line_gap: f64,
    file_name: &str,
    edge_image_output_dir: &str,
) -> Result<f64, opencv::Error> {
    let fft_image = {
        #[allow(unused_variables)]
        let (magnitude_image, magnitude_log_image) = get_fft_image(gray_tm).unwrap();
        magnitude_log_image
    };

    let mut edges = Mat::default();

    // 使用 canny 边缘检测算法检测图像边缘
    canny(
        &fft_image,
        &mut edges,
        canny_threshold_1,
        canny_threshold_2,
        3,
        false,
    )?;

    // 直线图
    let mut lined_img = Mat::default();
    cvt_color(&edges, &mut lined_img, imgproc::COLOR_GRAY2BGR, 0)?;

    // 在边缘图像中检测直线
    let mut lines = Mat::default();
    let rho = 1.0;
    let theta = PI / 180.0;
    let threshold = 100;

    hough_lines_p(
        &edges,
        &mut lines,
        rho,
        theta,
        threshold,
        min_line_length,
        max_line_gap,
    )?;

    // 计算所有直线的斜率，并选择斜率最接近垂直方向的直线

    let mut average_angle = 0.0;
    let mut max_votes = 0;
    for i in 0..lines.rows() {
        let line = lines.at_row::<Vec4i>(i)?[0];
        let x1 = line[0] as f64;
        let y1 = line[1] as f64;
        let x2 = line[2] as f64;
        let y2 = line[3] as f64;

        // 画线
        imgproc::line(
            &mut lined_img,
            Point::new(line[0], line[1]),
            Point::new(line[2], line[3]),
            Scalar::new(186.0, 88.0, 255.0, 0.0),
            1,
            imgproc::LINE_AA,
            0,
        )?;

        let angle = {
            let counted_angle = ((y2 - y1).atan2(x2 - x1) * 180.0) as f64 / PI;
            if counted_angle < -45.0 {
                counted_angle + 90.0
            } else if counted_angle > 45.0 {
                counted_angle - 90.0
            } else {
                counted_angle
            }
        };
        // 计算数据分布概率密度（投票）
        let mut votes = 0;
        for j in 0..lines.rows() {
            if i == j {
                continue;
            }
            let line_j = lines.at_row::<Vec4i>(i)?[0];
            let x1_j = line_j[0] as f64;
            let y1_j = line_j[1] as f64;
            let x2_j = line_j[2] as f64;
            let y2_j = line_j[3] as f64;
            let angle_j = ((y2_j - y1_j).atan2(x2_j - x1_j) * 180.0) / PI;
            let distance = (angle_j - angle).abs();
            if distance < 0.1 {
                votes += 1;
            }
        }
        if votes > max_votes {
            max_votes = votes;
            average_angle = angle;
        }
    }

    imgcodecs::imwrite(
        &(String::from(edge_image_output_dir) + file_name),
        &lined_img,
        &VectorOfi32::from(vec![imgcodecs::IMWRITE_JPEG_QUALITY, 100]),
    )
    .unwrap();

    Ok(average_angle)
}
