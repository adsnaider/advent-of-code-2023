use std::collections::HashMap;

use day1::read_input;
use once_cell::sync::Lazy;

fn main() {
    let mut sum: u64 = 0;
    for line in read_input() {
        let line = line.unwrap();
        sum += get_number(&line);
    }
    println!("The sum is {sum}");
}

fn get_number(line: &str) -> u64 {
    static MAP: Lazy<HashMap<&'static str, u8>> = Lazy::new(|| {
        map! {
            "0" => 0,
            "1" => 1,
            "2" => 2,
            "3" => 3,
            "4" => 4,
            "5" => 5,
            "6" => 6,
            "7" => 7,
            "8" => 8,
            "9" => 9,
            "zero" => 0,
            "one" => 1,
            "two" => 2,
            "three" => 3,
            "four" => 4,
            "five" => 5,
            "six" => 6,
            "seven" => 7,
            "eight" => 8,
            "nine" => 9,
        }
    });

    let mut first = line.len();
    let mut last = 0;
    let mut first_digit = 0;
    let mut last_digit = 0;

    for (key, value) in MAP.iter() {
        if let Some(idx) = line.find(key) {
            if idx <= first {
                first = idx;
                first_digit = (*value).into()
            }
        }
        if let Some(idx) = line.rfind(key) {
            if idx >= last {
                last = idx;
                last_digit = (*value).into()
            }
        }
    }

    first_digit * 10 + last_digit
}

#[macro_export]
macro_rules! map {
    {} => {
        std::collections::HashMap::new()
    };
    {$($key:expr => $value:expr,)*} => {
        let mut map = std::collections::HashMap::new();
        $(
            assert!(map.insert($key, $value).is_none());
        )*
        map
    };
}
