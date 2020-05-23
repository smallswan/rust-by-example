use std::collections::BTreeSet;
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

use std::str::Chars;

#[derive(Debug)]
struct SensitiveWordMap {
    word: char,
    is_end: char,
    word_map: Option<HashMap<char, Box<SensitiveWordMap>>>,
}

/// 递归地修改map
fn r_map(map: &mut SensitiveWordMap, chars: &mut Chars) {
    if let Some(ch) = chars.next() {
        if let Some(now_map) = map.word_map.as_mut() {
            let contains_key = now_map.contains_key(&ch);
            println!("ch:{},contains_key:{:?}", ch, contains_key);

            if contains_key {
                if let Some(m) = now_map.get_mut(&ch) {
                    r_map(&mut *m, &mut *chars);
                }
            } else {
                let mut swm = SensitiveWordMap {
                    word: ch,
                    is_end: '0',
                    word_map: Some(HashMap::<char, Box<SensitiveWordMap>>::new()),
                };
                now_map.insert(ch, Box::new(swm));
                if let Some(m) = now_map.get_mut(&ch) {
                    r_map(&mut *m, &mut *chars);
                }
            }
        }
    }
    println!("{:?}", map);
}

/// 读取敏感词库，将敏感词放入HashSet中，构建一个DFA算法模型
///
fn build_sensitive_word_map(set: BTreeSet<String>) -> HashMap<char, SensitiveWordMap> {
    let mut sensitive_word_map = HashMap::<char, SensitiveWordMap>::new();

    let mut iterator = set.iter();
    for key in iterator {
        let len = key.chars().count();
        let mut key_chars = key.chars();
        //读取每行的首个字符
        if let Some(first_char) = key_chars.next() {
            if let Some(word_map) = sensitive_word_map.get_mut(&first_char) {
                println!("first_char1：{}", first_char);
                //读取下一个字符
                r_map(&mut *word_map, &mut key_chars);
            } else {
                println!("first_char2：{}", first_char);

                let mut now_map = SensitiveWordMap {
                    word: first_char,
                    is_end: '0',
                    word_map: Some(HashMap::<char, Box<SensitiveWordMap>>::new()),
                };
                sensitive_word_map.insert(first_char, now_map);

                if let Some(now_map) = sensitive_word_map.get_mut(&first_char) {
                    r_map(&mut *now_map, &mut key_chars);
                }
            }
        }
        println!("sensitive_word_map-----{:?}", sensitive_word_map);
    }

    sensitive_word_map
}

/// 读取敏感词库中的内容，将内容添加到set集合中
fn read_sensitive_word_file() -> BTreeSet<String> {
    let mut set = BTreeSet::<String>::new();
    match File::open("sensitive-words.txt") {
        Ok(f) => {
            let reader = BufReader::new(f);
            let lines = reader.lines();
            for line in lines.map(|x| x.unwrap()) {
                println!("{}", line);

                set.insert(line);
            }
        }
        Err(e) => panic!("can't open this file :{}", e),
    }

    set
}

#[test]
fn read_file() {
    let set = read_sensitive_word_file();
    let sensitive_word_map = build_sensitive_word_map(set);
    println!("last sensitive_word_map-----{:?}", sensitive_word_map);
}
