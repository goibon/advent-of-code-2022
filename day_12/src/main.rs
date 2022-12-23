use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashMap},
};

#[derive(Copy, Clone, Eq, PartialEq)]
struct State {
    cost: usize,
    position: (usize, usize),
}

// The priority queue depends on `Ord`.
// Explicitly implement the trait so the queue becomes a min-heap
// instead of a max-heap.
impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        // Notice that the we flip the ordering on costs.
        // In case of a tie we compare positions - this step is necessary
        // to make implementations of `PartialEq` and `Ord` consistent.
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| self.position.cmp(&other.position))
    }
}

// `PartialOrd` needs to be implemented as well.
impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn convert_character(character: char) -> u8 {
    if character == 'S' {
        b'a'
    } else if character == 'E' {
        b'z'
    } else {
        character as u8
    }
}

fn can_move_to_character(current_character: char, neighbor_character: char) -> bool {
    let current_character = convert_character(current_character);
    let neighbor_character = convert_character(neighbor_character);

    neighbor_character <= current_character || neighbor_character == current_character + 1
}

fn get_adjacent_indices(
    y: usize,
    x: usize,
    max_y: usize,
    max_x: usize,
    map: &[Vec<char>],
) -> Vec<(usize, usize)> {
    let mut adjacent_indices: Vec<(usize, usize)> = Vec::new();
    let current_character = map[y][x];

    if y > 0 {
        let neighbor_character = map[y - 1][x];

        if can_move_to_character(current_character, neighbor_character) {
            adjacent_indices.push((y - 1, x))
        }
    };
    if y < max_y {
        let neighbor_character = map[y + 1][x];
        if can_move_to_character(current_character, neighbor_character) {
            adjacent_indices.push((y + 1, x))
        }
    };
    if x > 0 {
        let neighbor_character = map[y][x - 1];
        if can_move_to_character(current_character, neighbor_character) {
            adjacent_indices.push((y, x - 1))
        }
    };
    if x < max_x {
        let neighbor_character = map[y][x + 1];
        if can_move_to_character(current_character, neighbor_character) {
            adjacent_indices.push((y, x + 1))
        }
    };

    adjacent_indices
}

fn dijkstras(
    input: &[Vec<char>],
    start_position: (usize, usize),
    target: (usize, usize),
) -> Option<usize> {
    let (target_y, target_x) = target;
    let max_y = input.len() - 1;
    let max_x = input[0].len() - 1;

    let mut nodes_to_explore = BinaryHeap::new();
    nodes_to_explore.push(State {
        cost: 0,
        position: start_position,
    });
    let mut f_score: HashMap<(usize, usize), u32> = HashMap::from([(start_position, 0)]);

    while let Some(State { cost, position }) = nodes_to_explore.pop() {
        if position == (target_y, target_x) {
            return Some(cost);
        }

        if cost > *f_score.entry(position).or_insert(u32::MAX) as usize {
            continue;
        }

        for (neighbor_y, neighbor_x) in
            get_adjacent_indices(position.0, position.1, max_y, max_x, input)
        {
            let next = State {
                cost: cost + 1,
                position: (neighbor_y, neighbor_x),
            };

            let f_score_entry = f_score.entry(next.position).or_insert(u32::MAX);
            if next.cost < *f_score_entry as usize {
                nodes_to_explore.push(next);
                // Relaxation, we have now found a better way
                *f_score_entry = next.cost as u32;
            }
        }
    }

    None
}

fn part_1(input: &str) -> usize {
    let map: Vec<Vec<char>> = input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| line.chars().collect())
        .collect();
    let end_point = map
        .iter()
        .enumerate()
        .find_map(|(y_index, chars)| {
            chars
                .iter()
                .enumerate()
                .find_map(|(x_index, character)| {
                    if *character == 'E' {
                        Some(x_index)
                    } else {
                        None
                    }
                })
                .map(|x_index| (y_index, x_index))
        })
        .unwrap();
    let start_point = map
        .iter()
        .enumerate()
        .find_map(|(y_index, chars)| {
            chars
                .iter()
                .enumerate()
                .find_map(|(x_index, character)| {
                    if *character == 'S' {
                        Some(x_index)
                    } else {
                        None
                    }
                })
                .map(|x_index| (y_index, x_index))
        })
        .unwrap();

    dijkstras(&map, start_point, end_point).unwrap()
}

fn part_2(input: &str) -> usize {
    let map: Vec<Vec<char>> = input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| line.chars().collect())
        .collect();
    let end_point = map
        .iter()
        .enumerate()
        .find_map(|(y_index, chars)| {
            chars
                .iter()
                .enumerate()
                .find_map(|(x_index, character)| {
                    if *character == 'E' {
                        Some(x_index)
                    } else {
                        None
                    }
                })
                .map(|x_index| (y_index, x_index))
        })
        .unwrap();
    let mut paths = map
        .iter()
        .enumerate()
        .flat_map(|(y_index, chars)| {
            chars
                .iter()
                .enumerate()
                .filter_map(|(x_index, character)| {
                    if *character == 'S' || *character == 'a' {
                        Some(x_index)
                    } else {
                        None
                    }
                })
                .map(move |x_index| (y_index, x_index))
        })
        .filter_map(|start_position| {
            dijkstras(&map, start_position, end_point)
        }).collect::<Vec<_>>();
    paths.sort_unstable();
    paths[0]
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let path = &args[1];
    let input = std::fs::read_to_string(path)
        .unwrap_or_else(|error| panic!("No file found at: '{}'.\n{}", path, error));

    let part_1_result = part_1(&input);
    println!("Day 12 Part 1: {}", part_1_result);

    let part_2_result = part_2(&input);
    println!("Day 12 Part 2: {}", part_2_result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_should_return_fewest_amount_of_steps() {
        let input = "Sabqponm\nabcryxxl\naccszExk\nacctuvwj\nabdefghi\n\n";

        assert_eq!(part_1(input), 31);
    }

    #[test]
    fn part_2_should_return_fewest_amount_of_steps() {
        let input = "Sabqponm\nabcryxxl\naccszExk\nacctuvwj\nabdefghi\n\n";

        assert_eq!(part_2(input), 29);
    }
}
