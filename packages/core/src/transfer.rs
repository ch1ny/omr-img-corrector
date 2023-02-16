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
pub fn transfer_rgb_image_to_gray_image(src: &MyMat) -> Result<MyMat, opencv::Error> {
    let mut dst = Mat::default();
    imgproc::cvt_color(&src.mat, &mut dst, imgproc::COLOR_RGB2GRAY, 0)?;

    Ok(MyMat::new(dst))
}

/// 将灰度图转换成黑白二值图
pub fn transfer_gray_image_to_thresh_binary(src: &MyMat) -> Result<MyMat, opencv::Error> {
    let mut dst = Mat::default();
    imgproc::threshold(&src.mat, &mut dst, 127.0, 255.0, imgproc::THRESH_BINARY)?;

    Ok(MyMat::new(dst))
}

/// 将黑白二值图转换为横向投影图
pub fn transfer_thresh_binary_to_horizontal_projection(
    src: &MyMat,
) -> Result<MyMat, opencv::Error> {
    // 克隆原图作为目标图片
    let mut mat = (&src.mat).clone();

    // 遍历每一行
    for col_index in 0..mat.rows() {
        // 获取当前行的数据数组
        let row = mat.at_row_mut::<u8>(col_index)?;
        // 需要填色的色块次序
        let mut filled_index = 0;
        // 是否需要填色
        // 只有当出现了白色色块时才将此值修改为 true
        let mut flag = false;

        // 遍历当前行的每一个色块
        for row_index in 0..src.mat.cols() {
            // 如果为白色色块
            // 将 flag 置为 true
            // 不做其他处理
            if row[row_index as usize] == 255 {
                flag = true;
                continue;
            }

            // 如果为黑色色块
            // 且 flag 为 true
            // 则将本色块置为白色
            // 并将需要涂抹的色块置为黑色
            if flag {
                row[row_index as usize] = 255;
                row[filled_index] = 0;
            }
            filled_index += 1;
        }
    }

    Ok(MyMat::new(mat))
}
