use crate::transfer::TransformableMatrix;
use opencv::{
    core::{Point, Point2f, Scalar},
    imgcodecs,
    imgproc::{self, canny, cvt_color, hough_lines_p, line},
    prelude::Mat,
    types::{VectorOfVec4f, VectorOfi32},
};

/// ### 利用霍夫变换查找偏转角
///
/// **参数列表：**
/// - `gray_tm`: 包含灰度图的 `TransformableMatrix`
/// - `min_line_length`: 感知的最小线段长度
/// - `max_line_gap`: 感知的线段最大中断长度
///
pub fn get_angle_with_hough(
    gray_tm: &TransformableMatrix,
    min_line_length: f64,
    max_line_gap: f64,
    file_name: &str,
) -> Result<f64, opencv::Error> {
    let mat = gray_tm.get_mat();

    let mut edges = Mat::default();
    canny(mat, &mut edges, 50.0, 150.0, 3, false)?;

    // 霍夫变换
    let mut lines = VectorOfVec4f::default();
    hough_lines_p(
        &edges,
        &mut lines,
        1.0,
        std::f64::consts::PI / 180.0,
        // 指定阈值
        0,
        // 检测的直线最小长度
        min_line_length,
        // 检测直线之间的最大间隙
        max_line_gap,
    )?;

    // 直线图
    let mut lined_img = Mat::default();
    cvt_color(&edges, &mut lined_img, imgproc::COLOR_GRAY2BGR, 0)?;

    // 获取直线的斜率
    let mut angles = vec![];
    for l in lines.iter() {
        let pt1 = Point2f::new(l[0], l[1]);
        let pt2 = Point2f::new(l[2], l[3]);

        // 画线
        line(
            &mut lined_img,
            Point::new(l[0] as i32, l[1] as i32),
            Point::new(l[2] as i32, l[3] as i32),
            Scalar::new(186.0, 88.0, 255.0, 0.0),
            1,
            imgproc::LINE_AA,
            0,
        )?;

        let mut angle = (pt2.y - pt1.y).atan2(pt2.x - pt1.x) * 180.0 / std::f32::consts::PI;
        // 限制偏转角度在 -45deg ~ +45deg 之间
        if angle < -45.0 {
            angle = angle + 90.0;
        } else if angle > 45.0 {
            angle = angle - 90.0;
        }
        angles.push(angle);
    }

    // 找到最常出现的斜率作为图像的旋转角度
    let range = 0.1;
    // 目标角度
    let mut target_angle = angles[0];
    // 处于目标角度的直线数目
    let mut target_angle_lines_count = 0;
    // 找寻目标角度线段总长度最大的角度作为最终的偏转角度
    for i in 0..angles.len() {
        let mut count = 0;
        for j in 0..angles.len() {
            if (angles[i] - angles[j]).abs() < range {
                count += 1;
            }
        }
        if count > target_angle_lines_count {
            target_angle = angles[i];
            target_angle_lines_count = count;
        }
    }

    imgcodecs::imwrite(
        &(String::from("C:/Users/10563/Desktop/result/edges/") + file_name),
        &lined_img,
        &VectorOfi32::from(vec![imgcodecs::IMWRITE_JPEG_QUALITY, 100]),
    )
    .unwrap();

    // 返回旋转角度 target_angle
    Ok(target_angle as f64)
}
