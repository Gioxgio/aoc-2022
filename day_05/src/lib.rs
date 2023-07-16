use std::collections::{HashMap, LinkedList};

use input_wrapper::{CrateWrapper, Instruction};

pub mod input_wrapper;

pub fn process_part_1(input: &str) -> String {
    let inputs = input.split("\n\n").collect::<Vec<&str>>();
    let mut crate_wrapper = CrateWrapper::new(inputs[0]);
    let crates = crate_wrapper.crates_mut();

    let instructions = inputs[1]
        .lines()
        .map(|line| Instruction::new(line))
        .collect::<Vec<Instruction>>();

    process_instructions(crates, &instructions);

    (0..crates.len())
        .into_iter()
        .map(|i| crates.get_mut(&(i as u32)).unwrap().pop_back().unwrap())
        .collect::<String>()
}

pub fn process_part_2(input: &str) -> String {
    let inputs = input.split("\n\n").collect::<Vec<&str>>();
    let mut crate_wrapper = CrateWrapper::new(inputs[0]);
    let crates = crate_wrapper.crates_mut();

    let instructions = inputs[1]
        .lines()
        .map(|line| Instruction::new(line))
        .collect::<Vec<Instruction>>();

    process_instructions_2(crates, &instructions);

    (0..crates.len())
        .into_iter()
        .map(|i| crates.get_mut(&(i as u32)).unwrap().pop_back().unwrap())
        .collect::<String>()
}

pub fn process_instructions(
    crates: &mut HashMap<u32, LinkedList<String>>,
    instructions: &Vec<Instruction>,
) {
    instructions.iter().for_each(|instruction| {
        let amount = instruction.amount().to_owned();
        let from = instruction.from();
        let to = instruction.to();

        for _ in 0..amount {
            let from_value = crates.get_mut(from).unwrap().pop_back().unwrap();
            crates.get_mut(to).unwrap().push_back(from_value)
        }
    });
}

pub fn process_instructions_2(
    crates: &mut HashMap<u32, LinkedList<String>>,
    instructions: &Vec<Instruction>,
) {
    instructions.iter().for_each(|instruction| {
        let amount = instruction.amount().to_owned();
        let from = instruction.from();
        let to = instruction.to();

        let strings = (0..amount)
            .into_iter()
            .map(|_| crates.get_mut(from).unwrap().pop_back().unwrap())
            .collect::<Vec<String>>();

        strings
            .iter()
            .rev()
            .for_each(|c| crates.get_mut(to).unwrap().push_back(c.to_owned()))
    });
}
#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

 move 1 from 2 to 1
 move 3 from 1 to 3
 move 2 from 2 to 1
 move 1 from 1 to 2";

    #[test]
    fn process_part_1_test() {
        let result = process_part_1(INPUT);
        assert_eq!(result, "CMZ");
    }

    #[test]
    fn process_part_2_test() {
        let result = process_part_2(INPUT);
        assert_eq!(result, "MCD");
    }
}
