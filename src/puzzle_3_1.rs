fn find_duplicate_characters(bag_contents: &str) -> Vec<&str> {
    let chars_count = bag_contents.chars().count();
    
    Vec::new()
}

fn score_duplicate(duplicates: Vec<&str>) -> i32 {
    0
}

pub fn puzzle(input: &str) -> i32 {
    input
        .lines()
        .map(|bag_contents| find_duplicate_characters(bag_contents))
        .map(|duplicate_bag_contents| score_duplicate(duplicate_bag_contents))
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
