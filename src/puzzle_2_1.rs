use std::str::FromStr;

#[derive(Debug, PartialEq, Clone)]
enum Move {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

#[derive(Debug, PartialEq)]
enum RoundOutcome {
    Loss = 0,
    Draw = 3,
    Win = 6,
}

#[derive(Debug, PartialEq, Clone)]
struct Round {
    my_move: Move,
    opponent_move: Move,
}

#[derive(Debug)]
struct RoundParseError {
    unknown_char: char,
}

impl FromStr for Round {
    type Err = RoundParseError;

    fn from_str(round: &str) -> Result<Self, Self::Err> {
        let chars: Vec<char> = round.chars().collect();
        let opponent_move = match chars[0] {
            'A' => Move::Rock,
            'B' => Move::Paper,
            'C' => Move::Scissors,
            unknown => {
                return Err(RoundParseError {
                    unknown_char: unknown,
                })
            }
        };
        let my_move = match chars[2] {
            'X' => Move::Rock,
            'Y' => Move::Paper,
            'Z' => Move::Scissors,
            unknown => {
                return Err(RoundParseError {
                    unknown_char: unknown,
                })
            }
        };
        Ok(Round {
            my_move: my_move,
            opponent_move: opponent_move,
        })
    }
}

fn round_outcome(round: Round) -> RoundOutcome {
    match (round.my_move, round.opponent_move) {
        (Move::Rock, Move::Scissors) => RoundOutcome::Win,
        (Move::Scissors, Move::Paper) => RoundOutcome::Win,
        (Move::Paper, Move::Rock) => RoundOutcome::Win,
        (my, other) => {
            if my == other {
                return RoundOutcome::Draw;
            }
            return RoundOutcome::Loss;
        }
    }
}

fn round_score(round: Round, outcome: RoundOutcome) -> i32 {
    round.my_move as i32 + outcome as i32
}

pub fn puzzle(input: &str) -> i32 {
    input
        .split('\n')
        .map(|round_line| Round::from_str(round_line.trim()).unwrap())
        .map(|round| (round.clone(), round_outcome(round.clone())))
        .map(|(round, outcome)| round_score(round, outcome))
        .sum()
}

#[test]
fn test_puzzle() {
    let input = "A Y
    B X
    C Z";

    let result = puzzle(input);

    assert_eq!(result, 15);
}
