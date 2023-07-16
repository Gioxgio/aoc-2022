pub fn process_part_1(input: &str) -> String {
    let tree_map = input
        .lines()
        .map(|l| {
            l.chars()
                .map(|c| c.to_string().parse::<i32>().unwrap())
                .collect::<Vec<i32>>()
        })
        .collect::<Vec<Vec<i32>>>();
    let mut visible_trees: Vec<Vec<bool>> = vec![vec![false; tree_map[0].len()]; tree_map.len()];

    // From left
    tree_map.iter().enumerate().for_each(|(y, t)| {
        let mut highest: i32 = -1;
        t.iter().enumerate().for_each(|(x, t)| {
            if t.gt(&highest) {
                highest = t.clone();
                visible_trees[y][x.to_owned()] = true;
            }
        });
    });

    // From right
    tree_map.iter().enumerate().for_each(|(y, t)| {
        let mut highest: i32 = -1;
        let mut owned = t.to_owned();
        owned.reverse();
        owned.iter().enumerate().for_each(|(x, t)| {
            if t.gt(&highest) {
                highest = t.clone();
                visible_trees[y][owned.len() - 1 - x.to_owned()] = true;
            }
        });
    });

    // From top
    for i in 0..tree_map[0].len() {
        let mut highest: i32 = -1;
        for j in 0..tree_map.len() {
            let t = tree_map[j][i];
            if t.gt(&highest) {
                highest = t.clone();
                visible_trees[j][i] = true;
            }
        }
    }

    // From bottom
    for i in (0..tree_map[0].len()).rev() {
        let mut highest: i32 = -1;
        for j in (0..tree_map.len()).rev() {
            let t = tree_map[j][i];
            if t.gt(&highest) {
                highest = t.clone();
                visible_trees[j][i] = true;
            }
        }
    }

    visible_trees
        .iter()
        .fold(0, |mut acc, t| {
            acc += t.iter().fold(0, |mut acc, b| {
                if b.to_owned() {
                    acc += 1;
                }
                acc
            });
            acc
        })
        .to_string()
}

pub fn process_part_2(input: &str) -> String {
    let tree_map = input
        .lines()
        .map(|l| {
            l.chars()
                .map(|c| c.to_string().parse::<i32>().unwrap())
                .collect::<Vec<i32>>()
        })
        .collect::<Vec<Vec<i32>>>();

    tree_map
        .iter()
        .enumerate()
        .map(|(x, row)| {
            row.iter()
                .enumerate()
                .map(|(y, t)| calculate_score(&tree_map, x, y, t))
                .max()
                .unwrap()
        })
        .max()
        .unwrap()
        .to_string()
}

fn calculate_score(tree_map: &Vec<Vec<i32>>, mut x: usize, mut y: usize, height: &i32) -> i32 {
    let initial_x = x;
    let initial_y = y;

    let mut plus_x = tree_map.len().ne(&(x + 1));
    let mut minus_x = (0 as usize).ne(&x);
    let mut plus_y = tree_map[x].len().ne(&(y + 1));
    let mut minus_y = (0 as usize).ne(&y);

    let mut incr_x = 0;
    let mut decr_x = 0;
    let mut incr_y = 0;
    let mut decr_y = 0;

    loop {
        if plus_x {
            x += 1;
            incr_x += 1;
            let temp_t = tree_map[x][y];

            if &temp_t >= height || tree_map.len().eq(&(x + 1)) {
                plus_x = false;
                x = initial_x;
            }
        } else if minus_x {
            x -= 1;
            decr_x += 1;
            let temp_t = tree_map[x][y];
            if &temp_t >= height || (0 as usize).eq(&(x)) {
                minus_x = false;
                x = initial_x;
            }
        } else if plus_y {
            y += 1;
            incr_y += 1;
            let temp_t = tree_map[x][y];
            if &temp_t >= height || tree_map[x].len().eq(&(y + 1)) {
                plus_y = false;
                y = initial_y;
            }
        } else if minus_y {
            y -= 1;
            decr_y += 1;
            let temp_t = tree_map[x][y];
            if &temp_t >= height || (0 as usize).eq(&(y)) {
                minus_y = false;
                y = initial_y;
            }
        } else {
            break;
        }
    }

    (incr_x * incr_y * decr_x * decr_y) as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "30373
25512
65332
33549
35390";

    #[test]
    fn process_part_1_test() {
        let result = process_part_1(INPUT);
        assert_eq!(result, "21");
    }

    #[test]
    fn process_part_2_test() {
        let result = process_part_2(INPUT);
        assert_eq!(result, "8");
    }
}
