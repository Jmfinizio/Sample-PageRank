use rand::Rng;
use rand::prelude::SliceRandom;
use approx::assert_abs_diff_eq;

fn random_walk(graph: Vec<Vec<usize>>) -> Vec<usize> {
    let mut rng = rand::thread_rng();
    let mut endpoints: Vec<usize> = Vec::new();
    for i in 0..=graph.len()-1 {
        let mut vertex = i;
        for _ in 1..=100 {
            if graph[vertex].len() > 0 {
                let rand: f64 = rng.gen();
                if rand >= 0.1 {
                    vertex = *graph[vertex].choose(&mut rng).unwrap();
                }
                else if rand < 0.1 {
                    vertex = rng.gen_range(0..graph.len());
                }
            }
            else {
            vertex = rng.gen_range(0..graph.len())
            }
        }
        endpoints.push(vertex);
    }
    endpoints
}

fn pagerank(graph: Vec<Vec<usize>>) -> Vec<(usize, usize)> {
    let mut pagerank_values: Vec<(usize, usize)> = Vec::new();
    let mut walk_results: Vec<usize> = Vec::new();
    for _ in 0..100 {
        walk_results.extend(random_walk(graph.clone()));
    }
    for i in 0..=graph.len()-1 {
        let count = walk_results.iter().filter(|&&x| x == i).count();
        pagerank_values.push((count, i));
    }
    pagerank_values.sort_by(|a, b| b.cmp(a));
    pagerank_values
}

fn print_top_5(graph: Vec<Vec<usize>>) {
    let pagerank: Vec<(usize, usize)> = pagerank(graph.clone());
    for (count,vertex) in &pagerank[..5] {
        let value = *count as f64 / (100.0 * graph.len() as f64);
        println!("vertex {}: approximate Pagerank {}", vertex, value);
    }
}

fn test_sum() {
    let edges = graph::read_file("pagerank_data.txt");
    let graph_list = graph::graph_list(edges,1000);
    let pagerank = pagerank(graph_list.clone());
    let mut total = 0.0;
    for (count, _vertex) in pagerank {
        let value = count as f64 / (100.0 * graph_list.len() as f64);
        total += value;
    }
    assert_abs_diff_eq!(total, 1.0, epsilon = 0.00001);
    assert!(total == 1.0, "Sum of PageRank values does not equal 1");
}



mod graph;
fn main() {
    let edges = graph::read_file("pagerank_data.txt");
    let graph_list = graph::graph_list(edges,1000);
    print_top_5(graph_list);
    test_sum()
}
