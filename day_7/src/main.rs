use std::collections::HashMap;

#[derive(Debug)]
struct Dir {
    // size of files directly in this dir
    size: u32,
    children: Vec<String>,
}

fn get_size_of_directory(
    directory_path: &str,
    map: &HashMap<String, Dir>,
    sizes: &mut Vec<u32>,
) -> u32 {
    let mut child_dir_size = 0;
    let dir = map.get(directory_path).unwrap();
    for child in dir.children.iter() {
        child_dir_size +=
            get_size_of_directory(&(directory_path.to_owned() + "/" + child), map, sizes);
    }

    let directory_size = dir.size + child_dir_size;
    sizes.push(directory_size);

    directory_size
}

fn build_dir_map(lines: &mut std::str::Lines) -> HashMap<String, Dir> {
    let mut map = HashMap::<String, Dir>::new();
    let mut current_path = Vec::new();
    for line in lines.by_ref() {
        if line.starts_with("$ cd ") {
            let (_, new_directory) = line.split_once("$ cd ").unwrap();
            if new_directory == ".." {
                current_path.pop();
            } else {
                current_path.push(new_directory.to_string());
            }
            continue;
        }

        if line.starts_with("$ ls") {
            continue;
        }

        let dir_key = current_path.join("/");
        let known_dir = map.entry(dir_key).or_insert_with(|| Dir {
            size: 0,
            children: Vec::<_>::new(),
        });
        if line.starts_with("dir") {
            let (_, directory_name) = line.split_once(' ').unwrap();
            known_dir.children.push(directory_name.to_string());
            continue;
        }

        if let Some((size, _)) = line.split_once(' ') {
            let size = size.parse::<u32>().ok().unwrap();
            known_dir.size += size;
        }
    }
    map
}

fn part_1(input: &str) -> u32 {
    let map = build_dir_map(&mut input.lines());
    let mut sizes = Vec::<u32>::new();
    get_size_of_directory("/", &map, &mut sizes);

    sizes
        .iter()
        .filter(|&&directory_size| directory_size <= 100000)
        .sum()
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let path = &args[1];
    let input = std::fs::read_to_string(path)
        .unwrap_or_else(|error| panic!("No file found at: '{}'.\n{}", path, error));

    let part_1_result = part_1(&input);
    println!("Day 7 Part 1: {}", part_1_result);
}
