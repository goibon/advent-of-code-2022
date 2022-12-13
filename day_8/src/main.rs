use std::collections::HashMap;

enum Direction {
    Left,
    Right,
    Up,
    Down,
}

fn create_map(input: &str) -> HashMap<(usize, usize), u32> {
    HashMap::from_iter(
        input
            .split('\n')
            .filter(|line| !line.is_empty())
            .enumerate()
            .flat_map(|(y_index, line)| {
                line.chars()
                    .filter_map(|character| character.to_digit(10))
                    .enumerate()
                    .map(|(x_index, tree_height)| ((y_index, x_index), tree_height))
                    .collect::<Vec<_>>()
            }),
    )
}

fn search_until_edge_or_blocked(
    start_y: usize,
    start_x: usize,
    map: &HashMap<(usize, usize), u32>,
    direction: Direction,
    limit: usize,
) -> (i32, bool) {
    let (y_direction, x_direction): (i32, i32) = match direction {
        Direction::Left => (0, -1),
        Direction::Right => (0, 1),
        Direction::Up => (-1, 0),
        Direction::Down => (1, 0),
    };

    let starting_tree = map.get(&(start_y, start_x)).unwrap();
    let mut distance = 0;
    let mut current_y = start_y as i32;
    let mut current_x = start_x as i32;
    let mut hit_edge = false;
    loop {
        if current_y == 0
            || current_y == limit as i32
            || current_x == 0
            || current_x == limit as i32
        {
            hit_edge = true;
            break;
        }

        distance += 1;
        current_y += y_direction;
        current_x += x_direction;
        let other_tree = map
            .get(&(current_y.try_into().unwrap(), current_x.try_into().unwrap()))
            .unwrap();
        if other_tree >= starting_tree {
            break;
        }
    }
    (distance, hit_edge)
}

fn part_1(input: &str) -> usize {
    let map: HashMap<(usize, usize), u32> = create_map(input);
    let mut visible_trees = Vec::<(usize, usize)>::new();
    let upper_limit = ((map.len() as f32).sqrt() - 1.) as usize;
    for ((y, x), _) in map.iter() {
        let (_, hit_edge) =
            search_until_edge_or_blocked(*y, *x, &map, Direction::Left, upper_limit);
        if hit_edge {
            visible_trees.push((*y, *x));
            continue;
        }
        let (_, hit_edge) =
            search_until_edge_or_blocked(*y, *x, &map, Direction::Right, upper_limit);
        if hit_edge {
            visible_trees.push((*y, *x));
            continue;
        }
        let (_, hit_edge) = search_until_edge_or_blocked(*y, *x, &map, Direction::Up, upper_limit);
        if hit_edge {
            visible_trees.push((*y, *x));
            continue;
        }
        let (_, hit_edge) =
            search_until_edge_or_blocked(*y, *x, &map, Direction::Down, upper_limit);
        if hit_edge {
            visible_trees.push((*y, *x));
            continue;
        }
    }

    visible_trees.len()
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let path = &args[1];
    let input = std::fs::read_to_string(path)
        .unwrap_or_else(|error| panic!("No file found at: '{}'.\n{}", path, error));

    let part_1_result = part_1(&input);
    println!("Day 8 Part 1: {}", part_1_result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_should_return_amount_of_visible_trees() {
        let input = "30373\n25512\n65332\n33549\n35390\n\n";

        assert_eq!(part_1(input), 21)
    }
}
