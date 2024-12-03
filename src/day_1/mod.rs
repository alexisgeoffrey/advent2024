use std::{collections::HashMap, iter};

const INPUT: &str = include_str!("day_1.txt");

pub fn sum_list_distances() -> i32 {
    let (mut list1, mut list2): (Vec<_>, Vec<_>) = INPUT
        .lines()
        .map(|line| -> (i32, i32) {
            let split = line.split_whitespace().collect::<Vec<_>>();
            (split[0].parse().unwrap(), split[1].parse().unwrap())
        })
        .unzip();

    list1.sort();
    list2.sort();

    iter::zip(list1, list2).map(|(x, y)| (x - y).abs()).sum()
}

pub fn calc_similarity_score() -> i32 {
    let (list1, list2): (Vec<_>, Vec<_>) = INPUT
        .lines()
        .map(|line| -> (i32, i32) {
            let split = line.split_whitespace().collect::<Vec<_>>();
            (split[0].parse().unwrap(), split[1].parse().unwrap())
        })
        .unzip();

    let freq = list2.into_iter().fold(HashMap::new(), |mut freq, num| {
        freq.entry(num).and_modify(|e| *e += 1).or_insert(1);
        freq
    });

    list1
        .into_iter()
        .map(|x| x * freq.get(&x).unwrap_or(&0))
        .sum()
}
