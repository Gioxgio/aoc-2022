
// Refactor the logic that parse the function
// re-add the logic related to the worry level
use monkey::Monkey;

mod monkey;

pub fn process_part_1(input: &str) -> String {
    let mut monkeys = input
        .split("\n\n")
        .map(|m| Monkey::new(m, 3.0))
        .collect::<Vec<Monkey>>();

    for _ in 0..20 {
        for i in 0..monkeys.len() {
            let monkey = &mut monkeys[i];
            let mut objects_to_throw: Vec<(usize, i64)> = Vec::new();

            while monkey.can_throw() {
                let item = monkey.throw_object();
                let dest_monkey_name = monkey.choose_monkey(item);
                objects_to_throw.push((dest_monkey_name, item));
            }

            objects_to_throw
                .into_iter()
                .for_each(|(dest_monkey, item)| monkeys[dest_monkey].catch(item));
        }
    }

    let mut inspections = monkeys
        .iter()
        .map(|m| m.inspections())
        .collect::<Vec<i64>>();
    inspections.sort_by(|a, b| b.cmp(a));

    (inspections[0] * inspections[1]).to_string()
}

pub fn process_part_2(input: &str) -> String {
    let mut monkeys = input
        .split("\n\n")
        .map(|m| Monkey::new(m, 1.0))
        .collect::<Vec<Monkey>>();

    for _ in 0..20 {
        for i in 0..monkeys.len() {
            let monkey = &mut monkeys[i];
            let mut objects_to_throw: Vec<(usize, i64)> = Vec::new();

            while monkey.can_throw() {
                let item = monkey.throw_object();
                let dest_monkey_name = monkey.choose_monkey(item);
                objects_to_throw.push((dest_monkey_name, item));
            }

            objects_to_throw
                .into_iter()
                .for_each(|(dest_monkey, item)| monkeys[dest_monkey].catch(item));
        }
    }

    let mut inspections = monkeys
        .iter()
        .map(|m| m.inspections())
        .collect::<Vec<i64>>();
    inspections.sort_by(|a, b| b.cmp(a));

    dbg!(&inspections);
    (inspections[0] * inspections[1]).to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1";

    #[test]
    fn process_part_1_test() {
        let result = process_part_1(INPUT);
        assert_eq!(result, "10605");
    }

    #[test]
    fn process_part_2_test() {
        let result = process_part_2(INPUT);
        assert_eq!(result, "2713310158");
    }
}
