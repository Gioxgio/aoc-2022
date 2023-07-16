#[derive(Debug)]
pub enum Direction {
    UP,
    RIGHT,
    DOWN,
    LEFT,
}
impl std::str::FromStr for Direction {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "U" => Ok(Direction::UP),
            "R" => Ok(Direction::RIGHT),
            "D" => Ok(Direction::DOWN),
            "L" => Ok(Direction::LEFT),
            _ => Err(format!("'{}' is not a valid value for Direction", s)),
        }
    }
}

#[derive(Debug)]
pub struct Move {
    direction: Direction,
    steps: i32,
}

impl Move {
    pub fn new(line: &str) -> Move {
        let mut splitted_line = line.split_whitespace();
        let direction = splitted_line.next().unwrap().parse::<Direction>().unwrap();
        let steps = splitted_line.next().unwrap().parse::<i32>().unwrap();

        Move { direction, steps }
    }

    pub fn direction(&self) -> &Direction {
        &self.direction
    }

    pub fn steps(&self) -> &i32 {
        &self.steps
    }
}
