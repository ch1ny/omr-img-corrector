use opencv::{imgproc, prelude::Mat};

/**
 * 将图片转换成灰度图
 */
pub fn transfer_to_gray_image(src: &Mat) -> Mat {
    let mut dst = Mat::default();
    imgproc::cvt_color(src, &mut dst, imgproc::COLOR_RGB2GRAY, 0).expect("灰度转化失败");
    return dst;
}
