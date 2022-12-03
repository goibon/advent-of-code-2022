const ROCK_VALUE: u16 = 1;
const PAPER_VALUE: u16 = 2;
const SCISSORS_VALUE: u16 = 3;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Hand {
    Rock,
    Paper,
    Scissors,
}

const WIN_VALUE: u16 = 6;
const LOSE_VALUE: u16 = 0;
const DRAW_VALUE: u16 = 3;

fn map_input_for_part_1(string: &str) -> Vec<(Hand, Hand)> {
    string
        .split('\n')
        .filter(|line| !line.is_empty())
        .map(|line| {
            let hands = line
                .chars()
                .filter_map(|character| match character {
                    'A' | 'X' => Some(Hand::Rock),
                    'B' | 'Y' => Some(Hand::Paper),
                    'C' | 'Z' => Some(Hand::Scissors),
                    _ => None,
                })
                .collect::<Vec<_>>();
            (hands[0], hands[1])
        })
        .collect::<_>()
}

fn part_1(input: &str) -> u16 {
    let games = map_input_for_part_1(input);
    games
        .iter()
        .map(|(player_1, player_2)| {
            let outcome_value = if player_1 == player_2 {
                DRAW_VALUE
            } else if match player_1 {
                Hand::Rock => *player_2 == Hand::Paper,
                Hand::Paper => *player_2 == Hand::Scissors,
                Hand::Scissors => *player_2 == Hand::Rock,
            } {
                WIN_VALUE
            } else {
                LOSE_VALUE
            };

            outcome_value
                + match player_2 {
                    Hand::Rock => ROCK_VALUE,
                    Hand::Paper => PAPER_VALUE,
                    Hand::Scissors => SCISSORS_VALUE,
                }
        })
        .sum()
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let path = &args[1];
    let input = std::fs::read_to_string(path)
        .unwrap_or_else(|error| panic!("No file found at: '{}'.\n{}", path, error));

    let part_1_result = part_1(&input);
    println!("Day 2 Part 1: {}", part_1_result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn map_input_for_part_1_should_split_on_newline_and_map_to_game() {
        let test_input = "A X\n\n";

        let result = map_input_for_part_1(test_input);
        assert_eq!(result[0].0, Hand::Rock);
        assert_eq!(result[0].1, Hand::Rock);
    }

    #[test]
    fn map_input_for_part_1_should_split_on_newline_and_map_to_something() {
        let test_input = "A X\nB Y\nC Z\n\n";

        let result = map_input_for_part_1(test_input);
        assert_eq!(result[0].0, Hand::Rock);
        assert_eq!(result[0].1, Hand::Rock);
        assert_eq!(result[1].0, Hand::Paper);
        assert_eq!(result[1].1, Hand::Paper);
        assert_eq!(result[2].0, Hand::Scissors);
        assert_eq!(result[2].1, Hand::Scissors);
    }

    #[test]
    fn part_1_should_return_the_score_from_following_the_strategy() {
        let test_input = "A Y\nB X\nC Z\n\n";

        assert_eq!(part_1(test_input), 15);
    }
}
