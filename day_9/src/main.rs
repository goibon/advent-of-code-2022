use std::{collections::HashSet, f64::consts::SQRT_2};

#[derive(Debug)]
enum Move {
    Up(i32),
    Down(i32),
    Left(i32),
    Right(i32),
}

fn touches(head_x: i32, head_y: i32, tail_x: i32, tail_y: i32) -> bool {
    let distance = f64::sqrt(((tail_x - head_x).pow(2) + (tail_y - head_y).pow(2)).into());
    distance <= SQRT_2
}

fn new_tail_position(head_x: i32, head_y: i32, tail_x: i32, tail_y: i32) -> (i32, i32) {
    let mut new_tail_x = tail_x;
    let mut new_tail_y = tail_y;
    let clamped_x_diff = ((head_x - tail_x) as f32).clamp(-1.0, 1.0) as i32;
    let clamped_y_diff = ((head_y - tail_y) as f32).clamp(-1.0, 1.0) as i32;
    if head_x == tail_x {
        new_tail_y += clamped_y_diff;
    } else if head_y == tail_y {
        new_tail_x += clamped_x_diff;
    } else {
        // Diagonal move required
        new_tail_x += clamped_x_diff;
        new_tail_y += clamped_y_diff;
    }
    (new_tail_x, new_tail_y)
}

fn part_1(input: &str) -> usize {
    let mut current_head_position = (0, 0);
    let mut current_tail_position = (0, 0);
    input
        .lines()
        .filter_map(|line| line.split_once(' '))
        .filter_map(|(direction, distance)| {
            if let Ok(distance) = distance.parse::<i32>() {
                Some((direction, distance))
            } else {
                None
            }
        })
        .filter_map(|(direction, distance)| match direction {
            "U" => Some(Move::Up(distance)),
            "D" => Some(Move::Down(distance)),
            "L" => Some(Move::Left(distance)),
            "R" => Some(Move::Right(distance)),
            _ => None,
        })
        .flat_map(|movement| {
            let mut visited_positions = vec![(0, 0)];
            let (mut head_x, mut head_y) = current_head_position;
            let (mut tail_x, mut tail_y) = current_tail_position;
            match movement {
                Move::Up(distance) => {
                    head_y += distance;
                }
                Move::Down(distance) => {
                    head_y -= distance;
                }
                Move::Left(distance) => {
                    head_x -= distance;
                }
                Move::Right(distance) => {
                    head_x += distance;
                }
            };

            while !touches(head_x, head_y, tail_x, tail_y) {
                (tail_x, tail_y) = new_tail_position(head_x, head_y, tail_x, tail_y);
                visited_positions.push((tail_x, tail_y));
            }

            current_head_position = (head_x, head_y);
            current_tail_position = (tail_x, tail_y);
            visited_positions
        })
        .collect::<HashSet<(i32, i32)>>()
        .len()
}

fn part_2(input: &str) -> usize {
    let movements = input
        .lines()
        .filter_map(|line| line.split_once(' '))
        .filter_map(|(direction, distance)| {
            if let Ok(distance) = distance.parse::<i32>() {
                Some((direction, distance))
            } else {
                None
            }
        })
        .filter_map(|(direction, distance)| match direction {
            "U" => Some(Move::Up(distance)),
            "D" => Some(Move::Down(distance)),
            "L" => Some(Move::Left(distance)),
            "R" => Some(Move::Right(distance)),
            _ => None,
        });

    let mut visited_positions: HashSet<(i32, i32)> = HashSet::from([(0, 0)]);
    let mut knots: [(i32, i32); 10] = [(0, 0); 10];
    for movement in movements {
        let (mut target_head_x, mut target_head_y) = knots[0];
        match movement {
            Move::Up(distance) => {
                target_head_y += distance;
            }
            Move::Down(distance) => {
                target_head_y -= distance;
            }
            Move::Left(distance) => {
                target_head_x -= distance;
            }
            Move::Right(distance) => {
                target_head_x += distance;
            }
        };

        while knots[0].0 != target_head_x || knots[0].1 != target_head_y {
            let (head_x, head_y) = knots[0];
            let (head_x, head_y) = new_tail_position(target_head_x, target_head_y, head_x, head_y);
            knots[0] = (head_x, head_y);

            for knot_index in 1..=9 {
                let (head_x, head_y) = knots[knot_index - 1];
                let (mut tail_x, mut tail_y) = knots[knot_index];
                if touches(head_x, head_y, tail_x, tail_y) {
                    // This knot won't move, so neither will any of the subsequent knots
                    break;
                }

                (tail_x, tail_y) = new_tail_position(head_x, head_y, tail_x, tail_y);
                if knot_index == 9 {
                    visited_positions.insert((tail_x, tail_y));
                }
                knots[knot_index] = (tail_x, tail_y);
            }
        }
    }

    visited_positions.len()
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let path = &args[1];
    let input = std::fs::read_to_string(path)
        .unwrap_or_else(|error| panic!("No file found at: '{}'.\n{}", path, error));

    let part_1_result = part_1(&input);
    println!("Day 9 Part 1: {}", part_1_result);

    let part_2_result = part_2(&input);
    println!("Day 9 Part 2: {}", part_2_result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_should_return_the_amount_of_positions_visited() {
        let input = "R 4\nU 4\nL 3\nD 1\nR 4\nD 1\nL 5\nR 2\n\n";

        assert_eq!(part_1(input), 13);
    }

    #[test]
    fn touches_should_return_whether_the_head_and_tail_knots_touch() {
        assert!(touches(0, 0, -1, 1));
        assert!(touches(0, 0, 0, 1));
        assert!(touches(0, 0, 1, 1));
        assert!(touches(0, 0, -1, 0));
        assert!(touches(0, 0, 0, 0));
        assert!(touches(0, 0, 1, 0));
        assert!(touches(0, 0, -1, -1));
        assert!(touches(0, 0, 0, -1));
        assert!(touches(0, 0, 1, -1));
        assert!(!touches(0, 0, 2, 0));
        assert!(!touches(0, 0, 0, 2));
        assert!(!touches(0, 0, -2, 0));
        assert!(!touches(0, 0, 0, -2));
    }

    #[test]
    fn part_2_small_example() {
        let input = "R 4\nU 4\nL 3\nD 1\nR 4\nD 1\nL 5\nR 2\n\n";

        assert_eq!(part_2(input), 1);
    }

    #[test]
    fn part_2_larger_example() {
        let input = "R 5\nU 8\nL 8\nD 3\nR 17\nD 10\nL 25\nU 20\n\n";

        assert_eq!(part_2(input), 36);
    }
}
