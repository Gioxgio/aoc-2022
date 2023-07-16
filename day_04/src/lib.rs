pub fn process_part_1(input: &str) -> String {
    input
        .lines()
        .filter(|assignments| {
            let pairs = assignments.split(",").collect::<Vec<&str>>();
            let a = pairs[0]
                .split("-")
                .map(|v| v.parse::<u32>().unwrap())
                .collect::<Vec<u32>>();
            let b = pairs[1]
                .split("-")
                .map(|v| v.parse::<u32>().unwrap())
                .collect::<Vec<u32>>();

            a[0] <= b[0] && a[1] >= b[1] || b[0] <= a[0] && b[1] >= a[1]
        })
        .count()
        .to_string()
}

pub fn process_part_2(input: &str) -> String {
    input
        .lines()
        .filter(|assignments| {
            let pairs = assignments.split(",").collect::<Vec<&str>>();
            let a = pairs[0]
                .split("-")
                .map(|v| v.parse::<u32>().unwrap())
                .collect::<Vec<u32>>();
            let b = pairs[1]
                .split("-")
                .map(|v| v.parse::<u32>().unwrap())
                .collect::<Vec<u32>>();

            a[0] <= b[0] && a[1] >= b[0] || b[0] <= a[0] && b[1] >= a[0]
        })
        .count()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";

    #[test]
    fn process_part_1_test() {
        let result = process_part_1(INPUT);
        assert_eq!(result, "2");
    }

    #[test]
    fn process_part_2_test() {
        let result = process_part_2(INPUT);
        assert_eq!(result, "4");
    }
}
