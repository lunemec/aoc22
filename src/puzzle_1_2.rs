use itertools::Itertools;

pub fn puzzle(input: &str) -> i32 {
    let grouped_iter = input.split('\n').map(|x| x.trim()).group_by(|x| *x != "");

    let mut calories_per_elf: Vec<i32> = Vec::new();
    for (is_elf, group) in grouped_iter.into_iter() {
        let group_data: Vec<&str> = group.collect();

        if is_elf {
            calories_per_elf.push(
                group_data
                    .into_iter()
                    .map(|calories_str| calories_str.parse::<i32>().unwrap())
                    .sum(),
            );
        }
    }

    calories_per_elf.into_iter().sorted().rev().take(3).sum()
}

#[test]
fn test_puzzle() {
    let input = "1000
    2000
    3000
    
    4000
    
    5000
    6000
    
    7000
    8000
    9000
    
    10000
    ";

    let result = puzzle(input);

    assert_eq!(result, 45000);
}
