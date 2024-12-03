mod day_1;
mod day_2;
mod day_3;

fn main() {
    println!("Day 1:");
    println!(
        "Sum of list item distances: {}\nSimilarity score of both lists: {}",
        day_1::sum_list_distances(),
        day_1::calc_similarity_score()
    );
    println!();

    println!("Day 2:");
    println!(
        "Number of safe reports: {}\nNumber of safe reports with dampener: {}",
        day_2::calc_safe_reports(),
        day_2::calc_safe_reports_with_dampener()
    );
    println!();

    println!("Day 3:");
    println!(
        "Multiplication results: {}\nMultiplication results with conditional commands: {}",
        day_3::multiplication_results(),
        day_3::multiplication_results_with_conditional()
    );
    println!();
}
