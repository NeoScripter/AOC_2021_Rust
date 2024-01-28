use std::collections::{HashMap, HashSet};

struct Graph {
    adjacency_list: HashMap<String, HashSet<String>>,
}

impl Graph {
    fn new() -> Self {
        Self {
            adjacency_list: HashMap::new(),
        }
    }
    fn add_node(&mut self, node: String) {
        self.adjacency_list.entry(node).or_insert_with(|| HashSet::new());
    }
    fn add_edge(&mut self, node1: String, node2: String) {
        self.adjacency_list.entry(node1.clone()).or_default().insert(node2.clone());
        self.adjacency_list.entry(node2).or_default().insert(node1);
    }
    fn count_paths_part1(&self, start: &String, end: &String) -> usize {
        let mut visited = HashSet::new();
        let mut small_caves = HashSet::new();
        self.count_paths_helper_part1(start, end, &mut visited, &mut small_caves)
    }
    
    fn count_paths_helper_part1(&self, current: &String, end: &String, visited: &mut HashSet<String>, small_caves: &mut HashSet<String>) -> usize {
        if current == end {
            return 1;
        }
    
        let is_small_cave = current.chars().all(char::is_lowercase);
        if is_small_cave && !small_caves.insert(current.clone()) {
            return 0;
        }
    
        if !is_small_cave {
            visited.insert(current.clone());
        }
    
        let mut path_count = 0;
        if let Some(neighbors) = self.adjacency_list.get(current) {
            for neighbor in neighbors {
                if is_small_cave || !visited.contains(neighbor) {
                    path_count += self.count_paths_helper_part1(neighbor, end, visited, small_caves);
                }
            }
        }
    
        if is_small_cave {
            small_caves.remove(current);
        } else {
            visited.remove(current);
        }
        path_count
    }    
    fn count_paths_part2(&self, start: &String, end: &String) -> usize {
        let mut visited = HashSet::new();
        let mut small_caves = HashMap::new();
        self.count_paths_helper_part2(start, end, &mut visited, &mut small_caves)
    }
    fn count_paths_helper_part2(&self, current: &String, end: &String, visited: &mut HashSet<String>, small_caves: &mut HashMap<String, i32>) -> usize {
        if current == end {
            return 1;
        }
    
        let is_small_cave = current.chars().all(char::is_lowercase);
        if is_small_cave {
            *small_caves.entry(current.clone()).or_insert(0) += 1;
        
            let is_invalid_path = small_caves.values().any(|&v| v > 2)
                || small_caves.values().filter(|&&v| v >= 2).count() > 1
                || *small_caves.get("start").unwrap() > 1;
        
            if is_invalid_path {
                *small_caves.entry(current.clone()).or_insert(0) -= 1;
                return 0;
            }
        }
        if !is_small_cave {
            visited.insert(current.clone());
        }
    
        let mut path_count = 0;
        if let Some(neighbors) = self.adjacency_list.get(current) {
            for neighbor in neighbors {
                if is_small_cave || !visited.contains(neighbor) {
                    path_count += self.count_paths_helper_part2(neighbor, end, visited, small_caves);
                }
            }
        }
    
        if is_small_cave {
            *small_caves.entry(current.clone()).or_insert(0) -= 1;
        } else {
            visited.remove(current);
        }
        path_count
    }      
}
fn solve(input: &str) -> usize {
    let mut graph = Graph::new();
    input.lines().for_each(|line| {
        let (node1, node2) = line.split_once('-').unwrap();
        graph.add_node(node1.to_string());
        graph.add_node(node2.to_string());
        graph.add_edge(node1.to_string(), node2.to_string());
    });
    let start = String::from("start");
    let end = String::from("end");
    graph.count_paths_part2(&start, &end)
}
fn main() {
    let input = include_str!("input12.txt");
    println!("{}", solve(input));
}