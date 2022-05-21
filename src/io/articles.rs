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
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn famous() -> io::Result<()> {
        let mut entries = fs::read_dir("C:\\data")?
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
        let mut words: HashMap<String, i32> = HashMap::with_capacity(300);
        match File::open("C:\\data\\寒窑赋.txt") {
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

        let mut sorted: Vec<(String, i32)> = words
            .iter()
            .map(|(key, value)| (key.to_string(), *value))
            .collect();
        sorted.sort_by_key(|pair| pair.1);

        sorted.iter().rev().for_each(|(key, value)| {
            println!("{} {}", key, value);
        });
    }
}
