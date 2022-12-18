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

fn part_1(input: &str) -> usize {
    let moves = input
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

    let mut visited_positions = HashSet::<(i32, i32)>::from([(0, 0)]);
    let mut current_head_position = (0, 0);
    let mut current_tail_position = (0, 0);
    for movement in moves {
        let (mut head_x, mut head_y) = current_head_position;
        let (mut tail_x, mut tail_y) = current_tail_position;
        let mut x_step = 0;
        let mut y_step = 0;
        match movement {
            Move::Up(distance) => {
                head_y += distance;
                y_step = 1;
            }
            Move::Down(distance) => {
                head_y -= distance;
                y_step = -1;
            }
            Move::Left(distance) => {
                head_x -= distance;
                x_step = -1;
            }
            Move::Right(distance) => {
                head_x += distance;
                x_step = 1;
            }
        };

        while !touches(head_x, head_y, tail_x, tail_y) {
            tail_x += x_step;
            tail_y += y_step;
            // Diagonal move required
            if head_x != tail_x && head_y != tail_y {
                if matches!(movement, Move::Up(_)) || matches!(movement, Move::Down(_)) {
                    tail_x += head_x - tail_x;
                } else {
                    tail_y += head_y - tail_y;
                }
            }
            visited_positions.insert((tail_x, tail_y));
        }

        current_head_position = (head_x, head_y);
        current_tail_position = (tail_x, tail_y);
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
}
