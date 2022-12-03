use crate::get_data_as_string;

#[derive(PartialEq)]
enum Move {
    Rock,
    Paper,
    Scissors,
}

enum Result {
    Win,
    Tie,
    Loss
}

struct Match {
    opponent: Move,
    player: Move,
}

struct MatchUpdated {
    opponent: Move,
    player: Result,
}

pub struct Matches {
    matches: Vec<Match>,
}

pub struct MatchesUpdated {
    matches: Vec<MatchUpdated>,
}

impl Matches {
    fn new(unparsed: String) -> Matches {
        let mut matches:Vec<Match> = Vec::new();
        let all_matches = unparsed.lines();
        for a_match in all_matches {
            matches.push(Match::new(a_match.to_string()))
        }
        Matches {
            matches
        }
    }

    fn result_in_points(&self) -> u32 {
        self.matches
            .iter()
            .map(| n | n.result())
            .fold(0, | acc, n | acc + n)
    }


}

impl MatchesUpdated {
    fn new(unparsed: String) -> MatchesUpdated {
        let mut matches:Vec<MatchUpdated> = Vec::new();
        let all_matches = unparsed.lines();
        for a_match in all_matches {
            matches.push(MatchUpdated::new(a_match.to_string()))
        }
        MatchesUpdated {
            matches
        }
    }

    fn result_in_points(&self) -> u32 {
        self.matches
            .iter()
            .map(| n | n.result())
            .fold(0, | acc, n | acc + n)
    }
}

impl Match {
    fn new(unparsed_match: String) -> Match {
        let mut moves = unparsed_match.split(" ");
        let opponent = Move::new(moves.next().unwrap()).unwrap();
        let player = Move::new(moves.next().unwrap()).unwrap();
        Match {
            opponent,
            player,
        }
    }

    fn result(&self) -> u32 {
        match self.player.winner(&self.opponent) {
            Result::Win => {6 + self.player.to_points()}
            Result::Tie => {3 + self.player.to_points()}
            Result::Loss => {0 + self.player.to_points()}
        }
    }
}

impl MatchUpdated {
    fn new(unparsed_match: String) -> MatchUpdated {
        let mut moves = unparsed_match.split(" ");
        let opponent = Move::new(moves.next().unwrap()).unwrap();
        let player = Result::new(moves.next().unwrap()).unwrap();
        MatchUpdated {
            opponent,
            player,
        }
    }

    fn result(&self) -> u32 {
        let player_move = match self.player {
            Result::Win => {
                match self.opponent {
                    Move::Rock => Move::Paper,
                    Move::Paper => Move::Scissors,
                    Move::Scissors => Move::Rock,
                }
            }
            Result::Tie => {
                match self.opponent {
                    Move::Rock => Move::Rock,
                    Move::Paper => Move::Paper,
                    Move::Scissors => Move::Scissors,
                }
            }
            Result::Loss => {
                match self.opponent {
                    Move::Rock => Move::Scissors,
                    Move::Paper => Move::Rock,
                    Move::Scissors => Move::Paper,
                }
            }
        };
        match self.player {
            Result::Win => {6 + player_move.to_points()}
            Result::Tie => {3 + player_move.to_points()}
            Result::Loss => {0 + player_move.to_points()}
        }
    }
}

impl Result {
    fn new(input: &str) -> Option<Result> {
        match input {
            "X" => Some(Result::Loss),
            "Y" => Some(Result::Tie),
            "Z" => Some(Result::Win),
            _ => None,
        }
    }
}

impl Move {
    fn new(input: &str) -> Option<Move> {
        match input {
            "A" | "X" => Some(Move::Rock),
            "B" | "Y" => Some(Move::Paper),
            "C" | "Z" => Some(Move::Scissors),
            _ => None,
        }
    }

    fn winner(&self, other: &Move) -> Result {
        if self == other {
            Result::Tie
        }
        else if self == &Move::Rock && other == &Move::Scissors ||
                self == &Move::Paper && other == &Move::Rock ||
                self == &Move::Scissors && other == &Move::Paper
        {
            Result::Win
        }
        else {
            Result::Loss
        }
    }
    fn to_points(&self) -> u32 {
        match self {
            Move::Rock => {1}
            Move::Paper => {2}
            Move::Scissors => {3}
        }
    }
}

pub fn get_points_first_version(is_example: bool) -> u32 {
    let matches = Matches::new(get_data_as_string(is_example, "dec2"));
    matches.result_in_points()
}

pub fn get_points_second_version(is_example: bool) -> u32 {
    let matches = MatchesUpdated::new(get_data_as_string(is_example, "dec2"));
    matches.result_in_points()
}