use opencv::{
    core::{Mat, Point2f, Scalar, Size2f, Size2i, CV_PI},
    imgcodecs, imgproc,
    prelude::{MatTrait, MatTraitConst, MatTraitConstManual},
    types::{VectorOfVec4f, VectorOfi32},
};

fn get_mat_horizontal_standard_deviation(mat: &Mat) -> opencv::Result<f64> {
    let mut result: Vec<f64> = Vec::with_capacity(mat.rows() as usize);
    // 遍历每一行
    for row_index in 0..mat.rows() {
        // 获取当前行的数据数组
        let row = mat.at_row::<u8>(row_index)?;

        // 当前行黑色的色块总数
        let mut sum = 0;

        // 遍历当前行的每一个色块
        for item in row {
            // 如果为黑色色块
            // 总数加一
            // 如果为白色色块
            // 不做处理
            if *item == 0 {
                sum += 1;
            }
        }

        result.push(sum as f64);
    }
    Ok(crate::calculate::get_standard_deviation(&result))
}

fn get_mat_vertical_standard_deviation(mat: &Mat) -> opencv::Result<f64> {
    let mut result = vec![0.0; mat.cols() as usize];

    // 遍历每一行
    for row_index in 0..mat.rows() {
        // 获取当前行的数据数组
        let row = mat.at_row::<u8>(row_index)?;

        let mut col_index = 0;
        // 遍历当前行的每一个色块
        row.iter().for_each(|item| {
            // 如果为白色色块
            // 不做处理
            // 如果为黑色色块
            // 总数加一
            if *item == 0 {
                result[col_index] += 1.0;
            }
            col_index += 1;
        });
    }
    Ok(crate::calculate::get_standard_deviation(&result))
}

enum ProjectionResultStatus {
    Believed,
    NeedCheck,
    NotAResult,
}

pub fn correct_default(
    input_file: &str,
    output_file: &str,
    projection_max_angle: u16,
    projection_angle_step: f64,
    projection_resize_scale: f64,
    hough_min_line_length: f64,
    hough_max_line_gap: f64,
) -> opencv::Result<(f64, bool)> {
    let src_mat = imgcodecs::imread(input_file, imgcodecs::IMREAD_COLOR)?;

    // 找出旋转角度以及是否需要复查
    let (rotate_angle, need_check) = {
        // 先使用基本的投影标准差方法进行纠偏
        let (projection_angle, projection_result_status) = {
            let scaled_mat = {
                let gray_mat = {
                    let mut dst_mat = Mat::default();
                    imgproc::cvt_color(&src_mat, &mut dst_mat, imgproc::COLOR_RGB2GRAY, 0)?;
                    dst_mat
                };

                // 先对输入图像进行腐蚀预处理以提升图像锐度
                let mut eroded = Mat::default();
                let kernel = imgproc::get_structuring_element(
                    imgproc::MORPH_ELLIPSE,
                    opencv::core::Size::new(3, 3),
                    opencv::core::Point::new(-1, -1),
                )?;
                imgproc::erode(
                    &gray_mat,
                    &mut eroded,
                    &kernel,
                    opencv::core::Point::new(-1, -1),
                    3,
                    opencv::core::BORDER_CONSTANT,
                    imgproc::morphology_default_border_value()?,
                )?;

                let mut scaled = Mat::default();
                let size = eroded.size()?;
                imgproc::resize(
                    &eroded,
                    &mut scaled,
                    Size2i::new(
                        ((size.width as f64) * projection_resize_scale) as i32,
                        ((size.height as f64) * projection_resize_scale) as i32,
                    ),
                    projection_resize_scale,
                    projection_resize_scale,
                    if projection_resize_scale > 1.0 {
                        imgproc::INTER_LINEAR
                    } else {
                        imgproc::INTER_AREA
                    },
                )?;
                scaled
            };
            let thresh_binary_mat = {
                let mut dst_mat = Mat::default();
                imgproc::threshold(
                    &scaled_mat,
                    &mut dst_mat,
                    127.0,
                    255.0,
                    imgproc::THRESH_BINARY,
                )?;
                dst_mat
            };
            let projection_range_max_angle =
                (projection_max_angle as f64 / projection_angle_step) as u16;
            let projection_range = {
                let min_angle = -(projection_range_max_angle as i32);
                min_angle..(projection_range_max_angle as i32)
            };

            {
                let mut max_horizontal_standard_deviation = 0.0;
                let mut max_vertical_standard_deviation = 0.0;
                let mut possible_horizontal_counts = 1u32;
                let mut possible_vertical_counts = 1u32;
                let mut most_possible_deg_vec: Vec<i32> = vec![];

                for deg in projection_range {
                    // let rotated_image = rotate_mat(
                    //     &thresh_image,
                    //     deg as f64 * step,
                    //     1.0,
                    //     imgproc::WARP_POLAR_LINEAR,
                    //     opencv::core::BORDER_CONSTANT,
                    //     Scalar::new(255.0, 255.0, 255.0, 0.0), // b g r
                    //     RotateClipStrategy::DEFAULT,
                    // )
                    let rotated_mat = {
                        let mut dst = Mat::default();
                        let size = thresh_binary_mat.size()?;
                        let center_point =
                            Point2f::new((size.width as f32) / 2.0, (size.height as f32) / 2.0);
                        let rotate_matrix = imgproc::get_rotation_matrix_2d(
                            center_point,
                            deg as f64 * projection_angle_step,
                            projection_resize_scale,
                        )?;

                        imgproc::warp_affine(
                            &thresh_binary_mat,
                            &mut dst,
                            &rotate_matrix,
                            size,
                            imgproc::WARP_POLAR_LINEAR,
                            opencv::core::BORDER_CONSTANT,
                            Scalar::new(255.0, 255.0, 255.0, 0.0), // b g r
                        )?;
                        dst
                    };

                    // 先获取水平投影标准差
                    let horizontal_projection_standard_deviation =
                        get_mat_horizontal_standard_deviation(&rotated_mat)?;
                    if max_horizontal_standard_deviation < horizontal_projection_standard_deviation
                    {
                        max_horizontal_standard_deviation =
                            horizontal_projection_standard_deviation;
                        // 再获取垂直投影标准差
                        max_vertical_standard_deviation =
                            get_mat_vertical_standard_deviation(&rotated_mat)?;
                        possible_horizontal_counts = 1;
                        possible_vertical_counts = 1;
                        most_possible_deg_vec = vec![deg];
                    } else if max_horizontal_standard_deviation
                        == horizontal_projection_standard_deviation
                    {
                        possible_horizontal_counts += 1;
                        let vertical_projection_standard_deviation =
                            get_mat_vertical_standard_deviation(&rotated_mat)?;
                        if max_vertical_standard_deviation < vertical_projection_standard_deviation
                        {
                            possible_vertical_counts = 1;
                            max_vertical_standard_deviation =
                                vertical_projection_standard_deviation;
                            most_possible_deg_vec = vec![deg];
                        } else if max_vertical_standard_deviation
                            == vertical_projection_standard_deviation
                        {
                            possible_vertical_counts += 1;
                            most_possible_deg_vec.push(deg);
                        }
                    }
                }

                if possible_horizontal_counts == 1 && possible_vertical_counts == 1 {
                    let target_angle = most_possible_deg_vec[0] as f64 * projection_angle_step;
                    (target_angle, ProjectionResultStatus::Believed)
                } else if most_possible_deg_vec.len() == 1 {
                    (
                        most_possible_deg_vec[0] as f64 * projection_angle_step,
                        ProjectionResultStatus::NeedCheck,
                    )
                } else {
                    (0.0, ProjectionResultStatus::NotAResult)
                }
            }
        };

        match projection_result_status {
            ProjectionResultStatus::Believed => (projection_angle, false),
            _ => {
                // 投影标准差方案不确定，方案降级至霍夫变化进行比对
                {
                    // 边缘检测
                    let edges = {
                        let mut dst = Mat::default();
                        imgproc::canny(&src_mat, &mut dst, 50.0, 150.0, 3, false)?;
                        dst
                    };
                    // 霍夫概率变换
                    let lines = {
                        let mut dst = VectorOfVec4f::default();
                        imgproc::hough_lines_p(
                            &edges,
                            &mut dst,
                            1.0,
                            std::f64::consts::PI / 180.0,
                            0,
                            hough_min_line_length,
                            hough_max_line_gap,
                        )?;
                        dst
                    };

                    // 获取直线的斜率
                    let mut angles = vec![];
                    for l in lines.iter() {
                        let pt1 = Point2f::new(l[0], l[1]);
                        let pt2 = Point2f::new(l[2], l[3]);

                        let mut angle =
                            (pt2.y - pt1.y).atan2(pt2.x - pt1.x) * 180.0 / std::f32::consts::PI;
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

                    // 返回旋转角度 target_angle
                    match projection_result_status {
                        ProjectionResultStatus::NeedCheck => (target_angle as f64, true),
                        ProjectionResultStatus::NotAResult => (target_angle as f64, true),
                        _ => unreachable!(),
                    }
                }
            }
        }
    };

    // 输出图像
    {
        // 旋转图像
        let rotated_mat = {
            let mut dst = Mat::default();
            // 计算旋转后的图像尺寸
            let rotated_width = ((src_mat.rows() as f64)
                * (rotate_angle * CV_PI / 180.0).sin().abs()
                + (src_mat.cols() as f64) * (rotate_angle * CV_PI / 180.0).cos().abs())
            .ceil();
            let rotated_height = ((src_mat.cols() as f64)
                * (rotate_angle * CV_PI / 180.0).sin().abs()
                + (src_mat.rows() as f64) * (rotate_angle * CV_PI / 180.0).cos().abs())
            .ceil();

            // 计算仿射变换矩阵
            let center_point = Point2f::from_size(Size2f::new(
                (rotated_width / 2.0).ceil() as f32,
                (rotated_height / 2.0).ceil() as f32,
            ));
            let mut rotate_matrix =
                imgproc::get_rotation_matrix_2d(center_point, rotate_angle, 1.0)?;

            // 防止切边，对平移矩阵进行修改
            let element = rotate_matrix.at_2d_mut::<f64>(0, 2)?;
            *element += ((rotated_width - src_mat.cols() as f64) / 2.0).ceil();
            let element = rotate_matrix.at_2d_mut::<f64>(1, 2)?;
            *element += ((rotated_height - src_mat.rows() as f64) / 2.0).ceil();

            // 应用仿射变换
            imgproc::warp_affine(
                &src_mat,
                &mut dst,
                &rotate_matrix,
                Size2i::new(rotated_width as i32, rotated_height as i32),
                imgproc::WARP_POLAR_LINEAR,
                opencv::core::BORDER_CONSTANT,
                Scalar::new(255.0, 255.0, 255.0, 0.0), // b g r
            )?;
            dst
        };

        let quality_vec = VectorOfi32::from(vec![imgcodecs::IMWRITE_JPEG_QUALITY, 100]);
        imgcodecs::imwrite(output_file, &rotated_mat, &quality_vec)?;
    };

    Ok((rotate_angle, need_check))
}
