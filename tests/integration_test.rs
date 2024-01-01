extern crate rust_by_example;

#[test]
fn sensitive_word() {
    assert_eq!(rust_by_example::add(3, 2), 5);

    let timestamp = rust_by_example::time_gen();
    println!("now timestamp : {:?}", timestamp);

    //交换x,y的值
    let (mut x, mut y) = (254, 128);
    x ^= y;
    y ^= x;
    x ^= y;
    println!("now x ,y: {:?},{:?}", x, y);
    assert_eq!(254, y);
    assert_eq!(128, x);

    let machine_kind = if cfg!(unix) {
        "unix"
    } else if cfg!(windows) {
        "windows"
    } else {
        "unknown"
    };

    println!("I'm running on a {} machine!", machine_kind);
}

#[test]
fn rusty_book() {
    // 浮点数数组可以使用 Vec::sort_by 和 PartialOrd::partial_cmp 进行排序。
    let mut vec = vec![1.1, 1.15, 5.5, 1.123, 2.0, 3.14, 0.618];

    vec.sort_by(|a, b| a.partial_cmp(b).unwrap());

    assert_eq!(vec, vec![0.618, 1.1, 1.123, 1.15, 2.0, 3.14, 5.5]);
}

use image::GenericImageView;
use image::ImageFormat;
use std::{fs::File, time::Instant};
// use image::dynimage::DynamicImage;

#[test]
fn half_height() {
    let mut img = image::open("examples/scaledown/借条.jpeg").unwrap();
    let (width, height) = img.dimensions();
    let timer = Instant::now();
    let scaled = img.crop(0, 0, width, height / 2);
    // println!("Scaled by {} in {}", name, Elapsed::from(&timer));
    let mut output =
        File::create(&format!("examples/scaledown/借条-{}.jpeg", "half-height")).unwrap();
    scaled.write_to(&mut output, ImageFormat::Png).unwrap();
    println!("{:?}", timer.elapsed());
}

use calamine::DataType::{
    Bool, DateTime, DateTimeIso, Duration, DurationIso, Empty, Error, Float, String,
};
use calamine::{
    open_workbook, open_workbook_auto, Ods, Reader, Sheet, SheetType, SheetVisible, Xls, Xlsb, Xlsx,
};
use calamine::{CellErrorType::*, DataType};

#[test]
fn any_sheets_xlsx() {
    let path = format!(
        "{}/examples/file/any_sheets.xlsx",
        env!("CARGO_MANIFEST_DIR")
    );
    println!("{}", &path);
    let mut workbook: Xlsx<_> = open_workbook(path).unwrap();

    let range = workbook.worksheet_range("Visible").unwrap();
    let total_cells = range.get_size().0 * range.get_size().1;
    let non_empty_cells: usize = range.used_cells().count();
    println!(
        "Found {} cells in 'Sheet1', including {} non empty cells",
        total_cells, non_empty_cells
    );
    // alternatively, we can manually filter rows
    assert_eq!(
        non_empty_cells,
        range
            .rows()
            .flat_map(|r| r.iter().filter(|&c| c != &DataType::Empty))
            .count()
    );

    // assert_eq!(
    //     workbook.sheets_metadata(),
    //     &[
    //         Sheet {
    //             name: "Visible".to_string(),
    //             typ: SheetType::WorkSheet,
    //             visible: SheetVisible::Visible
    //         },
    //         Sheet {
    //             name: "Hidden".to_string(),
    //             typ: SheetType::WorkSheet,
    //             visible: SheetVisible::Hidden
    //         },
    //         Sheet {
    //             name: "VeryHidden".to_string(),
    //             typ: SheetType::WorkSheet,
    //             visible: SheetVisible::VeryHidden
    //         },
    //         Sheet {
    //             name: "Chart".to_string(),
    //             typ: SheetType::ChartSheet,
    //             visible: SheetVisible::Visible
    //         },
    //     ]
    // );
}

//ColumnNumberToName
#[test]
fn test_col() {
    println!("{}", column_number_to_name(MAX_COLUMNS));
    println!("{}", column_number_to_name(37));
    println!("{}", column_name_to_number("AK".to_string()));
    println!("{}", column_name_to_number("XFC".to_string()));
}

const MIN_COLUMNS: usize = 1;
const MAX_COLUMNS: usize = 16384;
/// The column number convert to column name
fn column_number_to_name(num: usize) -> std::string::String {
    if num < MIN_COLUMNS || num > MAX_COLUMNS {
        return "".to_string();
    }
    let mut ret = num;
    let mut col = std::string::String::new();
    while ret > 0 {
        let ch = ((ret - 1) % 26 + 65) as u8;
        ret = (ret - 1) / 26;
        col.insert(0, ch as char);
    }

    col
}

///
/// column name convert to column number
fn column_name_to_number(name: std::string::String) -> usize {
    let len = name.len();
    if len == 0 {
        return 0;
    }
    let mut col = 0;
    let bytes = name.as_bytes();
    let mut i = len - 1;
    let mut multi = 1;
    loop {
        let ch = bytes[i];
        if ch >= b'A' && ch <= b'Z' {
            col += multi * (ch - b'A' + 1) as usize;
        } else if ch >= b'a' && ch <= b'z' {
            col += multi * (ch - b'a' + 1) as usize;
        } else {
            return 0;
        }

        if i < 1 {
            break;
        }
        i -= 1;
        multi *= 26;
    }
    if col > MAX_COLUMNS {
        return 0;
    }

    col
}
