use std::fs;

fn find_score(pair: &str, our_move: &str) -> i32 {
    let win_score = match pair {
        "AX" => 3, //Rock-Rock
        "AY" => 6, //Rock-paper
        "AZ" => 0, //Rock-scissors
        "BX" => 0, //Paper-rock
        "BY" => 3, //Paper-paper
        "BZ" => 6, //Paper-scissors
        "CX" => 6, //Scissors-Rock
        "CY" => 0, //Scissors-Paper
        "CZ" => 3, //Scissors-Scissors
        _ => 0,
    };
    let choose_score = match our_move {
        "X" => 1,
        "Y" => 2,
        "Z" => 3,
        _ => 0,
    };
    win_score + choose_score
}

fn main() {
    let input = fs::read_to_string("./input.txt").unwrap();
    let lines: Vec<_> = input.split_terminator("\n").collect();

    let rounds: Vec<Vec<_>> = lines
        .iter()
        .map(|round| round.split(" ").collect())
        .collect();

    let mut problem_1 = 0;
    let mut problem_2 = 0;
    for round in rounds.clone() {
        let chosen_move = match round.join("").as_str() {
            "AX" => "Z",
            "AY" => "X",
            "AZ" => "Y",
            "BX" => "X",
            "BY" => "Y",
            "BZ" => "Z",
            "CX" => "Y",
            "CY" => "Z",
            "CZ" => "X",
            _ => "",
        };

        let new_move = [round[0], chosen_move].join("");
        problem_1 = problem_1 + find_score(round.join("").as_str(), round[1]);
        problem_2 = problem_2 + find_score(new_move.as_str(), chosen_move);
    }
    println!("{}\n{}", problem_1, problem_2);
}
