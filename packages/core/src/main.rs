use opencv::{highgui, imgcodecs};

fn main() {
    println!("Hello, world!");
    let img = imgcodecs::imread(
        "C:/Users/10563/Desktop/tomori_nao.png",
        imgcodecs::IMREAD_COLOR,
    )
    .expect("读取图片时发生错误");

    highgui::imshow("winname", &img).expect("展示图片窗口时发生错误");
    highgui::wait_key(0).expect("等待函数出错");
}
