use itertools::Itertools;

pub fn calc_safe_reports(lines: &Vec<String>) -> u32 {
    lines
        .iter()
        .map(|line| -> u32 {
            let mut windows = line
                .split_whitespace()
                .map(|x| x.parse::<u32>().unwrap())
                .tuple_windows();
            let (first_num, second_num) = windows.next().unwrap();
            let diff = first_num.abs_diff(second_num);
            if diff < 1 || diff > 3 {
                return 0;
            }
            let increasing = first_num < second_num;
            for (num1, num2) in windows {
                let diff = num1.abs_diff(num2);
                if diff < 1 || diff > 3 {
                    return 0;
                }
                if (increasing && num1 > num2) || (!increasing && num1 < num2) {
                    return 0;
                }
            }
            1
        })
        .sum()
}

pub fn calc_safe_reports_with_dampener(lines: &Vec<String>) -> u32 {
    lines
        .iter()
        .map(|line| -> u32 {
            let nums = line
                .split_whitespace()
                .map(|x| x.parse::<u32>().unwrap())
                .collect::<Vec<_>>();
            if !check_vec(&nums, false) {
                if check_vec(&nums[1..], true) {
                    return 1;
                } else {
                    return 0;
                }
            }
            return 1;
        })
        .sum()
}

fn check_vec(nums: &[u32], skipped: bool) -> bool {
    let mut i = 0;
    let mut j = 1;
    let mut skipped = skipped;
    let increasing = nums[0] < nums[1];
    while j < nums.len() {
        let diff = nums[i].abs_diff(nums[j]);
        if (diff < 1 || diff > 3)
            || (increasing && nums[i] > nums[j])
            || (!increasing && nums[i] < nums[j])
        {
            if skipped {
                return false;
            }
            if j == nums.len() - 1 {
                return true;
            }
            j += 1;
            let diff = nums[i].abs_diff(nums[j]);
            if (diff < 1 || diff > 3)
                || (increasing && nums[i] > nums[j])
                || (!increasing && nums[i] < nums[j])
            {
                return false;
            }
            i = j;
            j += 1;
            skipped = true;
        } else {
            i += 1;
            j += 1;
        }
    }
    return true;
}
