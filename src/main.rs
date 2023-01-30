use std::env;
use std::fs;

use rand::seq::SliceRandom;


fn parse_file_names(file_name: &str) -> Vec<String> {
    let file = fs::read_to_string(file_name).expect("Unable to read file");
    let mut file_names: Vec<String> = Vec::new();
    for line in file.lines() {
        file_names.push(line.to_string());
    }
    file_names
}

fn melange(names: &mut Vec<String>) {
    let mut rng = rand::thread_rng();
    names.shuffle(&mut rng);
}

fn create_groups(names: Vec<String>, groups_size: &usize) -> Vec<Vec<String>> {
    let mut groups: Vec<Vec<String>> = Vec::new();
    let mut group: Vec<String> = Vec::new();
    for name in names {
        group.push(name);
        if group.len() == *groups_size {
            groups.push(group);
            group = Vec::new();
        }
    }
    if group.len() > 0 {
        groups.push(group);
    }
    groups
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_name = &args[1];
    let groups_size = &args[2].parse::<usize>().unwrap();
    
    println!("File name: {}", file_name);
    let mut names = parse_file_names(file_name);
    for name in names.iter() {
        println!("Name: {}", name);
    }

    melange(&mut names);

    let groups = create_groups(names, groups_size);
    for group in groups.iter() {
        println!("Group:");
        for name in group.iter() {
            println!("Name: {}", name);
        }
    }


    println!("Hello, world!");
}
