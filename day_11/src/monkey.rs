use std::collections::LinkedList;

use eval::Expr;

#[derive(Debug)]
pub struct Monkey {
    _name: i64,
    items: LinkedList<i64>,
    function: String,
    test: i64,
    test_true: i64,
    test_false: i64,
    inspections: i64,
    wl: f64,
}

impl Monkey {
    pub fn new(input: &str, wl: f64) -> Self {
        let mut lines = input.lines();
        let _name = lines
            .next()
            .unwrap()
            .split_whitespace()
            .last()
            .unwrap()
            .replace(":", "")
            .parse::<i64>()
            .unwrap();
        let items = lines
            .next()
            .unwrap()
            .split(": ")
            .last()
            .unwrap()
            .split(", ")
            .map(|e| e.parse::<i64>().unwrap())
            .collect::<LinkedList<i64>>();
        let function = lines.next().unwrap().split("= ").last().unwrap().to_owned();
        let test = lines
            .next()
            .unwrap()
            .split("by ")
            .last()
            .map(|c| c.parse::<i64>().unwrap())
            .unwrap() as i64;
        let test_true = lines
            .next()
            .unwrap()
            .chars()
            .last()
            .map(|c| c.to_digit(10).unwrap())
            .unwrap() as i64;
        let test_false = lines
            .next()
            .unwrap()
            .chars()
            .last()
            .map(|c| c.to_digit(10).unwrap())
            .unwrap() as i64;
        let inspections = i64::default();
        Self {
            _name,
            items,
            function,
            test,
            test_true,
            test_false,
            inspections,
            wl,
        }
    }

    pub fn can_throw(&self) -> bool {
        !self.items.is_empty()
    }

    pub fn throw_object(&mut self) -> i64 {
        let item_to_throw = self.items.pop_front().unwrap();
        self.inspect(item_to_throw)
    }

    pub fn choose_monkey(&self, item: i64) -> usize {
        if item % self.test == 0 {
            self.test_true as usize
        } else {
            self.test_false as usize
        }
    }

    pub fn catch(&mut self, item: i64) {
        self.items.push_back(item);
    }

    pub fn inspections(&self) -> i64 {
        self.inspections
    }

    fn inspect(&mut self, item: i64) -> i64 {
        
        
        dbg!(&self.function);
        dbg!(item);

        let new_wl = Expr::new(&self.function)
            .value("old", item)
            .exec()
            .unwrap()
            .as_u64()
            .unwrap();
        self.inspections += 1;
        new_wl as i64
    }
}
