use opencv::{imgproc, prelude::Mat};

/**
 * 将RGB图片转换成灰度图
 */
pub fn transfer_rgb_image_to_gray_image(src: &Mat) -> Mat {
    let mut dst = Mat::default();
    imgproc::cvt_color(src, &mut dst, imgproc::COLOR_RGB2GRAY, 0).expect("RGB图转灰度图失败");
    return dst;
}

/**
 * 将灰度图转换成黑白二值图
 */
pub fn transfer_gray_image_to_thresh_binary(src: &Mat) -> Mat {
    let mut dst = Mat::default();
    imgproc::threshold(src, &mut dst, 127.0, 255.0, imgproc::THRESH_BINARY)
        .expect("灰度图二值化阈值处理失败");
    return dst;
}
