pub fn process_part_1(input: &str) -> String {
    input
        .lines()
        .map(|sack| {
            let (first, second) = sack.split_at(sack.len() / 2);
            let duplicate_char = first.chars().find(|c| second.contains(c.to_owned()));
            if let Some(c) = duplicate_char {
                return c as u32 - if c.is_uppercase() { 38 } else { 96 };
            }
            0
        })
        .sum::<u32>()
        .to_string()
}

pub fn process_part_2(input: &str) -> String {
    input
        .lines()
        .collect::<Vec<&str>>()
        .chunks(3)
        .map(|lines| {
            let duplicate_char = lines[0]
                .chars()
                .find(|c| lines[1].contains(c.to_owned()) && lines[2].contains(c.to_owned()));
            if let Some(c) = duplicate_char {
                return c as u32 - if c.is_uppercase() { 38 } else { 96 };
            }
            0
        })
        .sum::<u32>()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";

    #[test]
    fn process_part_1_test() {
        let result = process_part_1(INPUT);
        assert_eq!(result, "157");
    }

    #[test]
    fn process_part_2_test() {
        let result = process_part_2(INPUT);
        assert_eq!(result, "70");
    }
}
