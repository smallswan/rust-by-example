use image::imageops::FilterType;
use image::GenericImageView;
use image::ImageFormat;
use std::env;
use std::fs::File;
use std::time::Instant;

fn main() {
    // let (from, into) = if env::args_os().count() == 3 {
    //     (
    //         env::args_os().nth(1).unwrap(),
    //         env::args_os().nth(2).unwrap(),
    //     )
    // } else {
    //     println!("Please enter a from and into path.");
    //     std::process::exit(1);
    // };

    // // 裁剪图片：宽度不变，高度为原来的一半（保留上半部分）
    // let mut img = image::open(from).unwrap();
    // let (width, height) = img.dimensions();
    // let timer = Instant::now();
    // let scaled = img.crop(0, 0, width, height / 2);

    // let mut output = File::create(into).unwrap();
    // scaled.write_to(&mut output, ImageFormat::Png).unwrap();

    // println!("{:?}", timer.elapsed());

    let img = image::open("examples/scaledown/bridge.png").unwrap();
    let timer = Instant::now();
    let (width, height) = img.dimensions();
    let scaled = img.thumbnail(width / 2, height / 2);

    //webp (just decode)
    // jpg转png 大小会大好几倍
    let mut output = File::create("examples/scaledown/bridge-thumbnail.jpg").unwrap();
    scaled.write_to(&mut output, ImageFormat::Jpeg).unwrap();

    println!("Scaled by in {:?}", timer.elapsed());
}
