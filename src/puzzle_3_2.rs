use itertools::Itertools;

type BagContents = str;
type Item = char;
type Score = u32;

#[derive(Clone)]
struct SliceScorer {
    scores: Vec<Item>,
}

impl SliceScorer {
    fn new() -> Self {
        return SliceScorer {
            scores: "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ"
                .chars()
                .collect(),
        };
    }
}

trait Scorer {
    fn score(&self, item: Item) -> Score;
}

impl Scorer for &SliceScorer {
    fn score(&self, item: Item) -> Score {
        let (position, _) = self
            .scores
            .iter()
            .find_position(|c| **c == item)
            .expect("expect to find position");
        position as u32 + 1
    }
}

pub fn puzzle(input: &str) -> u32 {
    let scorer = &SliceScorer::new();

    input
        .lines()
        .chunks(3)
        .into_iter()
        .map(|chunk| find_duplicate_item_type(chunk))
        .map(|item| scorer.score(item))
        .sum()
}

use array_tool::vec::Intersect;

fn find_duplicate_item_type<'a>(chunk: impl Iterator<Item = &'a BagContents>) -> Item {
    let chunked_bag_contents: Vec<&BagContents> = chunk
        .map(|bag_contents: &BagContents| bag_contents.trim())
        .collect();

    *chunked_bag_contents[0]
        .chars()
        .collect::<Vec<_>>()
        .intersect(chunked_bag_contents[1].chars().collect())
        .intersect(chunked_bag_contents[2].chars().collect())
        .first()
        .unwrap()
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

    assert_eq!(result, 70);
}

#[test]
fn test_scorer() {
    let scorer = &SliceScorer::new();
    assert_eq!(scorer.score('p'), 16);
    assert_eq!(scorer.score('L'), 38);
    assert_eq!(scorer.score('P'), 42);
    assert_eq!(scorer.score('v'), 22);
    assert_eq!(scorer.score('t'), 20);
    assert_eq!(scorer.score('s'), 19);
}
