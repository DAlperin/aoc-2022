use std::fs;

fn main() {
    let input = fs::read_to_string("./input.txt").unwrap();
    let lines: Vec<_> = input.split_terminator("\n").collect();
    let pairs: Vec<Vec<Vec<_>>> = lines
        .iter()
        .map(|line| {
            line.split(",")
                .map(|item| {
                    item.split("-")
                        .map(|item| item.parse::<u32>().unwrap())
                        .collect()
                })
                .collect()
        })
        .collect();

    let part_1: Vec<_> = pairs
        .iter()
        .filter(|pair| {
            let a = &pair[0];
            let b = &pair[1];
            (a[0] >= b[0] && a[1] <= b[1]) || (b[0] >= a[0] && b[1] <= a[1])
        })
        .collect();

    let part_2: Vec<_> = pairs
        .iter()
        .filter(|pair| {
            let a = &pair[0];
            let b = &pair[1];
            let a_range: Vec<_> = (a[0]..a[1] + 1).collect();
            let b_range: Vec<_> = (b[0]..b[1] + 1).collect();
            a_range.iter().any(|i| b_range.contains(i))
        })
        .collect();

    println!("{}\n{}", part_1.len(), part_2.len());
}
