use std::{collections::HashMap, iter::zip};

pub fn sum_list_distances(lines: &Vec<String>) -> u32 {
    let (mut list1, mut list2) = lines
        .iter()
        .map(|line| -> (u32, u32) {
            let split = line.split_whitespace().collect::<Vec<_>>();
            (split[0].parse().unwrap(), split[1].parse().unwrap())
        })
        .fold((vec![], vec![]), |mut acc, x| {
            acc.0.push(x.0);
            acc.1.push(x.1);
            acc
        });
    list1.sort();
    list2.sort();
    zip(list1, list2).fold(0, |acc, x| acc + x.0.abs_diff(x.1))
}

pub fn calc_similarity_score(lines: &Vec<String>) -> u32 {
    let (list1, list2) = lines
        .iter()
        .map(|line| -> (u32, u32) {
            let split = line.split_whitespace().collect::<Vec<_>>();
            (split[0].parse().unwrap(), split[1].parse().unwrap())
        })
        .fold((vec![], vec![]), |mut acc, x| {
            acc.0.push(x.0);
            acc.1.push(x.1);
            acc
        });
    let freq = list2
        .into_iter()
        .fold(HashMap::<u32, u32>::new(), |mut freq, num| {
            freq.entry(num).and_modify(|e| *e += 1).or_insert(1);
            freq
        });
    list1.into_iter().fold(0, |acc, x| {
        acc + x * freq.get(&x).copied().unwrap_or_default()
    })
}
