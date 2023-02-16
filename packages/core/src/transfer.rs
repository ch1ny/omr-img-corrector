use std::collections::HashMap;

use opencv::{
    highgui, imgcodecs, imgproc,
    prelude::{Mat, MatTraitConst, MatTraitConstManual, MatTraitManual},
};

pub struct MyMat {
    mat: Mat,
}
impl MyMat {
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
}

/// 将RGB图片转换成灰度图
#[allow(dead_code)]
pub fn transfer_rgb_image_to_gray_image(src: &MyMat) -> Result<MyMat, opencv::Error> {
    let mut dst = Mat::default();
    imgproc::cvt_color(&src.mat, &mut dst, imgproc::COLOR_RGB2GRAY, 0)?;

    Ok(MyMat::new(dst))
}

/// 将灰度图转换成黑白二值图
#[allow(dead_code)]
pub fn transfer_gray_image_to_thresh_binary(src: &MyMat) -> Result<MyMat, opencv::Error> {
    let mut dst = Mat::default();
    imgproc::threshold(&src.mat, &mut dst, 127.0, 255.0, imgproc::THRESH_BINARY)?;

    Ok(MyMat::new(dst))
}

/// 提取黑白二值图的横向投影数据
#[allow(dead_code)]
pub fn get_horizontal_projection(src: &MyMat) -> Result<Vec<usize>, opencv::Error> {
    let mat = &src.mat;

    let mut result = vec![];

    // 遍历每一行
    for row_index in 0..mat.rows() {
        // 获取当前行的数据数组
        let row = mat.at_row::<u8>(row_index)?;

        // 当前行黑色的色块总数
        let mut sum: usize = 0;

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

        result.push(sum);
    }

    Ok(result)
}

/// 将黑白二值图转换为横向投影图
#[allow(dead_code)]
pub fn transfer_thresh_binary_to_horizontal_projection(
    src: &MyMat,
) -> Result<MyMat, opencv::Error> {
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

    Ok(MyMat::new(mat))
}

/// 提取黑白二值图的纵向投影数据
#[allow(dead_code)]
pub fn get_vertical_projection(src: &MyMat) -> Result<Vec<usize>, opencv::Error> {
    let mat = &src.mat;

    let mut result = vec![];

    // 遍历每一列
    for col_index in 0..mat.cols() {
        let mut sum: usize = 0;

        // 遍历每一行
        for row_index in 0..mat.rows() {
            // 获取指定位置的色块数值
            // 如果为黑色则加一
            if mat.at_row::<u8>(row_index)?[col_index as usize] == 0 {
                sum += 1;
            }
        }

        result.push(sum);
    }

    Ok(result)
}

/// 将黑白二值图转换为纵向投影图
#[allow(dead_code)]
pub fn transfer_thresh_binary_to_vertical_projection(src: &MyMat) -> Result<MyMat, opencv::Error> {
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

    Ok(MyMat::new(mat))
}
