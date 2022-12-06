use std::fs;

fn has_dup<T: PartialEq>(slice: &[T]) -> bool {
    for i in 1..slice.len() {
        if slice[i..].contains(&slice[i - 1]) {
            return true;
        }
    }
    false
}

fn solve(chars: &Vec<char>, offset: usize) -> Option<usize> {
    let mut output: Option<usize> = None;
    for i in 0..chars.len() {
        if i + offset > chars.len() {
            break;
        }
        let slice = &chars[i..i + offset];
        if !has_dup(slice) {
            output = Some(i + offset);
            break;
        }
    }
    output
}

fn main() {
    let input = fs::read_to_string("./input.txt").unwrap();
    let chars: Vec<_> = input.chars().collect();

    println!("{}", solve(&chars, 4).unwrap());
    println!("{}", solve(&chars, 14).unwrap());
}
