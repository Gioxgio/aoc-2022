use std::{panic, cell::Cell};

#[derive(Debug)]
pub struct Instruction {
    cycles: Cell<i32>,
    target_value: Option<i32>,
}

impl Instruction {
    pub fn new(line: &str) -> Self {
        let mut splitted_line = line.split_whitespace();
        let command = splitted_line.next().unwrap();
        let target_value = if let Some(x) = splitted_line.next() {
            Some(x.parse::<i32>().unwrap())
        } else {
            None
        };

        let raw_cycles = match command {
            "addx" => 2,
            "noop" => 1,
            _ => panic!(),
        };
        let cycles = Cell::new(raw_cycles);

        Self {
            cycles,
            target_value,
        }
    }

    pub fn cycles(&self) -> &Cell<i32> {
        &self.cycles
    }

    pub fn target_value(&self) -> Option<i32> {
        self.target_value
    }
}
