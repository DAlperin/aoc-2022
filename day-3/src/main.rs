use std::collections::HashSet;
use std::fs;

fn main() {
    let alphabet = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";
    let input = fs::read_to_string("./input.txt").unwrap();
    let lines: Vec<_> = input.split_terminator("\n").collect();
    let packs: Vec<_> = lines
        .iter()
        .map(|line| line.split_at(line.len() / 2))
        .collect();

    let mut out = 0;
    for pack in packs.clone() {
        let mut same: char = ' ';
        let (first, last) = pack;
        for char in first.chars() {
            for second_char in last.chars() {
                if char == second_char {
                    same = char;
                }
            }
        }
        out = out + alphabet.chars().position(|c| c == same).unwrap() + 1
    }
    println!("{}", out);

    let mut out2 = 0;
    for pack in lines.chunks(3) {
        let mut a: HashSet<char> = pack[0].chars().collect();
        let b: HashSet<char> = pack[1].chars().collect();
        let c: HashSet<char> = pack[2].chars().collect();

        let intersection = &mut a;
        for other in [b, c] {
            intersection.retain(|e| other.contains(e));
        }

        let char = intersection.iter().nth(0).unwrap();
        out2 = out2 + alphabet.chars().position(|c| c == *char).unwrap() + 1;
    }
    println!("{}", out2);
}
