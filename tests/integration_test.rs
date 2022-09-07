extern crate rust_by_example;

#[test]
fn sensitive_word() {
    assert_eq!(rust_by_example::add(3, 2), 5);

    let timestamp = rust_by_example::time_gen();
    println!("now timestamp : {:?}", timestamp);

    //交换x,y的值
    let (mut x, mut y) = (254, 128);
    x ^= y;
    y ^= x;
    x ^= y;
    println!("now x ,y: {:?},{:?}", x, y);
    assert_eq!(254, y);
    assert_eq!(128, x);

    let machine_kind = if cfg!(unix) {
        "unix"
    } else if cfg!(windows) {
        "windows"
    } else {
        "unknown"
    };

    println!("I'm running on a {} machine!", machine_kind);
}

#[test]
fn rusty_book() {
    // 浮点数数组可以使用 Vec::sort_by 和 PartialOrd::partial_cmp 进行排序。
    let mut vec = vec![1.1, 1.15, 5.5, 1.123, 2.0, 3.14, 0.618];

    vec.sort_by(|a, b| a.partial_cmp(b).unwrap());

    assert_eq!(vec, vec![0.618, 1.1, 1.123, 1.15, 2.0, 3.14, 5.5]);
}

use image::GenericImageView;
use image::ImageFormat;
use std::{fs::File, time::Instant};
// use image::dynimage::DynamicImage;

#[test]
fn half_height() {
    let mut img = image::open("examples/scaledown/借条.jpeg").unwrap();
    let (width, height) = img.dimensions();
    let timer = Instant::now();
    let scaled = img.crop(0, 0, width, height / 2);
    // println!("Scaled by {} in {}", name, Elapsed::from(&timer));
    let mut output =
        File::create(&format!("examples/scaledown/借条-{}.jpeg", "half-height")).unwrap();
    scaled.write_to(&mut output, ImageFormat::Png).unwrap();
    println!("{:?}", timer.elapsed());
}
