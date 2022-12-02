use std::str::FromStr;

fn main() {
    let file = include_str!("../input");

    let mut score = 0;
    for line in file.lines() {
        if let Some((opponent_move, choice)) = line.split_once(' ') {
            let opponent_move = opponent_move.parse::<OpponentMove>().unwrap();
            let choice = choice.parse::<Choice>().unwrap();

            let our_move = choice.choice_to_move(&opponent_move);

            score += our_move.to_score();
            match our_move.partial_cmp(&opponent_move) {
                Some(std::cmp::Ordering::Greater) => score += 6,
                Some(std::cmp::Ordering::Less) => score += 0,
                Some(std::cmp::Ordering::Equal) => score += 3,
                None => panic!("could not compare moves"),
            }
        }
    }

    print!("score: {}", score)
}

#[derive(Debug)]
enum OpponentMove {
    Rock,
    Paper,
    Scissors,
}

impl FromStr for OpponentMove {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" => Ok(OpponentMove::Rock),
            "B" => Ok(OpponentMove::Paper),
            "C" => Ok(OpponentMove::Scissors),
            i => Err(format!("unregonised move {}", i)),
        }
    }
}

#[derive(Debug)]
enum Move {
    Rock,
    Paper,
    Scissors,
}

impl Move {
    fn to_score(&self) -> i32 {
        match self {
            Move::Rock => 1,
            Move::Paper => 2,
            Move::Scissors => 3,
        }
    }
}

impl PartialOrd<OpponentMove> for Move {
    fn partial_cmp(&self, other: &OpponentMove) -> Option<std::cmp::Ordering> {
        match (self, other) {
            (Move::Rock, OpponentMove::Rock) => Some(std::cmp::Ordering::Equal),
            (Move::Rock, OpponentMove::Paper) => Some(std::cmp::Ordering::Less),
            (Move::Rock, OpponentMove::Scissors) => Some(std::cmp::Ordering::Greater),
            (Move::Paper, OpponentMove::Rock) => Some(std::cmp::Ordering::Greater),
            (Move::Paper, OpponentMove::Paper) => Some(std::cmp::Ordering::Equal),
            (Move::Paper, OpponentMove::Scissors) => Some(std::cmp::Ordering::Less),
            (Move::Scissors, OpponentMove::Rock) => Some(std::cmp::Ordering::Less),
            (Move::Scissors, OpponentMove::Paper) => Some(std::cmp::Ordering::Greater),
            (Move::Scissors, OpponentMove::Scissors) => Some(std::cmp::Ordering::Equal),
        }
    }
}
impl PartialEq<OpponentMove> for Move {
    fn eq(&self, other: &OpponentMove) -> bool {
        matches!(
            (self, other),
            (Move::Rock, OpponentMove::Rock)
                | (Move::Paper, OpponentMove::Paper)
                | (Move::Scissors, OpponentMove::Scissors)
        )
    }
}

// task 1
// impl FromStr for Move {
//     type Err = String;

//     fn from_str(s: &str) -> Result<Self, Self::Err> {
//         match s {
//             "X" => Ok(Move::Rock),
//             "Y" => Ok(Move::Paper),
//             "Z" => Ok(Move::Scissors),
//             i => Err(format!("unregonised move {}", i)),
//         }
//     }
// }

enum Choice {
    Win,
    Lose,
    Draw,
}

impl Choice {
    fn choice_to_move(&self, other: &OpponentMove) -> Move {
        match (self, other) {
            (Choice::Win, OpponentMove::Rock) => Move::Paper,
            (Choice::Win, OpponentMove::Paper) => Move::Scissors,
            (Choice::Win, OpponentMove::Scissors) => Move::Rock,
            (Choice::Lose, OpponentMove::Rock) => Move::Scissors,
            (Choice::Lose, OpponentMove::Paper) => Move::Rock,
            (Choice::Lose, OpponentMove::Scissors) => Move::Paper,
            (Choice::Draw, OpponentMove::Rock) => Move::Rock,
            (Choice::Draw, OpponentMove::Paper) => Move::Paper,
            (Choice::Draw, OpponentMove::Scissors) => Move::Scissors,
        }
    }
}

impl FromStr for Choice {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "X" => Ok(Choice::Lose),
            "Y" => Ok(Choice::Draw),
            "Z" => Ok(Choice::Win),
            i => Err(format!("unregonised choice {}", i)),
        }
    }
}
