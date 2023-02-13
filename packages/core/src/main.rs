use opencv::{highgui, imgcodecs};
use std::env;

mod transfer;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        panic!("参数数目有误！");
    }

    let img = imgcodecs::imread(&args[1], imgcodecs::IMREAD_COLOR).expect("读取图片时发生错误");
    let gray_image = transfer::transfer_to_gray_image(&img);

    highgui::imshow("gray", &gray_image).expect("展示图片窗口时发生错误");
    highgui::wait_key(0).expect("等待函数出错");
}
