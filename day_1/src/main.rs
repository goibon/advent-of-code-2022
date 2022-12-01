fn split_input(string: &str) -> Vec<Vec<u32>> {
    string
        .split("\n\n")
        .map(|set| {
            set.split('\n')
                .filter_map(|line| line.parse().ok())
                .collect()
        })
        .collect()
}

fn part_1(input: &str) -> u32 {
    let mut calories_sums = split_input(input)
        .iter()
        .map(|set| set.iter().sum::<u32>())
        .collect::<Vec<u32>>();
    calories_sums.sort_unstable();
    *calories_sums.last().unwrap()
}

fn part_2(input: &str) -> u32 {
    let mut calories_sums = split_input(input)
        .iter()
        .map(|set| set.iter().sum::<u32>())
        .collect::<Vec<u32>>();
    calories_sums.sort_unstable();
    calories_sums.iter().rev().take(3).sum()
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let path = &args[1];
    let input = std::fs::read_to_string(path)
        .unwrap_or_else(|error| panic!("No file found at: '{}'.\n{}", path, error));

    let part_1_result = part_1(&input);
    println!("Day 1 Part 1: {}", part_1_result);

    let part_2_result = part_2(&input);
    println!("Day 1 Part 2: {}", part_2_result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn split_input_should_split_on_double_newline() {
        let test_input = "1000\n\n2000";

        assert_eq!(split_input(test_input), vec![vec![1000], vec![2000]]);
    }

    #[test]
    fn part_1_should_return_the_highest_sum() {
        let test_input = "1000\n2000\n3000\n\n4000\n\n5000\n6000\n\n7000\n8000\n9000\n\n10000\n";

        assert_eq!(part_1(test_input), 24000);
    }

    #[test]
    fn part_2_should_return_the_sum_of_the_top_three_highest_sums() {
        let test_input = "1000\n2000\n3000\n\n4000\n\n5000\n6000\n\n7000\n8000\n9000\n\n10000\n";

        assert_eq!(part_2(test_input), 45000);
    }
}
