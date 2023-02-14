use opencv::{highgui, imgcodecs};
use std::env;

mod transfer;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        panic!("参数数目有误！");
    }

    let image = imgcodecs::imread(&args[1], imgcodecs::IMREAD_COLOR).expect("读取图片时发生错误");
    highgui::imshow("Original_Image", &image).expect("展示图片窗口时发生错误"); // 原图

    let gray_image = transfer::transfer_rgb_image_to_gray_image(&image);
    highgui::imshow("Gray_Image", &gray_image).expect("展示图片窗口时发生错误"); // 灰度图

    let thresh_image = transfer::transfer_gray_image_to_thresh_binary(&gray_image);
    highgui::imshow("Thresh_Binary_Image", &thresh_image).expect("展示图片窗口时发生错误"); // 黑白二值图

    highgui::wait_key(0).expect("等待函数出错");
}
