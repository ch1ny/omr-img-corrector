use std::collections::HashMap;

use opencv::{
    core::{Point2f, Scalar, Size2f, Size2i, CV_PI},
    highgui, imgcodecs,
    imgproc::{self, get_rotation_matrix_2d, warp_affine},
    prelude::{Mat, MatTrait, MatTraitConst, MatTraitConstManual, MatTraitManual},
    types::VectorOfi32,
};

use crate::{
    calculate,
    types::{ImageFormat, RotateClipStrategy},
};

pub struct TransformableMatrix {
    matrix: Mat,
}
impl ToOwned for TransformableMatrix {
    fn to_owned(&self) -> Self::Owned {
        Self {
            matrix: self.matrix.to_owned(),
        }
    }

    type Owned = TransformableMatrix;
}

impl TransformableMatrix {
    /// 生成默认 Mat 对象
    pub fn default() -> Self {
        Self {
            matrix: Mat::default(),
        }
    }

    /// 根据文件路径加载新的 Mat
    pub fn load_mat(
        self: &mut Self,
        filename: &str,
        flags: i32,
    ) -> Result<&mut Self, opencv::Error> {
        let image = imgcodecs::imread(filename, flags)?;
        self.matrix = image;

        Ok(self)
    }

    /// getter mat
    #[allow(dead_code)]
    pub fn get_mat(self: &Self) -> &Mat {
        &self.matrix
    }

    pub fn from_matrix(mat: &Mat) -> Self {
        Self {
            matrix: mat.clone(),
        }
    }

    pub fn new(filename: &str, flags: i32) -> Result<Self, opencv::Error> {
        let matrix = imgcodecs::imread(filename, flags)?;
        Ok(Self { matrix })
    }

    pub fn resize_self(self: &mut Self, scale: f64) -> Result<&mut Self, opencv::Error> {
        if scale == 1.0 {
            return Ok(self);
        }

        let mut dst = Mat::default();
        let size = self.matrix.size()?;
        imgproc::resize(
            &self.matrix,
            &mut dst,
            Size2i::new(
                ((size.width as f64) * scale) as i32,
                ((size.height as f64) * scale) as i32,
            ),
            scale,
            scale,
            if scale > 1.0 {
                imgproc::INTER_LINEAR
            } else {
                imgproc::INTER_AREA
            },
        )?;
        self.matrix = dst;

        Ok(self)
    }

    /// 利用 opencv::highgui 窗口展示图片
    pub fn show(self: &Self, win_name: &str) -> Result<(), opencv::Error> {
        highgui::imshow(win_name, &self.matrix)?;
        Ok(())
    }

    /// 获取 mat 字节数组
    #[allow(dead_code)]
    pub fn get_bytes(self: &Self) -> Result<&[u8], opencv::Error> {
        self.matrix.data_bytes()
    }

    /// 将图像自身输出到指定位置
    #[allow(dead_code)]
    pub fn im_write(
        self: &Self,
        filename: &str,
        format: ImageFormat,
        quality: i32,
    ) -> Result<bool, opencv::Error> {
        let mat = &self.matrix;

        let quality_vec = match format {
            ImageFormat::JPEG => VectorOfi32::from(vec![imgcodecs::IMWRITE_JPEG_QUALITY, quality]),
            ImageFormat::PNG => {
                VectorOfi32::from(vec![imgcodecs::IMWRITE_PNG_COMPRESSION, quality])
            }
            ImageFormat::WEBP => VectorOfi32::from(vec![imgcodecs::IMWRITE_WEBP_QUALITY, quality]),
        };

        return imgcodecs::imwrite(filename, mat, &quality_vec);
    }

    pub fn clone(&self) -> Self {
        Self {
            matrix: self.matrix.clone(),
        }
    }

    /// 图像膨胀处理
    /// `kernel_shape`: 图形处理核形状参数,
    /// `kernel_size`: 图形处理核尺寸,
    /// `anchor`: 膨胀处理锚点,
    /// `iterations`: 迭代次数
    ///
    /// 用例
    /// ```rust
    /// # use oics::transfer::TransformableMatrix;
    /// # use opencv::{imgcodecs, imgproc};
    ///
    /// let src = TransformableMatrix::new("01234.jpg", imgcodecs::IMREAD_GRAYSCALE).unwrap();
    /// let dilated = src.dilate(
    ///     imgproc::MORPH_ELLIPSE,
    ///     opencv::core::Size::new(3, 3),
    ///     opencv::core::Point::new(-1, -1),
    ///     3
    /// ).unwrap();
    /// ```
    pub fn dilate(
        &self,
        kernel_shape: i32,
        kernel_size: opencv::core::Size,
        anchor: opencv::core::Point,
        iterations: i32,
    ) -> opencv::Result<Self> {
        let mat = &self.matrix;
        let mut dilated = Mat::default();

        // let kernel = imgproc::get_structuring_element(
        //     imgproc::MORPH_ELLIPSE,
        //     opencv::core::Size::new(3, 3),
        //     opencv::core::Point::new(-1, -1),
        // )?;
        let kernel = imgproc::get_structuring_element(kernel_shape, kernel_size, anchor)?;
        imgproc::dilate(
            &mat,
            &mut dilated,
            &kernel,
            anchor,
            iterations,
            opencv::core::BORDER_CONSTANT,
            imgproc::morphology_default_border_value()?,
        )?;

        Ok(Self { matrix: dilated })
    }

    /// 图像腐蚀处理
    /// `kernel_shape`: 图形处理核形状参数,
    /// `kernel_size`: 图形处理核尺寸,
    /// `anchor`: 腐蚀处理锚点,
    /// `iterations`: 迭代次数
    ///
    /// 用例
    /// ```rust
    /// # use oics::transfer::TransformableMatrix;
    /// # use opencv::{imgcodecs, imgproc};
    ///
    /// let src = TransformableMatrix::new("01234.jpg", imgcodecs::IMREAD_GRAYSCALE).unwrap();
    /// let eroded = src.erode(
    ///     imgproc::MORPH_ELLIPSE,
    ///     opencv::core::Size::new(3, 3),
    ///     opencv::core::Point::new(-1, -1),
    ///     3
    /// ).unwrap();
    /// ```
    pub fn erode(
        &self,
        kernel_shape: i32,
        kernel_size: opencv::core::Size,
        anchor: opencv::core::Point,
        iterations: i32,
    ) -> opencv::Result<Self> {
        let mat = &self.matrix;
        let mut eroded = Mat::default();

        let kernel = imgproc::get_structuring_element(kernel_shape, kernel_size, anchor)?;
        imgproc::erode(
            &mat,
            &mut eroded,
            &kernel,
            anchor,
            iterations,
            opencv::core::BORDER_CONSTANT,
            imgproc::morphology_default_border_value()?,
        )?;

        Ok(Self { matrix: eroded })
    }
}

unsafe impl Sync for TransformableMatrix {}

/// 将RGB图片转换成灰度图
#[allow(dead_code)]
pub fn transfer_rgb_image_to_gray_image(
    src: &TransformableMatrix,
) -> Result<TransformableMatrix, opencv::Error> {
    let mut dst = Mat::default();
    imgproc::cvt_color(&src.matrix, &mut dst, imgproc::COLOR_RGB2GRAY, 0)?;

    Ok(TransformableMatrix::from_matrix(&dst))
}

/// 将灰度图转换成黑白二值图
#[allow(dead_code)]
pub fn transfer_gray_image_to_thresh_binary(
    src: &TransformableMatrix,
) -> Result<TransformableMatrix, opencv::Error> {
    let mut dst = Mat::default();
    imgproc::threshold(&src.matrix, &mut dst, 127.0, 255.0, imgproc::THRESH_BINARY)?;

    Ok(TransformableMatrix::from_matrix(&dst))
}

/// 提取黑白二值图的横向投影数据
#[allow(dead_code)]
pub fn get_horizontal_projection(src: &TransformableMatrix) -> Result<Vec<f64>, opencv::Error> {
    let mat = &src.matrix;

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

    Ok(result)
}

/// 将黑白二值图转换为横向投影图
#[allow(dead_code)]
pub fn transfer_thresh_binary_to_horizontal_projection(
    src: &TransformableMatrix,
) -> Result<TransformableMatrix, opencv::Error> {
    // 克隆原图作为目标图片
    let mut mat = (&src.matrix).clone();

    // 遍历每一行
    for row_index in 0..mat.rows() {
        // 获取当前行的数据数组
        let row = mat.at_row_mut::<u8>(row_index)?;
        // 需要填色的色块次序
        let mut filled_index = 0;
        // 是否需要填色
        // 只有当出现了白色色块时才将此值修改为 true
        let mut flag = false;

        // 遍历当前行的每一个色块
        for col_index in 0..src.matrix.cols() {
            // 如果为白色色块
            // 将 flag 置为 true
            // 不做其他处理
            if row[col_index as usize] == 255 {
                flag = true;
                continue;
            }

            // 如果为黑色色块
            // 且 flag 为 true
            // 则将本色块置为白色
            // 并将需要涂抹的色块置为黑色
            if flag {
                row[col_index as usize] = 255;
                row[filled_index] = 0;
            }
            filled_index += 1;
        }
    }

    Ok(TransformableMatrix::from_matrix(&mat))
}

/// 提取黑白二值图的纵向投影数据
#[allow(dead_code)]
pub fn get_vertical_projection(src: &TransformableMatrix) -> Result<Vec<f64>, opencv::Error> {
    let mat: &Mat = &src.matrix;

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

    Ok(result)
}

/// 将黑白二值图转换为纵向投影图
#[allow(dead_code)]
pub fn transfer_thresh_binary_to_vertical_projection(
    src: &TransformableMatrix,
) -> Result<TransformableMatrix, opencv::Error> {
    // 克隆原图作为目标图片
    let mut mat = (&src.matrix).clone();

    // 记录每一列中的黑点个数及对应的列数
    let mut col_black_counts: HashMap<i32, Vec<usize>> = HashMap::new();

    let (width, height) = (mat.cols(), mat.rows());

    // 遍历每一列
    for col_index in 0..width {
        let mut sum: i32 = 0;

        // 遍历每一行
        for row_index in 0..height {
            // 获取指定位置的色块数值
            let target = mat.at_row_mut::<u8>(row_index)?[col_index as usize];
            if target <= 127 {
                sum += 1;
            }
            // 将对应色块覆盖为白色
            mat.at_row_mut::<u8>(row_index)?[col_index as usize] = 255;
        }

        // 寻找对应黑点数的所有列并插入
        // 如果没有对应的 hash 值则新建一条记录并插入
        let same_cols = col_black_counts.entry(sum).or_insert(vec![]);
        same_cols.push(col_index as usize);
    }

    // 遍历 HashMap
    for (counts, columns) in col_black_counts.iter() {
        // 对指定高度的列进行遍历
        for col_index in columns.iter() {
            // 从指定高度向图片底部遍历每一行
            for row_index in (height - counts)..height {
                let row = mat.at_row_mut::<u8>(row_index)?;
                // 将每一行的色块涂黑
                row[*col_index] = 0;
            }
        }
    }

    Ok(TransformableMatrix::from_matrix(&mat))
}

/// 旋转视图
#[allow(dead_code)]
pub fn rotate_mat(
    src: &TransformableMatrix,
    angle: f64,
    scale: f64,
    flags: i32,
    border_mode: i32,
    border_value: Scalar,
    clip_strategy: RotateClipStrategy,
) -> Result<TransformableMatrix, opencv::Error> {
    let mat = &src.matrix;
    let mut dst = Mat::default();

    match clip_strategy {
        RotateClipStrategy::DEFAULT => {
            let size = mat.size()?;
            let center_point = Point2f::new((size.width as f32) / 2.0, (size.height as f32) / 2.0);
            let rotate_matrix = get_rotation_matrix_2d(center_point, angle, scale)?;

            warp_affine(
                mat,
                &mut dst,
                &rotate_matrix,
                size,
                flags,
                border_mode,
                border_value,
            )?;
        }
        RotateClipStrategy::CONTAIN => {
            // 计算旋转后的图像尺寸
            let rotated_width = ((mat.rows() as f64) * (angle * CV_PI / 180.0).sin().abs()
                + (mat.cols() as f64) * (angle * CV_PI / 180.0).cos().abs())
            .ceil();
            let rotated_height = ((mat.cols() as f64) * (angle * CV_PI / 180.0).sin().abs()
                + (mat.rows() as f64) * (angle * CV_PI / 180.0).cos().abs())
            .ceil();

            // 计算仿射变换矩阵
            let center_point = Point2f::from_size(Size2f::new(
                (rotated_width / 2.0).ceil() as f32,
                (rotated_height / 2.0).ceil() as f32,
            ));
            let mut rotate_matrix = get_rotation_matrix_2d(center_point, angle, scale)?;

            // 防止切边，对平移矩阵进行修改
            let element = rotate_matrix.at_2d_mut::<f64>(0, 2)?;
            *element += ((rotated_width - mat.cols() as f64) / 2.0).ceil();
            let element = rotate_matrix.at_2d_mut::<f64>(1, 2)?;
            *element += ((rotated_height - mat.rows() as f64) / 2.0).ceil();

            // 应用仿射变换
            warp_affine(
                &mat,
                &mut dst,
                &rotate_matrix,
                Size2i::new(rotated_width as i32, rotated_height as i32),
                flags,
                border_mode,
                border_value,
            )?;
        }
    }

    Ok(TransformableMatrix::from_matrix(&dst))
}

/// 获取投影曲线的垂直标准差和水平标准差
#[allow(dead_code)]
pub fn get_projection_standard_deviations(
    src: &TransformableMatrix,
) -> Result<(f64, f64), opencv::Error> {
    let vertical_projection = &self::get_vertical_projection(src)?;
    let vertical_standard_deviation = calculate::get_standard_deviation(vertical_projection);
    let horizontal_projection = &self::get_horizontal_projection(src)?;
    let horizontal_standard_deviation = calculate::get_standard_deviation(horizontal_projection);

    Ok((vertical_standard_deviation, horizontal_standard_deviation))
}
