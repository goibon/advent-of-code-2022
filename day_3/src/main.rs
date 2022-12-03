use itertools::Itertools;
use std::collections::HashMap;

fn get_priority_map() -> HashMap<char, usize> {
    let mut priorities: HashMap<char, usize> = HashMap::new();
    let mut letters = ('a'..='z').collect::<Vec<_>>();
    letters.extend('A'..='Z');
    for (priority, character) in letters.iter().enumerate() {
        priorities.insert(*character, priority + 1);
    }
    priorities
}

fn part_1(input: &str) -> u16 {
    let priorities = get_priority_map();
    input
        .split('\n')
        .filter(|line| !line.is_empty())
        .map(|line| line.split_at(line.len() / 2))
        .filter_map(|(left_compartment, right_compartment)| {
            left_compartment
                .chars()
                .unique()
                .find(|left_char| right_compartment.chars().unique().contains(left_char))
        })
        .filter_map(|character| priorities.get(&character))
        .map(|priority| *priority as u16)
        .sum()
}

fn part_2(input: &str) -> usize {
    let priorities = get_priority_map();
    let mut rucksacks = input
        .split('\n')
        .filter(|line| !line.is_empty())
        .map(|line| line.chars().unique().collect::<Vec<_>>());

    let mut priorities_sum = 0;
    while let Some((elf_1, elf_2, elf_3)) =
        rucksacks.next_tuple::<(Vec<char>, Vec<char>, Vec<char>)>()
    {
        if let Some(shared_item) = elf_1
            .iter()
            .find(|item| elf_2.contains(item) && elf_3.contains(item))
        {
            if let Some(priority) = priorities.get(shared_item) {
                priorities_sum += priority;
            }
        }
    }

    priorities_sum
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let path = &args[1];
    let input = std::fs::read_to_string(path)
        .unwrap_or_else(|error| panic!("No file found at: '{}'.\n{}", path, error));

    let part_1_result = part_1(&input);
    println!("Day 3 Part 1: {}", part_1_result);

    let part_2_result = part_2(&input);
    println!("Day 3 Part 2: {}", part_2_result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_should_find_duplicated_letters_in_each_line_and_summarize_their_priorities() {
        let test_input = "vJrwpWtwJgWrhcsFMMfFFhFp\njqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL\nPmmdzqPrVvPwwTWBwg\nwMqvLMZHhHMvwLHjbvcjnnSBnvTQFn\nttgJtRGJQctTZtZT\nCrZsJsPPZsGzwwsLwLmpwMDw\n\n";

        assert_eq!(part_1(test_input), 157);
    }

    #[test]
    fn part_2_should_find_duplicated_letters_in_each_line_and_summarize_their_priorities() {
        let test_input = "vJrwpWtwJgWrhcsFMMfFFhFp\njqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL\nPmmdzqPrVvPwwTWBwg\nwMqvLMZHhHMvwLHjbvcjnnSBnvTQFn\nttgJtRGJQctTZtZT\nCrZsJsPPZsGzwwsLwLmpwMDw\n\n";

        assert_eq!(part_2(test_input), 70);
    }
}
