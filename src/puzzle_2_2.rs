use std::str::FromStr;

#[derive(Debug, PartialEq, Clone)]
enum Move {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

#[derive(Debug, PartialEq, Clone)]
enum RoundOutcome {
    Loss = 0,
    Draw = 3,
    Win = 6,
}

#[derive(Debug, PartialEq, Clone)]
struct IncompleteRound {
    opponent_move: Move,
    needed_outcome: RoundOutcome,
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

impl FromStr for IncompleteRound {
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
        let needed_outcome = match chars[2] {
            'X' => RoundOutcome::Loss,
            'Y' => RoundOutcome::Draw,
            'Z' => RoundOutcome::Win,
            unknown => {
                return Err(RoundParseError {
                    unknown_char: unknown,
                })
            }
        };
        Ok(IncompleteRound {
            opponent_move: opponent_move,
            needed_outcome: needed_outcome,
        })
    }
}

fn round_outcome(round: IncompleteRound) -> Round {
    let my_move = match (round.needed_outcome, &round.opponent_move) {
        (RoundOutcome::Draw, any) => any.clone(),
        (RoundOutcome::Win, opponent_move) => match opponent_move {
            Move::Rock => Move::Paper,
            Move::Paper => Move::Scissors,
            Move::Scissors => Move::Rock,
        },
        (RoundOutcome::Loss, opponent_move) => match opponent_move {
            Move::Rock => Move::Scissors,
            Move::Paper => Move::Rock,
            Move::Scissors => Move::Paper,
        },
    };
    Round {
        my_move: my_move,
        opponent_move: round.opponent_move,
    }
}

fn round_score(round: Round, outcome: RoundOutcome) -> i32 {
    round.my_move as i32 + outcome as i32
}

pub fn puzzle(input: &str) -> i32 {
    input
        .split('\n')
        .map(|round_line| IncompleteRound::from_str(round_line.trim()).unwrap())
        .map(|incomplete_round| {
            (
                incomplete_round.clone(),
                round_outcome(incomplete_round.clone()),
            )
        })
        .map(|(incomplete_round, round)| round_score(round, incomplete_round.needed_outcome))
        .sum()
}

#[test]
fn test_puzzle() {
    let input = "A Y
    B X
    C Z";

    let result = puzzle(input);

    assert_eq!(result, 12);
}
