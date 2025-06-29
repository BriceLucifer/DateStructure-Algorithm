use std::{collections::HashMap, i32};

fn main() {
    let num = 245_938_234;
    println!("{}", number_to_word(num));
}

pub fn number_to_word(num: i32) -> String {
    // Billion -> 10**9

    // Million -> 10**6

    // Thousand -> 10**3

    // Hundred -> 10**2
    // 2,147,483,647

    match num {
        0 => {
            return String::from("Zero");
        }
        1..1_000 => {
            return with3num(num);
        }
        1_000 => {
            return String::from("One Thousand");
        }
        1001..1_000_000 => {
            let thousand = num / 1000;
            let hundred = num % 1000;
            if hundred == 0 {
                format!("{} Thousand", with3num(thousand))
            } else {
                format!("{} Thousand {}", with3num(thousand), with3num(hundred))
            }
        }
        1_000_000 => {
            return String::from("One Million");
        }
        1_000_001..1_000_000_000 => {
            let million = num / 1000000;
            let remain = num % 1000000;
            if remain == 0 {
                format!("{} Million", with3num(million))
            } else {
                format!("{} Million {}", with3num(million), number_to_word(remain))
            }
        }
        1_000_000_000 => {
            return String::from("One Billion");
        }
        1_000_000_001..2_147_483_647 => {
            let billion = num / 1000000000;
            let remain = num % 1000000000;
            if remain == 0 {
                format!("{} Billion", with3num(billion))
            } else {
                format!("{} Billion {}", with3num(billion), number_to_word(remain))
            }
        }
        2_147_483_647 => {
            return String::from(
                "Two Billion One Hundred Forty Seven Million Four Hundred Eighty Three Thousand Six Hundred Forty Seven",
            );
        }
        _ => String::new(),
    }
}

// deals with 1..1000
pub fn with3num(num: i32) -> String {
    let mut within2num = HashMap::new();

    within2num.insert(1, "One");
    within2num.insert(2, "Two");
    within2num.insert(3, "Three");
    within2num.insert(4, "Four");
    within2num.insert(5, "Five");
    within2num.insert(6, "Six");
    within2num.insert(7, "Seven");
    within2num.insert(8, "Eight");
    within2num.insert(9, "Nine");
    within2num.insert(10, "Ten");
    within2num.insert(11, "Eleven");
    within2num.insert(12, "Twelve");
    within2num.insert(13, "Thirteen");
    within2num.insert(14, "Fourteen");
    within2num.insert(15, "Fifteen");
    within2num.insert(16, "Sixteen");
    within2num.insert(17, "Seventeen");
    within2num.insert(18, "Eighteen");
    within2num.insert(19, "Nineteen");
    within2num.insert(20, "Twenty");
    within2num.insert(30, "Thirty");
    within2num.insert(40, "Forty");
    within2num.insert(50, "Fifty");
    within2num.insert(60, "Sixty");
    within2num.insert(70, "Seventy");
    within2num.insert(80, "Eighty");
    within2num.insert(90, "Ninety");

    match num {
        0 => format!(""),
        1..21 => within2num.get(&num).unwrap().to_string(),
        21..100 => {
            let tens = num / 10;
            let ones = num % 10;
            if ones == 0 {
                format!("{}", within2num.get(&(tens * 10)).unwrap())
            } else {
                format!(
                    "{} {}",
                    within2num.get(&(tens * 10)).unwrap(),
                    within2num.get(&ones).unwrap()
                )
            }
        }
        100 => format!("One Hundred"),
        _ => {
            let hundreds = num / 100;
            let tens = (num % 100) / 10;
            let ones = (num % 100) % 10;

            if tens == 0 && ones == 0 {
                format!("{} Hundred", within2num.get(&hundreds).unwrap())
            } else {
                format!(
                    "{} Hundred {}",
                    within2num.get(&hundreds).unwrap(),
                    match tens {
                        0 => within2num.get(&ones).unwrap().to_string(),
                        1 => within2num.get(&(tens * 10 + ones)).unwrap().to_string(),
                        _ => format!("{}", with3num(tens * 10 + ones)),
                    }
                )
            }
        }
    }
}
