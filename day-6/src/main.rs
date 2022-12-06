use std::fs;

fn has_dup<T: PartialEq>(slice: &[T]) -> bool {
    for i in 1..slice.len() {
        if slice[i..].contains(&slice[i - 1]) {
            return true;
        }
    }
    false
}

fn main() {
    let input = fs::read_to_string("./input.txt").unwrap();
    let chars: Vec<_> = input.chars().collect();

    // Part 1
    for i in 0..chars.len() {
        let offset = 4;
        if i + offset > chars.len() {
            break;
        }
        let slice = &chars[i..i + offset];
        if !has_dup(slice) {
            println!("{}", i + offset);
            break;
        }
    }

    // Part 2
    for i in 0..chars.len() {
        let offset = 14;
        if i + offset > chars.len() {
            break;
        }
        let slice = &chars[i..i + offset];
        if !has_dup(slice) {
            println!("{}", i + offset);
            break;
        }
    }
}
