use itertools::Itertools;
use regex::Captures;
use regex::NoExpand;
use regex::Regex;
use std::collections::HashMap;

lazy_static! {
    static ref IDENTIFIER_REGEX: Regex = Regex::new("^([0-9ABCDEFGY]{1})([1239]{1})([0-9ABCDEFGHJKLMNPQRTUWXY]{6})([0-9ABCDEFGHJKLMNPQRTUWXY]{9})([0-9ABCDEFGHJKLMNPQRTUWXY])$").unwrap();
    static ref REGEX_18_ID_CARD_NO: Regex = Regex::new(r"^\d{17}[\dX]$").unwrap();
    static ref ID_CARD_POWER: Vec<usize> = vec![7, 9, 10, 5, 8, 4, 2, 1, 6, 3, 7, 9, 10, 5, 8, 4, 2];
    static ref WEIGHT: Vec<usize> = vec![1, 3, 9, 27, 19, 26, 16, 17, 20, 29, 25, 13, 8, 24, 10, 30, 28];
    static ref VERIFY_CODE_MAP: HashMap<char,usize> = {
        let verify_code = "0123456789ABCDEFGHJKLMNPQRTUWXY";
        let verify_code_vec :Vec<char> = verify_code.chars().collect();
        let mut verify_code_map = HashMap::new();
        for (i,item) in verify_code_vec.iter().enumerate().take(verify_code.len()){
            verify_code_map.insert(*item,i);

        }
        println!("{:?}",verify_code_map);
        verify_code_map
    };
    // 公司名称需要排除的字符串，不包含中英文括号
    static ref REGEX_NOT_COMPANY_NAME: Regex = Regex::new(r###"[`~!@#$%^&*+=|{}':;',\\.<>《》/?~！@#￥%……&*——+|\-{}\[\]【】‘；："”“’。，、？]"###).unwrap();
    // 金钱（千分位）
    static ref MONEY_REGEX: Regex = Regex::new(r"^(-)?\d{1,3}(,\d{3})*(.\d+)?$").unwrap();
    //驼峰命名
    static ref CAMEL_TO_SNAKE1: Regex = Regex::new(r"(.)([A-Z][a-z]+)").unwrap();
    static ref CAMEL_TO_SNAKE2: Regex = Regex::new(r"([a-z0-9])([A-Z])").unwrap();
}

/// 18位身份证号校验
pub fn is_18_id_card(id_card_no: &str) -> bool {
    if !REGEX_18_ID_CARD_NO.is_match(id_card_no) {
        println!("{} isn't match regex", id_card_no);
        return false;
    }

    let id_card_no_chars: Vec<char> = id_card_no.chars().collect();
    let mut sum = 0;
    for i in 0..17 {
        let ch = id_card_no_chars[i] as usize - 48;
        sum += ch * ID_CARD_POWER[i];
    }

    let check_code = match sum % 11 {
        10 => '2',
        9 => '3',
        8 => '4',
        7 => '5',
        6 => '6',
        5 => '7',
        4 => '8',
        3 => '9',
        2 => 'X',
        1 => '0',
        0 => '1',
        _ => 'N',
    };

    check_code == id_card_no_chars[17]
}

/// 校验法人和其他组织的统一社会信用代码（详情见 https://baike.baidu.com/item/%E6%B3%95%E4%BA%BA%E5%92%8C%E5%85%B6%E4%BB%96%E7%BB%84%E7%BB%87%E7%BB%9F%E4%B8%80%E7%A4%BE%E4%BC%9A%E4%BF%A1%E7%94%A8%E4%BB%A3%E7%A0%81/17702814?fromtitle=%E7%BB%9F%E4%B8%80%E7%A4%BE%E4%BC%9A%E4%BF%A1%E7%94%A8%E4%BB%A3%E7%A0%81&fromid=18754790&fr=aladdin）
/// 校验算法参考：https://blog.csdn.net/u013361668/article/details/51595169
///
/// #Example
/// ```
/// let result = validators::is_unified_social_credit_identifier("91450100MA5MYH3E16");
/// assert_eq!(result,true);
/// ```
///
pub fn is_unified_social_credit_identifier(identifier: &str) -> bool {
    if !IDENTIFIER_REGEX.is_match(identifier) {
        println!("{} not match regex", identifier);
        return false;
    }
    let identifier_chars: Vec<char> = identifier.chars().collect();
    let mut sum = 0;
    for i in 0..17 {
        let ch = identifier_chars[i];
        if let Some(index) = VERIFY_CODE_MAP.get(&ch) {
            sum += index * WEIGHT[i];
        }
    }

    let mut temp = sum % 31;
    if temp != 0 {
    } else {
        temp = 31;
    }
    let c18 = 31 - temp;
    let verify_code = identifier_chars[17];
    if let Some(index) = VERIFY_CODE_MAP.get(&verify_code) {
        c18 == *index
    } else {
        false
    }
}

/// 过滤公司名称中的特殊符号
///
pub fn filter_company_name(company_name: &str) -> String {
    let filtered = REGEX_NOT_COMPANY_NAME.replace_all(company_name, "");
    let temp = filtered.to_string();
    //替换空格
    // let blank = Regex::new("\\s").unwrap();
    // blank.replace_all(&temp,"").to_string()
    let temp = filtered.to_string();
    temp.trim().to_owned()
}

pub fn is_money(number: &str) -> bool {
    MONEY_REGEX.is_match(number)
}

/// 驼峰命名转为蛇形命名
pub fn camel_to_snake(origin: &str) -> String {
    let result0 = CAMEL_TO_SNAKE1.replace_all(origin, |caps: &Captures| {
        format!("{}_{}", &caps[1], &caps[2])
    });
    let result = CAMEL_TO_SNAKE2.replace_all(&result0, |caps: &Captures| {
        format!("{}_{}", &caps[1], &caps[2])
    });
    result.to_uppercase()
}

pub fn snake_to_pascal(origin: &str) -> String {
    // origin.split('_');
    let word_vec: Vec<&str> = origin.split('_').collect();
    //每个单词首字母大写
    word_vec
        .iter()
        .map(|&word| {
            let mut chars = word.chars();
            match chars.next() {
                Some(ch) => ch.to_uppercase().collect::<String>() + chars.as_str(),
                None => String::new(),
            }
        })
        .join("")
}

pub fn snake_to_camel(s: &str) -> String {
    let result = snake_to_pascal(s);
    let mut chars = result.chars();
    match chars.next() {
        Some(ch) => ch.to_lowercase().collect::<String>() + chars.as_str(),
        None => String::new(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn id_cards_test() {
        let idcards = vec![
            "142431199001145",
            "452726199205040216",
            "33072619901128272X",
            "330726199011282728",
            "210402196506064118",
            "511621199405120323",
            "350629196510284019",
            "320311770706001",
            "320311770706002",
            "411347200105170016",
            "220181200209086919",
            "370104194310250136",
            "452726199205040216",
            "810000199408230021",
        ];

        for id in &idcards {
            println!("{} is valid : {}", id, is_18_id_card(id));
        }
    }

    #[test]
    fn identifiers_test() {
        let identifiers = vec![
            "",
            "   ",
            "91512081MA62K0260E",
            "91370200163562681G",
            "9137V2O0163562681G",
            "123456789012345678",
            "91330106MA2B27HM90",
            "91110108585852240Q",
            "31430000MD0217741Q",
            "410302600144140",
            "92510107MA62M3M24U",
            "9114021105885245XE",
            "91330109MA27WFB10H",
            "92320303MA1NGJ535N",
            "92320303MA1NG535N",
            "92420502MA4AF3986P",
            "91420502MA4AF3986P",
            "92370902MA3QTFEQXN",
            "92130827MA0E9FUBOY",
        ];

        for id in &identifiers {
            println!(
                "{} is valid :{}",
                id,
                is_unified_social_credit_identifier(id)
            );
        }
    }

    #[test]
    fn money_test() {
        let numbers = vec![
            "134,004,000.25",
            "100",
            "10",
            "10.25",
            "-10.25",
            "1,234",
            "12345",
            "-1,234",
            "abc",
        ];
        for number in numbers {
            let b = is_money(number);
            println!("{} is money : {}", number, b);
        }
    }

    #[test]
    fn regex_demo() {
        let rep = Regex::new(r"(?P<y>\d{4})-(?P<m>\d{2})-(?P<d>\d{2})").unwrap();
        let before = "2012-03-14, 2013-01-01 and 2014-07-05";
        let after = rep.replace_all(before, "$m/$d/$y");
        assert_eq!(after, "03/14/2012, 01/01/2013 and 07/05/2014");

        let re = Regex::new(r"(?i)Δ+").unwrap();
        let mat = re.find("ΔδΔ").unwrap();
        assert_eq!((mat.start(), mat.end()), (0, 6));
    }

    #[test]
    fn company_name() {
        let filtered_name = filter_company_name("  \\  中道集团--中付支付（广州分公司）~!@#$%^&*+=|{}':;',\\\\\\\\[\\\\\\\\].<>/?~！@#￥%……&*+|{}\\[\\]【北京顶流】‘；：\"”“’。，、？《上海硅谷》\",\"Hong Kong ABC Company(DEF branch)（中文括号）[深圳务实]");

        println!("{}", filtered_name);
    }

    #[test]
    fn fileds() {
        let fileds_vec = vec!["FalconHeavyRocket", "HTTPResponseCodeXYZ"];
        fileds_vec.iter().for_each(|&filed| {
            let result = camel_to_snake(filed);
            println!("{}", result);
        });

        let columns_vec = vec!["falcon_heavy_rocket", "http_response_code_xyz"];
        columns_vec.iter().for_each(|&column| {
            let result = snake_to_pascal(column);
            println!("{}", result);
        });

        let columns_vec = vec!["falcon_heavy_rocket", "http_response_code_xyz"];
        columns_vec.iter().for_each(|&column| {
            let result = snake_to_camel(column);
            println!("{}", result);
        });
    }
}
