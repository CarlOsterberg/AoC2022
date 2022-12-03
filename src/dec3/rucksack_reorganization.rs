use crate::get_data_as_string;
use std::collections::HashSet;
use std::collections::HashMap;
use std::iter::zip;

struct Rucksack {
    first_half: HashSet<char>,
    second_half: HashSet<char>,
}

impl Rucksack {
    fn new(rucksack: &str) -> Rucksack {
        let (first_part, second_part) = rucksack.split_at(rucksack.len()/2);
        let mut left_set:HashSet<char> = HashSet::new();
        let mut right_set:HashSet<char> = HashSet::new();
        for char in first_part.chars() {
            left_set.insert(char);
        }
        for char in second_part.chars() {
            right_set.insert(char);
        }
        Rucksack {
            first_half: left_set,
            second_half: right_set,
        }
    }

    fn intersection_as_sum(&self) -> u32 {
        let values: HashMap<char, u32> =
            zip(('a'..='z').chain('A'..='Z'),
                (1..=26).chain(27..=52)).collect();
        let intersection =
            self.first_half.intersection(&self.second_half);
        let mut sum = 0;
        for value in intersection {
            sum += values.get(value).unwrap();
        }
        sum
    }

    fn intersection_of_three(&self, second: Rucksack, third: Rucksack) -> HashSet<char> {
        let first_complete: HashSet<char> = self.first_half.union(&self.second_half).copied().collect();
        let second_complete: HashSet<char> = second.first_half.union(&second.second_half).copied().collect();
        let third_complete: HashSet<char> = third.first_half.union(&third.second_half).copied().collect();
        let intersect_first_second: HashSet<char> = first_complete.intersection(&second_complete).copied().collect();
        let intersect_first_second_third: HashSet<char> = intersect_first_second.intersection(&third_complete).copied().collect();
        intersect_first_second_third
    }
}

pub fn rucksacks_part_one(is_example: bool) -> u32 {
    let unparsed = get_data_as_string(is_example, "dec3");
    let mut sacks:Vec<Rucksack> = Vec::new();
    for line in unparsed.lines() {
        sacks.push(Rucksack::new(line));
    }
    sacks.iter()
        .map(|n|n.intersection_as_sum())
        .fold(0, | acc, n | acc + n)
}

pub fn rucksacks_part_two(is_example: bool) -> u32 {
    let unparsed = get_data_as_string(is_example, "dec3");
    let mut sacks:Vec<Rucksack> = Vec::new();
    for line in unparsed.lines() {
        sacks.push(Rucksack::new(line));
    }
    let values: HashMap<char, u32> =
        zip(('a'..='z').chain('A'..='Z'),
            (1..=26).chain(27..=52)).collect();
    let mut sum = 0;
    for _ in 1..=(sacks.len()/3) {
        let first = sacks.pop().unwrap();
        let second = sacks.pop().unwrap();
        let third = sacks.pop().unwrap();
        let intersection_set = first.intersection_of_three(second, third);
        let intersection = intersection_set.iter().next().unwrap();
        sum += values.get(intersection).unwrap();
    }
    sum
}