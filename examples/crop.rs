use image::GenericImageView;
use image::ImageFormat;
// use std::cmp::Reverse;
// use std::collections::BinaryHeap;
// use std::collections::VecDeque;
use std::fs::{self, DirEntry};
use std::{fs::File, time::Instant};

use std::io;
use std::path::Path;

fn main() -> io::Result<()> {
    let entries = fs::read_dir("C:\\data")?
        .map(|res| res.map(|e| e.path()))
        .collect::<Result<Vec<_>, io::Error>>()?;

    entries.iter().for_each(|entry| {
        if let Some(file) = entry.file_name() {
            if file.to_str().unwrap().ends_with(".jpg") {
                let file_name = format!("C:\\data\\{}", file.to_str().unwrap());
                println!("{:?}", file_name);
                // let from = Path::new(file_name.as_str());

                // 裁剪图片：宽度不变，高度为原来的一半（保留上半部分）
                let mut img = image::open(file_name).unwrap();
                let (width, height) = img.dimensions();
                let timer = Instant::now();
                let scaled = img.crop(0, 0, width, height / 2);

                let file_name = format!("{}", entry.to_str().unwrap());
                println!("{:?}", file_name);
                // let into = Path::new(file_name.as_str());
                let mut output = File::create(&file_name).unwrap();
                scaled.write_to(&mut output, ImageFormat::Png).unwrap();
                println!("{} {:?}", &file_name, timer.elapsed());
            }
        }
    });

    Ok(())
}
