fn part_1(input: &str) -> u16 {
    input
        .split('\n')
        .filter(|line| !line.is_empty())
        .map(|line| {
            line.split(',')
                .flat_map(|side| side.split('-').filter_map(|side| side.parse::<u16>().ok()))
                .collect::<Vec<u16>>()
        })
        .filter_map(|line| {
            if let [left_start, left_end, right_start, right_end] = line[..] {
                if (left_start <= right_start && left_end >= right_end)
                    || (right_start <= left_start && right_end >= left_end)
                {
                    return Some(1);
                }
            }
            None
        })
        .sum()
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let path = &args[1];
    let input = std::fs::read_to_string(path)
        .unwrap_or_else(|error| panic!("No file found at: '{}'.\n{}", path, error));

    let part_1_result = part_1(&input);
    println!("Day 4 Part 1: {}", part_1_result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_should_find_the_assignments_that_are_fully_contained() {
        let test_input = "2-4,6-8\n2-3,4-5\n5-7,7-9\n2-8,3-7\n6-6,4-6\n2-6,4-8\n\n";

        assert_eq!(part_1(test_input), 2);
    }
}
