use std::{
    fs::File,
    io::{self, BufRead},
};

mod day_1;
mod day_2;

fn main() {
    let day_1_lines = get_lines(1);
    println!("Day 1:");
    println!(
        "Sum of list item distances: {}\nSimilarity score of both lists: {}",
        day_1::sum_list_distances(&day_1_lines),
        day_1::calc_similarity_score(&day_1_lines)
    );
    println!();

    let day_2_lines = get_lines(2);
    println!("Day 2:");
    println!(
        "Number of safe reports: {}\nNumber of safe reports with dampener: {}",
        day_2::calc_safe_reports(&day_2_lines),
        day_2::calc_safe_reports_with_dampener(&day_2_lines)
    )
}

fn get_lines(day: u8) -> Vec<String> {
    io::BufReader::new(File::open(path(day)).expect("Could not open file"))
        .lines()
        .map(|l| l.unwrap())
        .collect()
}

fn path(day: u8) -> String {
    format!("src/day_{day}/day_{day}.txt")
}
