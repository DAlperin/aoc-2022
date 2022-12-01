use std::fs;

fn get_elves() -> Vec<u32> {
    let input = fs::read_to_string("./input.txt").unwrap();
    let elves: Vec<_> = input.split_terminator("\n\n").collect();
    let mut elf_cals: Vec<u32> = Vec::new();
    for elf in elves {
        elf_cals.push(elf.lines().map(|cal| cal.parse::<u32>().unwrap()).sum());
    }
    elf_cals
}

fn main() {
    let elf_cals = get_elves();
    // Part one
    println!("{}", elf_cals.iter().max().unwrap());

    // Part two
    let mut sorted = elf_cals.clone();
    sorted.sort();
    println!("{}", sorted.iter().rev().take(3).sum::<u32>())
}
