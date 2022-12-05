use itertools::Itertools;
use std::fs;

fn read_input() -> (String, String) {
    let input = fs::read_to_string("./input.txt").unwrap();
    let parts: Vec<_> = input.split_terminator("\n\n").collect();
    let stacks = parts[0].to_string();
    let instructions = parts[1].to_string();
    (stacks, instructions)
}

fn parse_stacks(stacks: String) -> Vec<Vec<char>> {
    let lines: Vec<_> = stacks.split("\n").collect();
    let mut stacks = vec![vec![]];
    for (num, line) in lines.iter().enumerate() {
        if num == lines.len() - 1 {
            break;
        }
        for (idx, val) in line.as_bytes().chunks(4).enumerate() {
            if stacks.len() <= idx {
                stacks.push(vec![]);
            }

            if val[1] != b' ' {
                stacks[idx].push(val[1] as char);
            }
        }
    }
    let new = stacks
        .iter()
        .map(|stack| {
            let mut newstack = stack.clone();
            newstack.reverse();
            newstack
        })
        .collect_vec();
    println!("{:?}", new);
    new
}

#[derive(Debug, Clone, Copy)]
struct Instruction {
    from: u32,
    to: u32,
    quantity: u32,
}

fn parse_instructions(input: String) -> Vec<Instruction> {
    let lines: Vec<_> = input.split("\n").collect();
    let mut instructions: Vec<Instruction> = vec![];
    for line in lines {
        let parts: Vec<_> = line.split(" ").collect();
        instructions.push(Instruction {
            quantity: parts[1].parse::<u32>().unwrap(),
            from: parts[3].parse::<u32>().unwrap(),
            to: parts[5].parse::<u32>().unwrap(),
        })
    }
    instructions
}

fn execute_instructions_part_1(
    mut stacks: Vec<Vec<char>>,
    instructions: Vec<Instruction>,
) -> Vec<Vec<char>> {
    for instr in instructions {
        let mut i = 0;
        while i < instr.quantity {
            let val = stacks[instr.from as usize - 1].pop().unwrap();
            stacks[instr.to as usize - 1].push(val);
            i = i + 1;
        }
    }
    stacks
}

fn execute_instructions_part_2(
    mut stacks: Vec<Vec<char>>,
    instructions: Vec<Instruction>,
) -> Vec<Vec<char>> {
    for instr in instructions {
        let mut i: usize = 0;
        while i < instr.quantity as usize {
            let index = stacks[instr.from as usize - 1].len() + i - instr.quantity as usize;
            let val = stacks[instr.from as usize - 1][index];
            stacks[instr.from as usize - 1].remove(index);
            stacks[instr.to as usize - 1].push(val);
            i = i + 1;
        }
    }
    stacks
}

fn main() {
    let (raw_stacks, raw_instructions) = read_input();

    let stacks = parse_stacks(raw_stacks.clone());
    let instructions = parse_instructions(raw_instructions.clone());

    let part_1 = execute_instructions_part_1(stacks.clone(), instructions.clone());
    let mut output_part_1 = String::from("");
    for stack in &part_1 {
        output_part_1.push(*stack.last().unwrap());
    }

    let part_2 = execute_instructions_part_2(stacks, instructions);
    let mut output_part_2 = String::from("");
    for stack in &part_2 {
        output_part_2.push(*stack.last().unwrap());
    }

    println!("{}\n{}", output_part_1, output_part_2);
}
