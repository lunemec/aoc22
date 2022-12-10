use std::str::FromStr;

use array_tool::vec::Intersect;

pub fn puzzle(input: &str) -> u32 {
    input
        .trim()
        .lines()
        .map(to_pair_assignment)
        .filter(|pair_assignment| has_duplicate_section_assignments(pair_assignment))
        .count() as u32
}

#[derive(Clone)]
struct PairAssignment(AssignedSections, AssignedSections);
type AssignedSections = Vec<SectionID>;
type SectionID = u32;

fn to_pair_assignment(raw_pair_assignment: &str) -> PairAssignment {
    PairAssignment::from_str(raw_pair_assignment).unwrap()
}

#[derive(Debug)]
struct PairAssignmentParseError {}

impl FromStr for PairAssignment {
    type Err = PairAssignmentParseError;

    fn from_str(raw_pair_assignment: &str) -> Result<Self, Self::Err> {
        let pair_assignment: Vec<AssignedSections> = raw_pair_assignment
            .split(',')
            .map(|raw_min_max| {
                let min_max: Vec<_> = raw_min_max
                    .trim()
                    .split('-')
                    .map(|minmax| minmax.parse::<SectionID>().unwrap())
                    .collect();
                (min_max[0]..=min_max[1]).collect::<Vec<_>>()
            })
            .collect();
        Ok(PairAssignment {
            0: pair_assignment[0].clone(),
            1: pair_assignment[1].clone(),
        })
    }
}

fn has_duplicate_section_assignments(pair_assignment: &PairAssignment) -> bool {
    let intersect = pair_assignment.0.intersect(pair_assignment.1.to_owned());
    intersect.len() > 0
}

#[test]
fn test_puzzle() {
    let input = "2-4,6-8
    2-3,4-5
    5-7,7-9
    2-8,3-7
    6-6,4-6
    2-6,4-8";

    let result = puzzle(input);

    assert_eq!(result, 4);
}
