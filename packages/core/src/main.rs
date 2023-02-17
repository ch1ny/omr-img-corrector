use oics::{ highgui, imgcodecs, transfer };
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        panic!("参数数目有误！");
    }

    let mut image = transfer::TransformableMat::default();
    image
        .load_mat(&args[1], imgcodecs::IMREAD_UNCHANGED)
        .expect("读取图片时发生错误");
    image
        .show("Original_Image")
        .expect("展示图片窗口时发生错误"); // 原图

    let gray_image = transfer::transfer_rgb_image_to_gray_image(&image).expect("RGB图转灰度图失败");
    gray_image
        .show("Gray_Image")
        .expect("展示图片窗口时发生错误"); // 灰度图

    let thresh_image = transfer::transfer_gray_image_to_thresh_binary(&gray_image)
        .expect("灰度图二值化阈值处理失败");
    thresh_image
        .show("Thresh_Binary_Image")
        .expect("展示图片窗口时发生错误"); // 黑白二值图

    let hp = transfer::transfer_thresh_binary_to_horizontal_projection(&thresh_image)
        .expect("计算纵向投影发生错误");
    hp.show("Horizontal_Projection")
        .expect("展示图片窗口时发生错误");

    let vp = transfer::transfer_thresh_binary_to_vertical_projection(&thresh_image)
        .expect("计算横向投影发生错误");
    vp.show("Vertical_Projection")
        .expect("展示图片窗口时发生错误");

    highgui::wait_key(0).expect("等待函数出错");
}
