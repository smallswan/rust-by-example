use std::fs::{self, File};
#[cfg(target_os = "linux")]
use std::os::linux::fs::MetadataExt;
#[cfg(target_os = "windows")]
use std::os::windows::prelude::MetadataExt;

use zstd::stream;
fn main() -> std::io::Result<()> {
    // 压缩
    // why-rust.txt
    let source = File::open("C:\\data\\movies.json")?;
    let destination = File::create("C:\\data\\movies.zst")?;
    match stream::copy_encode(&source, &destination, 7) {
        Ok(_) => {
            let metadata1 = source.metadata()?;
            let metadata2 = destination.metadata()?;
            let size = metadata2.file_size();
            println!("compress success: {} => {}", metadata1.file_size(), size);

            if let Ok(metadata) = fs::metadata("C:\\data\\movies.zst") {
                println!(
                    "{:?},{},{:?}",
                    metadata.file_type(),
                    metadata.len(),
                    metadata.created().unwrap()
                );
                assert_eq!(size, metadata.len());
            }
        }
        Err(e) => println!("{}", e),
    }

    // 拆包
    let destination = File::open("why-rust.zst")?;
    let bytes = stream::decode_all(destination).unwrap();
    println!("{}", String::from_utf8_lossy(&bytes));

    Ok(())
}

use std::io::Read;
use tar::Archive;
use tar::Builder;

#[test]
fn archive_encode() {
    // 打包
    let file = File::create("examples/file/foo.tar").unwrap();
    let mut a = Builder::new(file);
    a.append_path("why-rust.txt").unwrap();
    a.append_path("sensitive-words.txt").unwrap();
    a.append_path("large_file.txt").unwrap();

    // 压缩
    let source = File::open("examples/file/foo.tar").unwrap();
    let destination = File::create("examples/file/foo.tar.zst").unwrap();
    match stream::copy_encode(&source, &destination, 7) {
        Ok(_) => {
            let metadata1 = source.metadata().unwrap();
            if let Ok(metadata2) = destination.metadata() {
                let size = metadata2.file_size();
                println!("compress success: {} => {}", metadata1.file_size(), size);
            }

            if let Ok(metadata) = fs::metadata("examples/file/foo.tar.zst") {
                println!(
                    "{:?},{},{:?}",
                    metadata.file_type(),
                    metadata.len(),
                    metadata.created().unwrap()
                );
            }
        }
        Err(e) => println!("copy_encode : {}", e),
    }
}

#[test]
fn decode_unpackage() {
    // 解压zst文件
    if let Ok(source) = File::open("examples/file/github_users_sample_set.tar.zst") {
        if let Ok(destination) = File::create("examples/file/github_users_sample_set.tar") {
            stream::copy_decode(source, destination);
        }
    }

    // 拆包tar文件
    let file = File::open("examples/file/github_users_sample_set.tar").unwrap();
    let mut a = Archive::new(file);

    for file in a.entries().unwrap() {
        // Make sure there wasn't an I/O error
        let mut file = file.unwrap();

        // Inspect metadata about the file
        println!("{:?}", file.header().path().unwrap());
        println!("{}", file.header().size().unwrap());

        // files implement the Read trait
        let mut s = String::new();
        file.read_to_string(&mut s).unwrap();
        println!("{}", s);
    }
}
