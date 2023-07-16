use std::collections::{HashMap, LinkedList};

pub struct CrateWrapper {
    crates: HashMap<u32, LinkedList<String>>,
}

pub struct Instruction {
    amount: u32,
    from: u32,
    to: u32,
}

impl CrateWrapper {
    pub fn new(input: &str) -> Self {
        let size = Self::get_size(input);
        let mut crates = Self::init_map(size);

        input.lines().for_each(|line| {
            let _a = line
                .chars()
                .collect::<Vec<char>>()
                .chunks(4)
                .enumerate()
                .for_each(|(n, c)| {
                    let chunk = c.to_owned().iter().collect::<String>().trim().to_owned();
                    if !chunk.is_empty() && chunk.contains('[') {
                        let clean_chunk = chunk.replace('[', "").replace(']', "");
                        crates.get_mut(&(n as u32)).unwrap().push_front(clean_chunk);
                    }
                });
        });

        Self { crates }
    }

    pub fn crates(&self) -> &HashMap<u32, LinkedList<String>> {
        &self.crates
    }

    pub fn crates_mut(&mut self) -> &mut HashMap<u32, LinkedList<String>> {
        &mut self.crates
    }

    fn get_size(input: &str) -> u32 {
        input
            .lines()
            .last()
            .unwrap()
            .split(" ")
            .filter(|c| !c.to_owned().is_empty())
            .last()
            .map(|v| v.parse::<u32>().unwrap())
            .unwrap()
    }

    fn init_map(size: u32) -> HashMap<u32, LinkedList<String>> {
        let mut map: HashMap<u32, LinkedList<String>> = HashMap::new();

        for i in 0..size {
            map.insert(i, LinkedList::new());
        }
        map
    }
}

impl Instruction {
    pub fn new(line: &str) -> Self {
        let instr = line.split_whitespace().collect::<Vec<&str>>();
        let amount = instr[1].parse::<u32>().unwrap();
        let from = instr[3].parse::<u32>().unwrap() - 1;
        let to = instr[5].parse::<u32>().unwrap() - 1;
        Self { amount, from, to }
    }

    pub fn amount(&self) -> &u32 {
        &self.amount
    }

    pub fn from(&self) -> &u32 {
        &self.from
    }

    pub fn to(&self) -> &u32 {
        &self.to
    }
}
