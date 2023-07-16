use std::collections::HashSet;

use movee::{Direction, Move};

use crate::position::Position;

pub mod movee;
pub mod position;

pub fn process_part_1(input: &str) -> String {
    let moves = input
        .lines()
        .map(|line| Move::new(line))
        .collect::<Vec<Move>>();
    let mut head = Position::new();
    let mut tail = Position::new();
    let mut tail_path: HashSet<String> = HashSet::new();

    for mov in moves.iter() {
        (0..mov.steps().to_owned()).into_iter().for_each(|_| {
            head.mov(mov.direction());
            move_tail(&head, &mut tail);
            tail_path.insert(format!("{}--{}", tail.x(), tail.y()));
        });
    }

    tail_path.len().to_string()
}

pub fn process_part_2(input: &str) -> String {
    let moves = input
        .lines()
        .map(|line| Move::new(line))
        .collect::<Vec<Move>>();
    let mut rope = vec![Position::new(); 10];
    let mut tail_path: HashSet<String> = HashSet::new();

    for mov in moves.iter() {
        (0..mov.steps().to_owned()).into_iter().for_each(|_| {
            rope[0].mov(mov.direction());

            for i in 0..rope.len() - 1 {
                let head = rope[i].clone();
                let tail = &mut rope[i + 1];
                move_tail(&head, tail);
            }

            tail_path.insert(format!("{}--{}", rope[9].x(), rope[9].y()));
        });
    }

    tail_path.len().to_string()
}

pub fn move_tail(head: &Position, tail: &mut Position) {
    let base = (head.x() - tail.x()) as f32;
    let height = (head.y() - tail.y()) as f32;
    let sqare = base.powi(2) + height.powi(2);

    let distance = sqare.sqrt();

    if distance.lt(&(2 as f32)) {
        return;
    }
    if head.y() > tail.y() {
        tail.mov(&Direction::UP);
    }
    if head.x() > tail.x() {
        tail.mov(&Direction::RIGHT);
    }
    if head.y() < tail.y() {
        tail.mov(&Direction::DOWN);
    }
    if head.x() < tail.x() {
        tail.mov(&Direction::LEFT);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn process_part_1_test() {
        let input = "R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2";
        let result = process_part_1(input);
        assert_eq!(result, "13");
    }

    #[test]
    fn process_part_2_test() {
        let input = "R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20";
        let result = process_part_2(input);
        assert_eq!(result, "36");
    }
}
