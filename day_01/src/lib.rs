pub fn process_part_1(input: &str) -> String {
    input
        .split("\n\n")
        .map(|load| {
            load.lines()
                .map(|k| k.trim().parse::<u32>().unwrap())
                .sum::<u32>()
        })
        .max()
        .unwrap()
        .to_string()
}

pub fn process_part_2(input: &str) -> String {
    let mut ordered_loads = input
        .split("\n\n")
        .map(|load| {
            load.lines()
                .map(|k| k.trim().parse::<u32>().unwrap())
                .sum::<u32>()
        })
        .collect::<Vec<u32>>();
    ordered_loads.sort_by(|a, b| b.cmp(a));
    ordered_loads.iter().take(3).sum::<u32>().to_string()
}


#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "1000
2000
3000

4000

5000
6000

7000
8000
9000

10000";

    #[test]
    fn process_part_1_test() {
        let result = process_part_1(INPUT);
        assert_eq!(result, "24000");
    }

    #[test]
    fn process_part_2_test() {
        let result = process_part_2(INPUT);
        assert_eq!(result, "45000");
    }
}
