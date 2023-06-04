use opencv::{
    core::{Mat, Point2f, Scalar, Size2f, Size2i, CV_PI},
    imgcodecs, imgproc,
    prelude::{MatTrait, MatTraitConst, MatTraitConstManual},
    types::{VectorOfVec4f, VectorOfi32},
};

fn get_mat_projection_data(mat: &Mat) -> opencv::Result<(Vec<f64>, Vec<f64>)> {
    // 定义数组分别记录水平方向、垂直方向上的投影数据
    let mut horizontal_projection_data: Vec<f64> = Vec::with_capacity(mat.rows() as usize);
    let mut vertical_projection_data: Vec<f64> = vec![0.0f64; mat.cols() as usize];

    // 遍历每一行
    for row_index in 0..mat.rows() {
        // 获取当前行的数据数组
        let row = mat.at_row::<u8>(row_index)?;

        // 当前行黑色的色块总数
        let mut row_black_sum = 0;
        // 记录迭代序列
        let mut col_index = 0;

        // 遍历当前行的每一个色块，通过迭代器提高遍历速度
        row.iter().for_each(|item| {
            // 如果为白色色块
            // 不做处理
            // 如果为黑色色块
            // 总数加一
            if *item == 0 {
                row_black_sum += 1;
                vertical_projection_data[col_index] += 1.0;
            }
            col_index += 1;
        });
        horizontal_projection_data.push(row_black_sum as f64);
    }

    Ok((horizontal_projection_data, vertical_projection_data))
}

pub enum ResultStatus {
    Believed,
    NeedCheck,
    NotAResult,
}
pub struct OmrResult {
    pub angle: f64,
    pub status: ResultStatus,
    pub candidates: Vec<f64>,
}

pub fn get_result_from_projection(
    src_mat: &Mat,
    projection_max_angle: u16,
    projection_angle_step: f64,
    projection_max_width: i32,
    projection_max_height: i32,
) -> opencv::Result<OmrResult> {
    // 计算缩放比例
    let projection_resize_scale = {
        let original_size = &src_mat.size()?;
        let original_width = original_size.width;
        let original_height = original_size.height;
        let width_scale = if projection_max_width <= 0 {
            1.0
        } else {
            projection_max_width as f64 / original_width as f64
        };
        let height_scale = if projection_max_height <= 0 {
            1.0
        } else {
            projection_max_height as f64 / original_height as f64
        };

        let target_scale = if width_scale < height_scale {
            width_scale
        } else {
            height_scale
        };

        target_scale
    };

    // 找出旋转角度
    // 先使用基本的投影标准差方法进行纠偏
    let (projection_angle, projection_result_status, projection_candidate_result_vec) = {
        let scaled_mat = {
            let gray_mat = {
                let mut dst_mat = Mat::default();
                imgproc::cvt_color(&src_mat, &mut dst_mat, imgproc::COLOR_RGB2GRAY, 0)?;
                dst_mat
            };

            // 先对输入图像进行腐蚀预处理以提升图像锐度
            // 声明腐蚀操作输出图像可变
            let mut eroded = Mat::default();
            // 定义腐蚀核，为一个内嵌至长宽3像素的矩形的填充椭圆
            let kernel = imgproc::get_structuring_element(
                imgproc::MORPH_ELLIPSE,
                opencv::core::Size::new(3, 3),
                opencv::core::Point::new(-1, -1),
            )?;
            // 通过调用opencv的腐蚀操作对图像进行处理，将输出结果写入eroded的可变引用中
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
                0.0,
                0.0,
                imgproc::INTER_AREA,
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

        let mut max_horizontal_standard_deviation = 0.0;
        let mut max_vertical_standard_deviation = 0.0;
        let mut possible_horizontal_counts = 1u32;
        let mut possible_vertical_counts = 1u32;
        let mut most_possible_deg_vec: Vec<f64> = vec![];

        for deg in projection_range {
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

            // 获取水平、垂直投影数据
            let (horizontal_projection_data, vertical_projection_data) =
                get_mat_projection_data(&rotated_mat)?;

            // 先获取水平投影标准差
            let horizontal_projection_standard_deviation =
                crate::calculate::get_standard_deviation(&horizontal_projection_data);
            if max_horizontal_standard_deviation < horizontal_projection_standard_deviation {
                max_horizontal_standard_deviation = horizontal_projection_standard_deviation;
                // 再获取垂直投影标准差
                max_vertical_standard_deviation =
                    crate::calculate::get_standard_deviation(&vertical_projection_data);
                possible_horizontal_counts = 1;
                possible_vertical_counts = 1;
                most_possible_deg_vec = vec![deg as f64 * projection_angle_step];
            } else if max_horizontal_standard_deviation == horizontal_projection_standard_deviation
            {
                possible_horizontal_counts += 1;
                // 再获取垂直投影标准差
                let vertical_projection_standard_deviation =
                    crate::calculate::get_standard_deviation(&vertical_projection_data);
                if max_vertical_standard_deviation < vertical_projection_standard_deviation {
                    possible_vertical_counts = 1;
                    max_vertical_standard_deviation = vertical_projection_standard_deviation;
                    most_possible_deg_vec = vec![deg as f64 * projection_angle_step];
                } else if max_vertical_standard_deviation == vertical_projection_standard_deviation
                {
                    possible_vertical_counts += 1;
                    most_possible_deg_vec.push(deg as f64 * projection_angle_step);
                }
            }
        }

        if possible_horizontal_counts == 1 && possible_vertical_counts == 1 {
            let target_angle = most_possible_deg_vec[0];
            (target_angle, ResultStatus::Believed, most_possible_deg_vec)
        } else if most_possible_deg_vec.len() == 1 {
            (
                most_possible_deg_vec[0],
                ResultStatus::NeedCheck,
                most_possible_deg_vec,
            )
        } else {
            (0.0, ResultStatus::NotAResult, most_possible_deg_vec)
        }
    };

    Ok(OmrResult {
        angle: projection_angle,
        status: projection_result_status,
        candidates: projection_candidate_result_vec,
    })
}

pub fn get_result_from_edges_detection(
    src_mat: &Mat,
    edges_min_line_length: f64,
    edges_max_line_gap: f64,
) -> opencv::Result<OmrResult> {
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
            edges_min_line_length,
            edges_max_line_gap,
        )?;
        dst
    };

    // 获取直线的斜率
    let mut angles = vec![];
    for l in lines.iter() {
        let pt1 = Point2f::new(l[0], l[1]);
        let pt2 = Point2f::new(l[2], l[3]);

        let mut angle = (pt2.y - pt1.y).atan2(pt2.x - pt1.x) * 180.0 / std::f32::consts::PI;
        // 限制偏转角度在 -45deg ~ +45deg 之间
        angle = angle % 45.0;
        angles.push(angle as f64);
    }
    // 找到最常出现的斜率作为图像的旋转角度
    let range = 0.1;

    // 目标角度
    let mut target_angle = angles[0];
    let mut candidates_vec = vec![];
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
            candidates_vec = vec![target_angle];
        } else if count == target_angle_lines_count {
            candidates_vec.push(angles[i]);
        }
    }

    Ok(OmrResult {
        angle: target_angle as f64,
        status: match candidates_vec.len() {
            0 => ResultStatus::NotAResult,
            1 => ResultStatus::Believed,
            _ => ResultStatus::NeedCheck,
        },
        candidates: candidates_vec,
    })
}

pub fn get_result_from_fourier_transform(
    src_mat: &Mat,
    canny_threshold_weak: f64,
    canny_threshold_strong: f64,
    fourier_min_line_length: f64,
    fourier_max_line_gap: f64,
) -> opencv::Result<OmrResult> {
    let fft_image = {
        let gray_tm = {
            let mut dst = Mat::default();
            imgproc::cvt_color(&src_mat, &mut dst, imgproc::COLOR_RGB2GRAY, 0)?;
            dst
        };
        #[allow(unused_variables)]
        let (magnitude_image, magnitude_log_image) =
            crate::fft::get_fft_image(&crate::transfer::TransformableMatrix::from_matrix(&gray_tm))
                .unwrap();
        magnitude_log_image
    };

    let mut edges = Mat::default();

    // 使用 canny 边缘检测算法检测图像边缘
    imgproc::canny(
        &fft_image,
        &mut edges,
        canny_threshold_weak,
        canny_threshold_strong,
        3,
        false,
    )?;

    get_result_from_edges_detection(&edges, fourier_min_line_length, fourier_max_line_gap)
}

pub fn correct_default(
    input_file: &str,
    output_file: &str,
    projection_max_angle: u16,
    projection_angle_step: f64,
    projection_max_width: i32,
    projection_max_height: i32,
    hough_min_line_length: f64,
    hough_max_line_gap: f64,
) -> opencv::Result<(f64, bool)> {
    let src_mat = imgcodecs::imread(input_file, imgcodecs::IMREAD_COLOR)?;

    // 找出旋转角度以及是否需要复查
    let (rotate_angle, need_check) = {
        let projection_result = get_result_from_projection(
            &src_mat,
            projection_max_angle,
            projection_angle_step,
            projection_max_width,
            projection_max_height,
        )?;

        match projection_result.status {
            ResultStatus::Believed => (projection_result.angle, false),
            _ => {
                // 投影标准差方案不确定，方案降级至霍夫变化进行比对
                {
                    let edges_result = get_result_from_edges_detection(
                        &src_mat,
                        hough_min_line_length,
                        hough_max_line_gap,
                    )?;

                    // 返回旋转角度 target_angle
                    match projection_result.status {
                        ResultStatus::NeedCheck => {
                            // 当两种方案偏差超过 0.1° 时判定需要额外检查
                            if (projection_result.angle - edges_result.angle).abs() >= 0.1 {
                                (edges_result.angle, true)
                            } else {
                                (projection_result.angle, false)
                            }
                        }
                        ResultStatus::NotAResult => {
                            match projection_result.candidates.into_iter().min_by(|&a, &b| {
                                (&(a - edges_result.angle).abs())
                                    .partial_cmp(&(b - edges_result.angle).abs())
                                    .unwrap()
                            }) {
                                Some(projection_candidate) => {
                                    if (projection_candidate - edges_result.angle).abs() < 0.05 {
                                        (projection_candidate, false)
                                    } else {
                                        (edges_result.angle, true)
                                    }
                                }
                                None => (edges_result.angle, true),
                            }
                        }
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
