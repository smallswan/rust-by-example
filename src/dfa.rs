use std::collections::BTreeSet;
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

use std::str::Chars;

/// 敏感词检测DFA算法（Rust实现，参考Java版实现 https://www.cnblogs.com/shihaiming/p/7048379.html）
/// 由于语言方面的限制，具体实现与Java有一定的差异。
///
lazy_static! {
    static ref SENSITIVE_WORD_MAP: HashMap<char, SensitiveWordMap> = {
        let set = read_sensitive_word_file();
        build_sensitive_word_map(set)
    };
}

pub enum MatchType {
    MinMatchType, //最小匹配规则
    MaxMatchType, //最大匹配规则
}

#[derive(Debug)]
struct SensitiveWordMap {
    word: char,
    is_end: char,
    word_map: Option<HashMap<char, Box<SensitiveWordMap>>>,
}

/// 替换敏感字字符
/// # Examples
/// ```
/// let result = rust_by_example::dfa::replace_sensitive_word("信用卡之家", &MatchType::MinMatchType, '*')
/// assert_eq!(result,"**卡之家");
/// ```
pub fn replace_sensitive_word(txt: &str, match_type: &MatchType, replace_char: char) -> String {
    let set: BTreeSet<String> = find_sensitive_word(txt, match_type);
    let mut replace_str = String::from(txt);
    for word in set {
        let len = word.chars().count();
        let replace_chars: String = vec![replace_char; len].iter().collect();
        replace_str = replace_str.replace(word.as_str(), &replace_chars);
    }

    replace_str
}
/// 判断文字是否包含敏感字符
///
pub fn is_contains_sensitive_word(txt: &str, match_type: &MatchType) -> bool {
    let mut is_contains = false;
    let len = txt.chars().count();
    let txt_vec: Vec<char> = txt.chars().collect();
    let mut i = 0;
    while i < len {
        let length = check_sensitive_word(txt, i, match_type);
        if length > 0 {
            is_contains = true;
            break;
        }
        i += 1;
    }
    is_contains
}

/// 获取文字中的敏感词
///
pub fn find_sensitive_word(txt: &str, match_type: &MatchType) -> BTreeSet<String> {
    let mut sensitive_word_set = BTreeSet::<String>::new();
    let len = txt.chars().count();
    let txt_vec: Vec<char> = txt.chars().collect();
    let mut i = 0;
    while i < len {
        let length = check_sensitive_word(txt, i, match_type);
        if length > 0 {
            //存在,加入list中
            sensitive_word_set.insert(txt_vec[i..i + length].iter().collect());
            i += length - 1; //减1的原因，是因为循环会自增
        }
        i += 1;
    }

    sensitive_word_set
}

/// 查文字中是否包含检敏感字符,如果存在，则返回敏感词字符的长度，不存在返回0
///
fn check_sensitive_word(txt: &str, begin_index: usize, match_type: &MatchType) -> usize {
    let mut match_flag = 0;
    let mut last_match_length = 0;
    let mut word: char;
    let txt_vec: Vec<char> = txt.chars().collect();
    let len = txt.len();
    if let Some(word) = &txt_vec.get(begin_index) {
        if let Some(swm) = SENSITIVE_WORD_MAP.get(word) {
            match_flag += 1;
            if (*swm).is_end == '1' {
                last_match_length = match_flag;

                match match_type {
                    MatchType::MinMatchType => {
                        return last_match_length;
                    }
                    MatchType::MaxMatchType => (),
                }
            }

            //递归查找
            let mut j = begin_index + 1;
            recursive_find_map(
                swm,
                &txt_vec,
                &mut j,
                &mut match_flag,
                &mut last_match_length,
                match_type,
            );
        }
    }
    last_match_length
}
/// 递归查找map
///
fn recursive_find_map(
    swm: &SensitiveWordMap,
    txt_vec: &[char],
    i: &mut usize,
    match_flag: &mut usize,
    last_match_length: &mut usize,
    match_type: &MatchType,
) {
    if let Some(word) = txt_vec.get(*i) {
        if let Some(wm) = &swm.word_map {
            if let Some(next_swm) = wm.get(word) {
                *match_flag += 1;

                if swm.is_end == '1' {
                    *last_match_length = *match_flag;
                    match match_type {
                        MatchType::MinMatchType => {
                            return;
                        }
                        MatchType::MaxMatchType => (),
                    }
                }

                if next_swm.is_end == '1' {
                    *last_match_length = *match_flag;
                    match match_type {
                        MatchType::MinMatchType => {
                            return;
                        }
                        MatchType::MaxMatchType => (),
                    }
                }

                if let Some(nwm) = &next_swm.word_map {
                    if nwm.is_empty() {
                        *last_match_length = *match_flag;
                        match match_type {
                            MatchType::MinMatchType => {
                                return;
                            }
                            MatchType::MaxMatchType => (),
                        }
                    }
                }

                *i += 1;
                recursive_find_map(
                    next_swm,
                    txt_vec,
                    i,
                    match_flag,
                    last_match_length,
                    match_type,
                );
            }
        }
    }
}
/// 递归地修改map
fn recursive_build_map(map: &mut SensitiveWordMap, chars: &mut Chars, count: &mut usize) {
    if let Some(ch) = chars.next() {
        *count -= 1;
        if let Some(now_map) = map.word_map.as_mut() {
            // let contains_key = now_map.contains_key(&ch);

            if let std::collections::hash_map::Entry::Vacant(e) = now_map.entry(ch) {
                let mut is_end = if *count == 0 { '1' } else { '0' };
                let mut swm = SensitiveWordMap {
                    word: ch,
                    is_end,
                    word_map: Some(HashMap::<char, Box<SensitiveWordMap>>::new()),
                };
                now_map.insert(ch, Box::new(swm));
                if let Some(m) = now_map.get_mut(&ch) {
                    recursive_build_map(&mut *m, &mut *chars, count);
                }
            }else if let Some(m) = now_map.get_mut(&ch) {
                recursive_build_map(&mut *m, &mut *chars, count);
            }
           
        }
    }
}

/// 读取敏感词库，将敏感词放入HashMap中，构建一个DFA算法模型
///  {
///   '信': SensitiveWordMap {
///       word: '信',
///       is_end: '0',
///       word_map: Some({
///           '用': SensitiveWordMap {
///               word: '用',
///               is_end: '0',
///               word_map: Some({
///                   '卡': SensitiveWordMap {
///                       word: '卡',
///                       is_end: '0',
///                       word_map: Some({
///                           '套': SensitiveWordMap {
///                               word: '套',
///                               is_end: '0',
///                               word_map: Some({
///                                   '现': SensitiveWordMap {
///                                       word: '现',
///                                       is_end: '1',
///                                       word_map: Som e({})
///                                   }
///                               })
///                           },
///                           '代': SensitiveWordMap {
///                               word: '代',
///                               is_end: '0',
///                               word_map: Some({
///                                   '付': SensitiveWordMap {
///                                       word: '付',
///                                       is_end: '1',
///                                       word_map: Some({})
///                                   },
///                                   '还': SensitiveWordMap {
///                                       word: '还',
///                                       is_end: '1',
///                                       word_map: Some({})
///                                   }
///                               })
///                           }
///                       })
///                   }
///               })
///           }
///       })
///   }
///
fn build_sensitive_word_map(set: BTreeSet<String>) -> HashMap<char, SensitiveWordMap> {
    let mut sensitive_word_map = HashMap::<char, SensitiveWordMap>::new();

    let mut iterator = set.iter();
    for key in iterator {
        let len = key.chars().count();
        let mut count = len;
        let mut key_chars = key.chars();
        //读取每行的首个字符
        if let Some(first_char) = key_chars.next() {
            count -= 1;
            if let Some(word_map) = sensitive_word_map.get_mut(&first_char) {
                //读取下一个字符
                recursive_build_map(&mut *word_map, &mut key_chars, &mut count);
            } else {
                let mut is_end = if len == 1 { '1' } else { '0' };

                let mut now_map = SensitiveWordMap {
                    word: first_char,
                    is_end,
                    word_map: Some(HashMap::<char, Box<SensitiveWordMap>>::new()),
                };
                sensitive_word_map.insert(first_char, now_map);

                if let Some(now_map) = sensitive_word_map.get_mut(&first_char) {
                    recursive_build_map(&mut *now_map, &mut key_chars, &mut count);
                }
            }
        }
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
    let str_vec = vec![
        "花呗信用卡代还OK套现",
        "套花呗分期代付",
        "马上套现信用卡",
        "期货套利",
        "空手套白狼",
        "守信用卡脖子",
        "坚定信心,同舟共济,科学防治,精准施策",
        "D+1还是T+1秒到结算免结算费",
    ];

    println!("find_sensitive_word MaxMatchType......");
    for str in &str_vec {
        let set = find_sensitive_word(str, &MatchType::MaxMatchType);
        println!("{} --> {:?}", str, set);
    }

    println!("find_sensitive_word MinMatchType......");
    for str in &str_vec {
        let set = find_sensitive_word(str, &MatchType::MinMatchType);
        println!("{} --> {:?}", str, set);
    }

    println!("is_contains_sensitive_word......");
    for str in &str_vec {
        let is_contains = is_contains_sensitive_word(str, &MatchType::MinMatchType);
        println!("{} is contains sensitive words : {}", str, is_contains);
    }

    println!("replace_sensitive_word......");
    for str in &str_vec {
        let replace_str = replace_sensitive_word(str, &MatchType::MinMatchType, '*');

        println!("{} --> {}", str, replace_str);
    }

    let result = replace_sensitive_word("信用卡之家", &MatchType::MinMatchType, '*');
    assert_eq!(result, "**卡之家");
}

#[test]
fn sub_str() {
    //实现类似Java String.substring()的功能，注意并不是适用于所有的字符。
    let str = String::from("hello world");
    let char_vec: Vec<char> = str.chars().collect();
    let sub_str: String = char_vec[0..5].iter().collect();
    println!("sub_str:{}", sub_str);

    //不能使用上述代码进行截取子字符串的字符
    for c in "नमस्ते".chars() {
        println!("{}", c);
    }
}

#[test]
fn set_iter() {
    let mut b_tree_set = BTreeSet::<String>::new();
    b_tree_set.insert(String::from("A"));
    b_tree_set.insert(String::from("B"));
    b_tree_set.insert(String::from("C"));
    b_tree_set.insert(String::from("D"));
    b_tree_set.insert(String::from("E"));

    for val in &b_tree_set {
        println!("{}", val);
    }

    let rm_key = String::from("C");
    b_tree_set.remove(&rm_key);

    println!("b_tree_set has {} items", b_tree_set.len());

    println!("using VSCode coding rust program is greate");
}
