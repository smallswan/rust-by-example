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
        for i in 0..verify_code.len(){
            verify_code_map.insert(verify_code_vec[i],i);

        }
        println!("{:?}",verify_code_map);
        verify_code_map
    };

}
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
            sum = sum + index * WEIGHT[i];
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
