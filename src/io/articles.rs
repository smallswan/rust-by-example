use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::collections::VecDeque;
use std::fs::{self, DirEntry};
use std::io;
use std::path::Path;

// one possible implementation of walking a directory only visiting files
fn visit_dirs(dir: &Path, cb: &dyn Fn(&DirEntry)) -> io::Result<()> {
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                visit_dirs(&path, cb)?;
            } else {
                cb(&entry);
            }
        }
    } else {
        println!("{}", dir.display());
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn all_files() {
        let path = Path::new(".");
        println!("{:?}", path.metadata());
    }

    /// 遍历目录下文件
    #[test]
    fn famous() -> io::Result<()> {
        let mut entries = fs::read_dir("examples/file")?
            .map(|res| res.map(|e| e.path()))
            .collect::<Result<Vec<_>, io::Error>>()?;

        // The order in which `read_dir` returns entries is not guaranteed. If reproducible
        // ordering is required the entries should be explicitly sorted.

        entries.sort();

        entries.iter().for_each(|entry| {
            println!(
                "{:?} is file ? {}",
                entry.file_name().unwrap(),
                entry.is_file()
            )
        });

        Ok(())
    }

    use std::collections::hash_map::Entry;
    use std::collections::HashMap;
    use std::fs::File;
    use std::io::prelude::*;
    use std::io::BufReader;
    #[test]
    fn come_on() {
        //统计中文字数
        let mut words: HashMap<String, i32> = HashMap::with_capacity(300);
        match File::open("examples\\file\\寒窑赋.txt") {
            Ok(f) => {
                let reader = BufReader::new(f);
                let lines = reader.lines();
                for line in lines.map(|x| x.unwrap()) {
                    println!("{}", line);

                    line.chars().for_each(|c| {
                        if c.is_alphabetic() {
                            match words.entry(c.to_string()) {
                                Entry::Occupied(entry) => *entry.into_mut() += 1,
                                Entry::Vacant(entry) => {
                                    *entry.insert(1);
                                }
                            }
                        }
                    });
                }
            }
            Err(e) => println!("{}", e),
        }

        println!("----------------");

        //根据字数多少从高到低排序
        let mut sorted: Vec<(String, i32)> = words
            .iter()
            .map(|(key, value)| (key.to_string(), *value))
            .collect();
        sorted.sort_by_key(|pair| pair.1);

        sorted.iter().rev().for_each(|(key, value)| {
            println!("{} {}", key, value);
        });
    }

    #[test]
    fn top_words() {
        // 1. 统计各个单词出现的频率
        let mut words: HashMap<String, i32> = HashMap::with_capacity(300);
        match File::open("why-rust.txt") {
            Ok(f) => {
                let reader = BufReader::new(f);
                let lines = reader.lines();
                for line in lines.map(|x| x.unwrap()) {
                    line.split_whitespace().for_each(|word| {
                        // 需要处理标点符号
                        if let Some(last_char) = word.chars().rev().nth(0) {
                            if last_char.is_ascii_punctuation() {
                                let new_word = &word[..word.len() - 1];

                                match words.entry(new_word.to_string()) {
                                    Entry::Occupied(entry) => *entry.into_mut() += 1,
                                    Entry::Vacant(entry) => {
                                        *entry.insert(1);
                                    }
                                }
                            } else {
                                match words.entry(word.to_string()) {
                                    Entry::Occupied(entry) => *entry.into_mut() += 1,
                                    Entry::Vacant(entry) => {
                                        *entry.insert(1);
                                    }
                                }
                            }
                        }
                    });
                }
            }
            Err(e) => println!("{}", e),
        }

        // 2. 排序
        let mut rank: Vec<(String, i32)> = words.into_iter().collect();
        rank.sort_by_key(|pair| pair.1);
        // 3. 取top3
        rank.iter().rev().take(3).for_each(|(key, value)| {
            println!("{}={}", key, value);
        })
    }

    use memmap::Mmap;
    use std::io::{Error, Write};

    /// https://rusty.rs/files/read-write.html
    #[test]
    fn mmap() -> Result<(), Error> {
        write!(
            File::create("content.txt")?,
            "My hovercraft is full of eels!"
        )?;

        let file = File::open("content.txt")?;
        let map = unsafe { Mmap::map(&file)? };

        let random_indexes = [0, 1, 2, 19, 22, 10, 11, 29];
        assert_eq!(&map[3..13], b"hovercraft");
        let random_bytes: Vec<u8> = random_indexes.iter().map(|&idx| map[idx]).collect();
        assert_eq!(&random_bytes[..], b"My loaf!");
        Ok(())
    }

    #[test]
    fn mmap_mut() -> Result<(), Error> {
        use memmap::MmapMut;
        use std::fs::OpenOptions;
        use std::io::Write;
        use std::ops::DerefMut;
        use std::path::PathBuf;

        // let file = File::options().write(true).open("abcd.txt")?;

        let path: PathBuf = PathBuf::from("abcd.txt");
        let file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(&path)?;
        file.set_len(30)?;

        let mut mmap = unsafe { MmapMut::map_mut(&file)? };

        // mmap.copy_from_slice(b"Hello, world!");
        (&mut mmap[..]).write_all(b"Hello BeiJing! Hello, world!")?;
        mmap.flush()?;

        Ok(())
    }
}
