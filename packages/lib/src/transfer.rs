use std::collections::HashMap;

use opencv::{
    core::{Point2f, Scalar},
    highgui, imgcodecs,
    imgproc::{self, get_rotation_matrix_2d, warp_affine},
    prelude::{Mat, MatTraitConst, MatTraitConstManual, MatTraitManual},
    types::VectorOfi32,
};

use crate::{calculate, types::ImageFormat};

pub struct TransformableMat {
    mat: Mat,
}
impl TransformableMat {
    /// 生成默认 Mat 对象
    pub fn default() -> Self {
        Self {
            mat: Mat::default(),
        }
    }

    /// 根据文件路径加载新的 Mat
    pub fn load_mat(
        self: &mut Self,
        filename: &str,
        flags: i32,
    ) -> Result<&mut Self, opencv::Error> {
        let image = imgcodecs::imread(filename, flags)?;
        self.mat = image;

        Ok(self)
    }

    /// getter mat
    #[allow(dead_code)]
    pub fn get_mat(self: &Self) -> &Mat {
        &self.mat
    }

    pub fn new(mat: Mat) -> Self {
        Self { mat }
    }

    /// 利用 opencv::highgui 窗口展示图片
    pub fn show(self: &Self, win_name: &str) -> Result<(), opencv::Error> {
        highgui::imshow(win_name, &self.mat)?;
        Ok(())
    }

    /// 获取 mat 字节数组
    #[allow(dead_code)]
    pub fn get_bytes(self: &Self) -> Result<&[u8], opencv::Error> {
        self.mat.data_bytes()
    }

    /// 将图像自身输出到指定位置
    #[allow(dead_code)]
    pub fn im_write(
        self: &Self,
        filename: &str,
        format: ImageFormat,
        quality: i32,
    ) -> Result<bool, opencv::Error> {
        let mat = &self.mat;

        let mut quality_vec = VectorOfi32::with_capacity(2);
        match format {
            ImageFormat::JPEG => {
                quality_vec.push(imgcodecs::IMWRITE_JPEG_QUALITY);
                quality_vec.push(quality);
            }
            ImageFormat::PNG => {
                quality_vec.push(imgcodecs::IMWRITE_PNG_COMPRESSION);
                quality_vec.push(quality);
            }
            ImageFormat::WEBP => {
                quality_vec.push(imgcodecs::IMWRITE_WEBP_QUALITY);
                quality_vec.push(quality);
            }
        };

        return imgcodecs::imwrite(filename, mat, &quality_vec);
    }
}

/// 将RGB图片转换成灰度图
#[allow(dead_code)]
pub fn transfer_rgb_image_to_gray_image(
    src: &TransformableMat,
) -> Result<TransformableMat, opencv::Error> {
    let mut dst = Mat::default();
    imgproc::cvt_color(&src.mat, &mut dst, imgproc::COLOR_RGB2GRAY, 0)?;

    Ok(TransformableMat::new(dst))
}

/// 将灰度图转换成黑白二值图
#[allow(dead_code)]
pub fn transfer_gray_image_to_thresh_binary(
    src: &TransformableMat,
) -> Result<TransformableMat, opencv::Error> {
    let mut dst = Mat::default();
    imgproc::threshold(&src.mat, &mut dst, 127.0, 255.0, imgproc::THRESH_BINARY)?;

    Ok(TransformableMat::new(dst))
}

/// 提取黑白二值图的横向投影数据
#[allow(dead_code)]
pub fn get_horizontal_projection(src: &TransformableMat) -> Result<Vec<f64>, opencv::Error> {
    let mat = &src.mat;

    let mut result: Vec<f64> = vec![];

    // 遍历每一行
    for row_index in 0..mat.rows() {
        // 获取当前行的数据数组
        let row = mat.at_row::<u8>(row_index)?;

        // 当前行黑色的色块总数
        let mut sum = 0;

        // 遍历当前行的每一个色块
        for col_index in 0..src.mat.cols() {
            // 如果为白色色块
            // 不做处理
            if row[col_index as usize] == 255 {
                continue;
            }

            // 如果为黑色色块
            // 总数加一
            sum += 1;
        }

        result.push(sum as f64);
    }

    Ok(result)
}

/// 将黑白二值图转换为横向投影图
#[allow(dead_code)]
pub fn transfer_thresh_binary_to_horizontal_projection(
    src: &TransformableMat,
) -> Result<TransformableMat, opencv::Error> {
    // 克隆原图作为目标图片
    let mut mat = (&src.mat).clone();

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
        for col_index in 0..src.mat.cols() {
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

    Ok(TransformableMat::new(mat))
}

/// 提取黑白二值图的纵向投影数据
#[allow(dead_code)]
pub fn get_vertical_projection(src: &TransformableMat) -> Result<Vec<f64>, opencv::Error> {
    let mat = &src.mat;

    let mut result = vec![0.0; mat.cols() as usize];

    // 遍历每一行
    for row_index in 0..mat.rows() {
        // 获取当前行的数据数组
        let row = mat.at_row::<u8>(row_index)?;

        // 遍历当前行的每一个色块
        for col_index in 0..src.mat.cols() {
            // 如果为白色色块
            // 不做处理
            if row[col_index as usize] == 255 {
                continue;
            }

            // 如果为黑色色块
            // 总数加一
            result[col_index as usize] += 1.0;
        }
    }

    Ok(result)
}

/// 将黑白二值图转换为纵向投影图
#[allow(dead_code)]
pub fn transfer_thresh_binary_to_vertical_projection(
    src: &TransformableMat,
) -> Result<TransformableMat, opencv::Error> {
    // 克隆原图作为目标图片
    let mut mat = (&src.mat).clone();

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
            if target == 0 {
                sum += 1;
                // 将对应色块覆盖为白色
                mat.at_row_mut::<u8>(row_index)?[col_index as usize] = 255;
            }
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

    Ok(TransformableMat::new(mat))
}

/// 旋转视图
#[allow(dead_code)]
pub fn rotate_mat(
    src: &TransformableMat,
    angle: f64,
    scale: f64,
    flags: i32,
    border_mode: i32,
    border_value: Scalar,
) -> Result<TransformableMat, opencv::Error> {
    let mat = &src.mat;

    let size = mat.size()?;

    let mut center_pos = Point2f::default();
    center_pos.x = (size.width as f32) / 2.0;
    center_pos.y = (size.height as f32) / 2.0;

    let rotate_matrix = get_rotation_matrix_2d(center_pos, angle, scale)?;

    let mut dst = Mat::default();

    warp_affine(
        mat,
        &mut dst,
        &rotate_matrix,
        size,
        flags,
        border_mode,
        border_value,
    )?;

    Ok(TransformableMat::new(dst))
}

/// 获取投影曲线的垂直标准差和水平标准差
#[allow(dead_code)]
pub fn get_projection_standard_deviations(
    src: &TransformableMat,
) -> Result<(f64, f64), opencv::Error> {
    let vertical_standard_deviation =
        calculate::get_standard_deviation(&self::get_vertical_projection(src)?);
    let horizontal_standard_deviation =
        calculate::get_standard_deviation(&self::get_horizontal_projection(src)?);

    Ok((vertical_standard_deviation, horizontal_standard_deviation))
}
