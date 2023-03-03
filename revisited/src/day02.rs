
type Score = u32;
enum Move {
    Rock,
    Paper,
    Scissor
}

enum Outcome {
    Win,
    Draw,
    Loss
}

fn round(my_move: &Move, other_move: &Move) -> Score {
    let outcome = play(my_move, other_move);
    return round_score(my_move, &   outcome);
}


fn round2(other_move: &Move, outcome: &Outcome) -> Score {
    let my_move = find_move(other_move, outcome);
    return round_score(&my_move, outcome);
}

fn play(my_move: &Move, other_move: &Move) -> Outcome {
    match (my_move, other_move) {
        (Move::Rock, Move::Rock) => Outcome::Draw,
        (Move::Rock, Move::Paper) => Outcome::Loss,
        (Move::Rock, Move::Scissor) => Outcome::Win,
        (Move::Paper, Move::Rock) => Outcome::Win,
        (Move::Paper, Move::Paper) => Outcome::Draw,
        (Move::Paper, Move::Scissor) => Outcome::Loss,
        (Move::Scissor, Move::Rock) => Outcome::Loss,
        (Move::Scissor, Move::Paper) => Outcome::Win,
        (Move::Scissor, Move::Scissor) => Outcome::Draw
    }
}

fn find_move(other_move: &Move, outcome: &Outcome) -> Move {
    match (other_move, outcome) {
        (Move::Rock, Outcome::Loss) => Move::Scissor,
        (Move::Rock, Outcome::Draw) => Move::Rock,
        (Move::Rock, Outcome::Win) => Move::Paper,
        (Move::Paper, Outcome::Loss) => Move::Rock,
        (Move::Paper, Outcome::Draw) => Move::Paper,
        (Move::Paper, Outcome::Win) => Move::Scissor,
        (Move::Scissor, Outcome::Loss) => Move::Paper,
        (Move::Scissor, Outcome::Draw) => Move::Scissor,
        (Move::Scissor, Outcome::Win) => Move::Rock
    }
}


fn round_score(my_move: &Move, outcome: &Outcome) -> Score {
    let move_score: Score = match my_move {
        Move::Rock => 1,
        Move::Paper => 2,
        Move::Scissor => 3
    };

    let outcome_score: Score = match outcome {
        Outcome::Loss => 0,
        Outcome::Draw => 3,
        Outcome::Win => 6
    };
    
    return move_score + outcome_score;
}

fn parse_move(c: char) -> Move {
    match c {
        'A' | 'X' => Move::Rock,
        'B' | 'Y' => Move::Paper,
        'C' | 'Z' => Move::Scissor,
        _ => panic!("Not valid char")
    }
}

fn parse_outcome(c: char) -> Outcome {
    match c {
        'X' => Outcome::Loss,
        'Y' => Outcome::Draw,
        'Z' => Outcome::Win,
        _ => panic!("Not valid char")
    }
}

fn parse_line_1(line: &str) -> (Move, Move) {
    let mut char_iter = line.chars();
    let move_1 = parse_move(char_iter.next().unwrap());
    char_iter.next();
    let move_2 = parse_move(char_iter.next().unwrap());
    return (move_1, move_2)
}

fn parse_line_2(line: &str) -> (Move, Outcome) {
    let mut char_iter = line.chars();
    let move_1 = parse_move(char_iter.next().unwrap());
    char_iter.next();
    let outcome = parse_outcome(char_iter.next().unwrap());
    return (move_1, outcome)
}

pub fn task1(input: &str) -> String {
    input.lines()
        .map(|line| parse_line_1(line))
        .map(|(m1, m2)| round(&m2, &m1))
        .sum::<Score>()
        .to_string()
}

pub fn task2(input: &str) -> String {
    input.lines()
    .map(|line| parse_line_2(line))
    .map(|(m1, outcome)| round2(&m1, &outcome))
    .sum::<Score>()
    .to_string()
}











