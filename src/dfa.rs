use std::collections::BTreeSet;
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

use std::str::Chars;

lazy_static! {
   static ref SENSITIVE_WORD_MAP : HashMap<char, SensitiveWordMap> = {
       let set = read_sensitive_word_file();
       return  build_sensitive_word_map(set);
   };
}

enum MatchType {
    MinMatchType,
    MaxMatchType
}

#[derive(Debug)]
struct SensitiveWordMap {
    word: char,
    is_end: char,
    word_map: Option<HashMap<char, Box<SensitiveWordMap>>>,
}
///
///
fn find_sensitive_word(txt:String,match_type:u32) -> BTreeSet<String>{
    let mut sensitive_word_set = BTreeSet::<String>::new();
    let len = txt.chars().count();
    let txt_vec:Vec<char> = txt.chars().collect();
    let mut i = 0;
    while i < len {
        let length = check_sensitive_word(&txt, i, MatchType::MaxMatchType);
        if length > 0{    //存在,加入list中
            println!("i:{},length:{}",i,length);
            sensitive_word_set.insert(txt_vec[i..i+length].iter().collect());
            i = i + length -1 ;    //减1的原因，是因为for会自增
        }
        i+=1;
    }


    sensitive_word_set
}

/// 查文字中是否包含检敏感字符,如果存在，则返回敏感词字符的长度，不存在返回0
///
fn check_sensitive_word(txt:&str, begin_index:usize, match_type: MatchType) -> usize{
    let mut match_flag = 0;
    let mut last_match_length = 0;
    let mut word: char;
    let txt_vec:Vec<char> = txt.chars().collect();
    let len = txt.len();
    for i in begin_index..len{
            if let Some(word) = &txt_vec.get(i){
                    if let Some(swm) = SENSITIVE_WORD_MAP.get(&word) {
                        println!("i:{},check_sensitive_word {:?}",i, *swm);
                        match_flag += 1;
                        last_match_length = match_flag;
                        //递归查找
                        recursive_find_map(swm,&txt_vec,i+1,&mut match_flag,&mut last_match_length);
                    } else {
                        break;
                    }

            }

    }
    match_flag
//    last_match_length
}
/// 递归查找map
///
fn recursive_find_map(swm:&SensitiveWordMap,txt_vec:&Vec<char>,i:usize,match_flag: &mut usize,last_match_length:&mut usize){
    if i < txt_vec.len(){
        if let Some(word) = txt_vec.get(i){
            println!("i:{},word:{}",i,word);
            if let Some(wm) = &swm.word_map{
                if let Some(next_swm) = wm.get(word){
                    *match_flag+=1;
                    if *&swm.is_end == '1'{
                        *last_match_length = *match_flag;
                    }
                    recursive_find_map(&next_swm,txt_vec,i+1,match_flag,last_match_length);
                }
            }
        }
    }

}
/// 递归地修改map
fn r_map(map: &mut SensitiveWordMap, chars: &mut Chars,count:&mut usize) {
    if let Some(ch) = chars.next() {
        *count-=1;
        if let Some(now_map) = map.word_map.as_mut() {
            let contains_key = now_map.contains_key(&ch);
            println!("ch:{},contains_key:{:?}", ch, contains_key);

            if contains_key {
                if let Some(m) = now_map.get_mut(&ch) {
                    r_map(&mut *m, &mut *chars,count);
                }
            } else {
//                let count = chars.count();
//                println!("count:{}",count);
                let mut is_end = '0';
                if *count == 0{
                    is_end = '1';
                }
                let mut swm = SensitiveWordMap {
                    word: ch,
                    is_end,
                    word_map: Some(HashMap::<char, Box<SensitiveWordMap>>::new()),
                };
                now_map.insert(ch, Box::new(swm));
                if let Some(m) = now_map.get_mut(&ch) {
                    r_map(&mut *m, &mut *chars,count);
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
        let mut count = len;
        let mut key_chars = key.chars();
        //读取每行的首个字符
        if let Some(first_char) = key_chars.next() {
            count-=1;
            if let Some(word_map) = sensitive_word_map.get_mut(&first_char) {
                println!("first_char1：{}", first_char);
                //读取下一个字符
                r_map(&mut *word_map, &mut key_chars,&mut count);
            } else {
                println!("first_char2：{}", first_char);
                let mut is_end = '0';
                if len == 1{
                    is_end = '1';
                }

                let mut now_map = SensitiveWordMap {
                    word: first_char,
                    is_end,
                    word_map: Some(HashMap::<char, Box<SensitiveWordMap>>::new()),
                };
                sensitive_word_map.insert(first_char, now_map);

                if let Some(now_map) = sensitive_word_map.get_mut(&first_char) {
                    r_map(&mut *now_map, &mut key_chars,&mut count);
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
//    let set = read_sensitive_word_file();
//    let sensitive_word_map = build_sensitive_word_map(set);
//    println!("last sensitive_word_map-----{:?}", sensitive_word_map);

    let last_match_length = check_sensitive_word("信用卡代还", 0, MatchType::MaxMatchType);

    println!("last_match_length:{}",last_match_length);

    let set = find_sensitive_word(String::from("信用卡代还信用卡套"),0);
    println!("{:?}",set);
}
