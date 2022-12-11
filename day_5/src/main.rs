fn parse_stack_line(line: &str) -> Vec<(usize, char)> {
    line.chars()
        .collect::<Vec<_>>()
        .chunks(4)
        .enumerate()
        .filter_map(|(index, chars)| {
            let characters = chars
                .iter()
                .filter(|character| character.is_alphabetic())
                .collect::<Vec<&char>>();
            if characters.is_empty() {
                None
            } else {
                Some((index, *characters[0]))
            }
        })
        .collect::<_>()
}

fn parse_instruction_line(instruction_line: &str) -> [usize; 3] {
    instruction_line
        .split(' ')
        .filter_map(|chunk| chunk.parse::<usize>().ok())
        .collect::<Vec<usize>>()
        .try_into()
        .unwrap()
}

fn get_stack_count(stack_lines: &str) -> usize {
    stack_lines
        .chars()
        .into_iter()
        .filter_map(|character| character.to_digit(10))
        .count()
}

fn stack_lines_to_map(mut lines: Vec<(usize, char)>, stack_count: usize) -> Vec<Vec<char>> {
    let mut result: Vec<Vec<char>> = Vec::new();
    for _ in 0..stack_count {
        result.push(Vec::new());
    }

    for (index, character) in lines.iter_mut().rev() {
        result[*index].push(*character);
    }

    result
}

fn part_1(input: &str) -> String {
    if let Some((initial_stacks, instructions)) = input.split_once("\n\n") {
        let stack_count = get_stack_count(initial_stacks);
        let stack_lines = initial_stacks
            .lines()
            .flat_map(parse_stack_line)
            .collect::<Vec<_>>();
        let mut stack_map = stack_lines_to_map(stack_lines, stack_count);
        instructions.lines().map(parse_instruction_line).for_each(
            |[move_count, from_stack, to_stack]| {
                for _ in 0..move_count {
                    if let Some(element_to_move) = stack_map[from_stack - 1].pop() {
                        stack_map[to_stack - 1].push(element_to_move);
                    }
                }
            },
        );
        stack_map
            .iter()
            .filter_map(|stack| stack.last())
            .collect::<String>()
    } else {
        panic!()
    }
}

fn part_2(input: &str) -> String {
    if let Some((initial_stacks, instructions)) = input.split_once("\n\n") {
        let stack_count = get_stack_count(initial_stacks);
        let stack_lines = initial_stacks
            .lines()
            .flat_map(parse_stack_line)
            .collect::<Vec<_>>();
        let mut stack_map = stack_lines_to_map(stack_lines, stack_count);
        instructions.lines().map(parse_instruction_line).for_each(
            |[move_count, from_stack, to_stack]| {
                let stack_length = stack_map[from_stack - 1].len();
                let mut element_to_append = stack_map[from_stack - 1]
                    .drain(stack_length - move_count..)
                    .collect();
                stack_map[to_stack - 1].append(&mut element_to_append);
            },
        );
        stack_map
            .iter()
            .filter_map(|stack| stack.last())
            .collect::<String>()
    } else {
        panic!()
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let path = &args[1];
    let input = std::fs::read_to_string(path)
        .unwrap_or_else(|error| panic!("No file found at: '{}'.\n{}", path, error));

    let part_1_result = part_1(&input);
    println!("Day 5 Part 1: {}", part_1_result);

    let part_2_result = part_2(&input);
    println!("Day 5 Part 2: {}", part_2_result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_stack_line_should_return_a_vec_with_chars_and_their_indices() {
        let line = "[Z] [M] [P]";
        assert_eq!(parse_stack_line(line), vec![(0, 'Z'), (1, 'M'), (2, 'P')]);
        let line = "    [D]    ";
        assert_eq!(parse_stack_line(line), vec![(1, 'D')]);
    }

    #[test]
    fn parse_stack_line_should_ignore_the_line_with_column_numbers() {
        let line = "1   2   3   4   5   6   7   8   9 ";
        assert_eq!(parse_stack_line(line), vec![]);
    }

    #[test]
    fn stack_lines_to_map_should_return_a_two_dimensional_vec_of_chars() {
        let input = vec![(0, 'A'), (1, 'B')];
        assert_eq!(stack_lines_to_map(input, 2), vec![vec!['A'], vec!['B']]);
        let input = vec![(0, 'A'), (2, 'B')];
        assert_eq!(
            stack_lines_to_map(input, 3),
            vec![vec!['A'], vec![], vec!['B']]
        );
    }

    #[test]
    fn get_stack_count_should_return_the_amount_of_stacks() {
        let stack_lines = "    [A] [B]\n 1   2   3 \n";
        assert_eq!(get_stack_count(stack_lines), 3);
    }
}
