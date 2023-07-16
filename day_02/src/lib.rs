pub fn process_part_1(input: &str) -> String {
    input.lines().map(process_round_1).sum::<u32>().to_string()
}

pub fn process_part_2(input: &str) -> String {
    input.lines().map(process_round_2).sum::<u32>().to_string()
}

fn process_round_1(round: &str) -> u32 {
    let winning_conditions = vec!["C X", "A Y", "B Z"];
    let draw_conditions = vec!["A X", "B Y", "C Z"];
    let mut partial_score = 0;
    if winning_conditions.contains(&round) {
        partial_score += 6
    } else if draw_conditions.contains(&round) {
        partial_score += 3
    }

    partial_score += match round.chars().nth(2).unwrap() {
        'X' => 1,
        'Y' => 2,
        'Z' => 3,
        _ => panic!(),
    };

    partial_score
}

fn process_round_2(round: &str) -> u32 {
    let mut partial_score = 0;

    let opponent_move = round.chars().nth(0).unwrap();
    let my_move = round.chars().nth(2).unwrap();

    partial_score += match my_move {
        'Y' => {
            3 + match opponent_move {
                'A' => 1,
                'B' => 2,
                'C' => 3,
                _ => 0,
            }
        }
        'Z' => {
            6 + match opponent_move {
                'A' => 2,
                'B' => 3,
                'C' => 1,
                _ => 0,
            }
        }
        _ => {
            0 + match opponent_move {
                'A' => 3,
                'B' => 1,
                'C' => 2,
                _ => 0,
            }
        }
    };

    partial_score += match opponent_move {
        'Y' => 3,
        'Z' => 6,
        _ => 0,
    };

    partial_score
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "A Y
B X
C Z";

    #[test]
    fn process_part_1_test() {
        let result = process_part_1(INPUT);
        assert_eq!(result, "15");
    }

    #[test]
    fn process_part_2_test() {
        let result = process_part_2(INPUT);
        assert_eq!(result, "12");
    }
}
