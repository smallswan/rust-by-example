use image::GenericImageView;
use image::ImageFormat;
use std::env;
use std::{fs::File, time::Instant};
fn main() {
    let (from, into) = if env::args_os().count() == 3 {
        (
            env::args_os().nth(1).unwrap(),
            env::args_os().nth(2).unwrap(),
        )
    } else {
        println!("Please enter a from and into path.");
        std::process::exit(1);
    };

    // 裁剪图片：宽度不变，高度为原来的一半（保留上半部分）
    let mut img = image::open(from).unwrap();
    let (width, height) = img.dimensions();
    let timer = Instant::now();
    let scaled = img.crop(0, 0, width, height / 2);

    let mut output = File::create(into).unwrap();
    scaled.write_to(&mut output, ImageFormat::Png).unwrap();

    // scaled.save(&Path::new(&into)).unwrap();

    println!("{:?}", timer.elapsed());
}
