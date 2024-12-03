use core::str;

const INPUT: &str = include_str!("day_3.txt");

pub fn multiplication_results() -> i32 {
    let mut mul_pairs = Vec::<String>::new();
    let mut i = 0;
    while i < INPUT.len() {
        if INPUT[i..].starts_with("mul(") {
            i += 4;
            let mut contents = String::new();
            while i < INPUT.len() {
                if &INPUT[i..i + 1] == ")" {
                    mul_pairs.push(contents);
                    break;
                } else if INPUT[i..].starts_with("mul(") {
                    i -= 1;
                    break;
                } else {
                    contents.push_str(&INPUT[i..i + 1]);
                }
                i += 1;
            }
        }
        i += 1;
    }

    mul_pairs
        .iter()
        .map(|s| -> i32 {
            let nums = s
                .split(',')
                .map(|n| n.parse::<i32>().unwrap_or_default())
                .collect::<Vec<_>>();
            if nums.len() != 2 {
                0
            } else {
                nums[0] * nums[1]
            }
        })
        .sum()
}

pub fn multiplication_results_with_conditional() -> i32 {
    let mut mul_pairs = Vec::<String>::new();
    let mut enabled = true;
    let mut i = 0;
    while i < INPUT.len() {
        if INPUT[i..].starts_with("do()") {
            enabled = true;
            i += 4;
        } else if INPUT[i..].starts_with("don't()") {
            enabled = false;
            i += 7;
        } else if enabled && INPUT[i..].starts_with("mul(") {
            i += 4;
            let mut contents = String::new();
            while i < INPUT.len() {
                if &INPUT[i..i + 1] == ")" {
                    mul_pairs.push(contents);
                    break;
                } else if INPUT[i..].starts_with("mul(")
                    || INPUT[i..].starts_with("do()")
                    || INPUT[i..].starts_with("don't()")
                {
                    break;
                } else {
                    contents.push_str(&INPUT[i..i + 1]);
                }
                i += 1;
            }
        } else {
            i += 1;
        }
    }

    mul_pairs
        .iter()
        .map(|s| -> i32 {
            let nums = s
                .split(',')
                .map(|n| n.parse::<i32>().unwrap_or_default())
                .collect::<Vec<_>>();
            if nums.len() != 2 {
                0
            } else {
                nums[0] * nums[1]
            }
        })
        .sum()
}
