use std::fs::File;
use std::io::BufRead;

pub fn read_file(path: &str) -> Vec<(u32,u32)> {
    let mut result: Vec<(u32,u32)> = Vec::new();
    let file = File::open(path).expect("Could not open file");
    let buf_reader = std::io::BufReader::new(file).lines();
    for line in buf_reader {
        let line_str = line.expect("Error reading");
        let v: Vec<&str> = line_str.trim().split(' ').collect();
        let x = v[0].parse::<u32>().unwrap();
        let y = v[1].parse::<u32>().unwrap();
        result.push((x,y));
    }
    result
}
    
pub fn graph_list(edges: Vec<(u32,u32)>, n:usize) -> Vec<Vec<usize>> {
    let mut graph_list : Vec<Vec<usize>> = vec![vec![];n];
    for (v,w) in edges.iter() {
    graph_list[*v as usize].push(*w as usize);
    };
    graph_list
}
    
