use std::fs::File;
use std::os::windows::prelude::MetadataExt;
use zstd::stream;
fn main() -> std::io::Result<()> {
    // why-rust.txt
    let source = File::open("C:\\data\\movies.json")?;
    let destination = File::create("C:\\data\\movies.zst")?;
    match stream::copy_encode(&source, &destination, 3) {
        Ok(_) => {
            let metadata1 = source.metadata()?;
            let metadata2 = destination.metadata()?;
            println!(
                "compress success: {} => {}",
                metadata1.file_size(),
                metadata2.file_size()
            )
        }
        Err(e) => println!("{}", e),
    }

    let destination = File::open("why-rust.zst")?;
    let bytes = stream::decode_all(destination).unwrap();
    println!("{}", String::from_utf8_lossy(&bytes));
    Ok(())
}
