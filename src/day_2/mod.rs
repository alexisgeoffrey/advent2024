const INPUT: &str = include_str!("day_2.txt");

pub fn calc_safe_reports() -> i32 {
    INPUT
        .lines()
        .map(|line| -> i32 {
            let nums = line
                .split_whitespace()
                .map(|x| x.parse::<i32>().unwrap())
                .collect::<Vec<_>>();
            if check_vec(&nums, true) {
                1
            } else {
                0
            }
        })
        .sum()
}

pub fn calc_safe_reports_with_dampener() -> i32 {
    INPUT
        .lines()
        .map(|line| -> i32 {
            let nums = line
                .split_whitespace()
                .map(|x| x.parse().unwrap())
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

fn check_vec(nums: &[i32], skipped: bool) -> bool {
    let mut i = 0;
    let mut j = 1;
    let mut skipped = skipped;
    let increasing = nums[0] < nums[1];
    while j < nums.len() {
        let diff = (nums[i] - nums[j]).abs();
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
            let diff = (nums[i] - nums[j]).abs();
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
