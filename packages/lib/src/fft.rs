// cSpell: disable
use opencv::{
    core::Vec4i,
    imgproc::{canny, cvt_color, hough_lines_p, COLOR_BGR2GRAY},
    prelude::{Mat, MatTraitConst, MatTraitConstManual},
};
use std::f64::consts::PI;

use crate::transfer::TransformableMatrix;

/// ### 利用傅里叶变换查找偏转角
///
pub fn get_angle_with_fft(gray_tm: &TransformableMatrix) -> Result<f64, opencv::Error> {
    let src = gray_tm.get_mat();

    let mut gray = Mat::default();
    let mut edges = Mat::default();

    // 将图像转为灰度图
    cvt_color(src, &mut gray, COLOR_BGR2GRAY, 0)?;

    // 使用 canny 边缘检测算法检测图像边缘
    canny(&gray, &mut edges, 100.0, 200.0, 3, false)?;

    // 在边缘图像中检测直线
    let mut lines = Mat::default();
    let rho = 1.0;
    let theta = PI / 180.0;
    let threshold = 100;

    hough_lines_p(&edges, &mut lines, rho, theta, threshold, 125.0, 5.0)?;

    // 计算所有直线的斜率，并选择斜率最接近垂直方向的直线

    let mut average_angle = 0.0;
    let mut max_votes = 0;
    for i in 0..lines.rows() {
        let line = lines.at_row::<Vec4i>(i)?[0];
        let x1 = line[0] as f64;
        let y1 = line[1] as f64;
        let x2 = line[2] as f64;
        let y2 = line[3] as f64;
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

    Ok(average_angle)
}
