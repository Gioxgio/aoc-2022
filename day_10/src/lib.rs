use std::collections::HashSet;

use instruction::Instruction;

pub mod instruction;

pub fn process_part_1(input: &str) -> String {
    let instructions = input
        .lines()
        .map(|line| Instruction::new(line))
        .collect::<Vec<Instruction>>();
    let mut cycle: i32 = 0;
    let mut i = 0;
    let mut x_value = 1;
    let intersting_indexes = vec![20, 60, 100, 140, 180, 220];
    let mut output = 0;

    loop {
        cycle += 1;

        let instruction = &instructions[i];
        let cycles = instruction.cycles();
        let target_value = instruction.target_value();

        if intersting_indexes.contains(&cycle) {
            output += cycle * x_value;
        }

        if cycles.get() == 1 {
            if let Some(x) = target_value {
                x_value += x;
            }
            i += 1;
            if i == instructions.len() {
                break;
            }
        } else {
            cycles.set(cycles.get() - 1);
        }
    }

    output.to_string()
}

pub fn process_part_2(input: &str) -> String {
    let instructions = input
        .lines()
        .map(|line| Instruction::new(line))
        .collect::<Vec<Instruction>>();
    let mut cycle: i32 = 0;
    let mut i = 0;
    let mut x_value = 1 as i32;
    let mut values: HashSet<i32> = HashSet::new();

    loop {
        if cycle % 40 > x_value - 2 && cycle % 40 < x_value + 2 {
            values.insert(cycle);
        }
        cycle += 1;

        if i == instructions.len() {
            break;
        }

        let instruction = &instructions[i];
        let cycles = instruction.cycles();
        let target_value = instruction.target_value();

        if cycles.get() == 1 {
            let v = target_value.unwrap_or_default();
            x_value += v;
            i += 1;
        } else {
            cycles.set(1);
        }
    }

    for i in 0..cycle - 1 {
        if i % 40 == 0 {
            println!()
        }
        if values.contains(&i) {
            print!("#");
        } else {
            print!(" ");
        }
    }

    "".to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop
";

    #[test]
    fn process_part_1_test() {
        let result = process_part_1(INPUT);
        assert_eq!(result, "13140");
    }

    #[test]
    fn process_part_2_test() {
        let result = process_part_2(INPUT);
        assert_eq!(result, "");
    }
}
