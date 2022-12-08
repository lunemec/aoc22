use itertools::Itertools;

fn find_duplicate_characters(bag_contents: &str) -> Vec<char> {
    let chars_count = bag_contents.chars().count();
    let half = chars_count / 2;
    let first_half = bag_contents.chars().take(half);
    let second_half: Vec<_> = bag_contents
        .chars()
        .skip(half)
        .take(chars_count - half)
        .collect();

    first_half
        .filter(|c| second_half.contains(&c))
        .unique()
        .collect()
}

fn score_one(scores: &Vec<char>, item: char) -> u32 {
    let (position, _) = scores
        .iter()
        .find_position(|c| **c == item)
        .expect("expect to find position");
    position as u32 + 1
}

fn score_duplicates(scores: &Vec<char>, duplicates: Vec<char>) -> u32 {
    duplicates.into_iter().map(|c| score_one(&scores, c)).sum()
}

pub fn puzzle(input: &str) -> u32 {
    let scores: Vec<char> = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ"
        .chars()
        .collect();

    input
        .lines()
        .map(|bag_contents| find_duplicate_characters(bag_contents.trim()))
        .map(|duplicate_bag_contents| score_duplicates(&scores, duplicate_bag_contents))
        .sum()
}

#[test]
fn test_puzzle() {
    let input = "vJrwpWtwJgWrhcsFMMfFFhFp
    jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
    PmmdzqPrVvPwwTWBwg
    wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
    ttgJtRGJQctTZtZT
    CrZsJsPPZsGzwwsLwLmpwMDw";

    let result = puzzle(input);

    assert_eq!(result, 157);
}

#[test]
fn test_find_duplicate_characters() {
    assert_eq!(
        find_duplicate_characters("vJrwpWtwJgWrhcsFMMfFFhFp"),
        vec!['p']
    );
    assert_eq!(
        find_duplicate_characters("jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL"),
        vec!['L']
    );
}

#[test]
fn test_score_one() {
    let scores: Vec<char> = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ"
        .chars()
        .collect();
    assert_eq!(score_one(&scores, 'p'), 16);
    assert_eq!(score_one(&scores, 'L'), 38);
    assert_eq!(score_one(&scores, 'P'), 42);
    assert_eq!(score_one(&scores, 'v'), 22);
    assert_eq!(score_one(&scores, 't'), 20);
    assert_eq!(score_one(&scores, 's'), 19);
}
