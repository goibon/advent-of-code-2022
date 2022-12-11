use std::collections::HashSet;

fn find_index_of_first_set_of_unique_chars(iter: &[char], set_size: usize) -> usize {
    iter.windows(set_size)
        .enumerate()
        .find_map(|(index, window)| {
            let mut unique = HashSet::new();
            if window.iter().all(|value| unique.insert(value)) {
                Some(index + set_size)
            } else {
                None
            }
        })
        .unwrap()
}

fn part_1(input: &str) -> usize {
    let characters = input.chars().collect::<Vec<_>>();
    find_index_of_first_set_of_unique_chars(&characters, 4)
}

fn part_2(input: &str) -> usize {
    let characters = input.chars().collect::<Vec<_>>();
    find_index_of_first_set_of_unique_chars(&characters, 14)
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let path = &args[1];
    let input = std::fs::read_to_string(path)
        .unwrap_or_else(|error| panic!("No file found at: '{}'.\n{}", path, error));

    let part_1_result = part_1(&input);
    println!("Day 6 Part 1: {}", part_1_result);

    let part_2_result = part_2(&input);
    println!("Day 6 Part 2: {}", part_2_result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_should_return_the_1_index_of_the_first_marker_found() {
        let input = "bvwbjplbgvbhsrlpgdmjqwftvncz";
        assert_eq!(part_1(input), 5);

        let input = "nppdvjthqldpwncqszvftbrmjlhg";
        assert_eq!(part_1(input), 6);

        let input = "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg";
        assert_eq!(part_1(input), 10);

        let input = "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw";
        assert_eq!(part_1(input), 11);
    }

    #[test]
    fn part_2_should_return_the_1_index_of_the_first_marker_found() {
        let input = "mjqjpqmgbljsphdztnvjfqwrcgsmlb";
        assert_eq!(part_2(input), 19);

        let input = "bvwbjplbgvbhsrlpgdmjqwftvncz";
        assert_eq!(part_2(input), 23);

        let input = "nppdvjthqldpwncqszvftbrmjlhg";
        assert_eq!(part_2(input), 23);

        let input = "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg";
        assert_eq!(part_2(input), 29);

        let input = "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw";
        assert_eq!(part_2(input), 26);
    }
}
