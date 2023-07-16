pub fn process_part_1(input: &str) -> String {
    for window in input
        .chars()
        .into_iter()
        .collect::<Vec<char>>()
        .windows(4)
        .enumerate()
    {
        let (i, window) = window.to_owned();

        let mut fixed = window.to_owned();
        fixed.sort();
        fixed.dedup();
        if fixed.len() == 4 {
            return (i + 4).to_string();
        }
    }

    "ERROR!".to_owned()
}

pub fn process_part_2(input: &str) -> String {
    for window in input
        .chars()
        .into_iter()
        .collect::<Vec<char>>()
        .windows(14)
        .enumerate()
    {
        let (i, window) = window.to_owned();

        let mut fixed = window.to_owned();
        fixed.sort();
        fixed.dedup();
        if fixed.len() == 14 {
            return (i + 14).to_string();
        }
    }

    "ERROR!".to_owned()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn process_part_1_test() {
        let inputs: Vec<&str> = vec![
            "mjqjpqmgbljsphdztnvjfqwrcgsmlb",
            "bvwbjplbgvbhsrlpgdmjqwftvncz",
            "nppdvjthqldpwncqszvftbrmjlhg",
            "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg",
            "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw",
        ];
        let outputs: Vec<&str> = vec!["7", "5", "6", "10", "11"];

        for (i, input) in inputs.iter().enumerate() {
            let output = outputs.get(i).unwrap();
            let result = process_part_1(input.to_owned());
            assert_eq!(result, output.to_owned());
        }
    }

    #[test]
    fn process_part_2_test() {
        let inputs: Vec<&str> = vec![
            "mjqjpqmgbljsphdztnvjfqwrcgsmlb",
            "bvwbjplbgvbhsrlpgdmjqwftvncz",
            "nppdvjthqldpwncqszvftbrmjlhg",
            "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg",
            "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw",
        ];
        let outputs: Vec<&str> = vec!["19", "23", "23", "29", "26"];

        for (i, input) in inputs.iter().enumerate() {
            let output = outputs.get(i).unwrap();
            let result = process_part_2(input.to_owned());
            assert_eq!(result, output.to_owned());
        }
    }
}
