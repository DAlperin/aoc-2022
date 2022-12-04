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

    let mut problem_1 = 0;
    let mut problem_2 = 0;

    for pair in pairs {
        let a = &pair[0];
        let b = &pair[1];
        let a_range: Vec<_> = (a[0]..a[1] + 1).collect();
        let b_range: Vec<_> = (b[0]..b[1] + 1).collect();

        if (a[0] >= b[0] && a[1] <= b[1]) || (b[0] >= a[0] && b[1] <= a[1]) {
            problem_1 = problem_1 + 1;
        }

        if a_range.iter().any(|i| b_range.contains(i)) {
            problem_2 = problem_2 + 1;
        }
    }
    println!("{}\n{}", problem_1, problem_2)
}
