use std::collections::{HashMap, HashSet};
use std::hash::Hash;
use crate::utils::read_lines;

#[derive(Debug, Clone)]
struct Graph<Vertex> {
    vertices: HashSet<Vertex>,
    adjacency: HashMap<Vertex, Vec<Vertex>>,
}

impl<Vertex: Clone + Eq + Hash> Graph<Vertex> {
    fn new() -> Graph<Vertex> {
        Graph { vertices: HashSet::new(), adjacency: HashMap::new() }
    }

    fn push_vertex(self: &mut Graph<Vertex>, vertex: Vertex) {
        self.vertices.insert(vertex);
    }

    fn remove_vertex(self: &mut Graph<Vertex>, vertex: &Vertex) {
        self.vertices.remove(vertex);
    }

    fn push_edge_directed(self: &mut Self, from: Vertex, to: Vertex) {
        self.push_vertex(from.clone());
        self.push_vertex(to.clone());
        let vertex_from = self.adjacency.entry(from).or_default();
        vertex_from.push(to);
    }

    fn push_edge(self: &mut Self, a: Vertex, b: Vertex) {
        self.push_edge_directed(a.clone(), b.clone());
        self.push_edge_directed(b, a);
    }

    fn receive_adjacency_vector(self: &mut Self, vertex: &Vertex) -> Vec<Vertex> {
        self.adjacency.get(vertex).unwrap().clone().into_iter()
            .filter(|v| self.vertices.contains(v)).collect::<Vec<Vertex>>()
    }
}

pub fn part_one(filename: &str) -> i32 {
    let lines = read_lines(filename);

    let mut graph = Graph::new();
    lines.into_iter().for_each(|line| {
        let vertices = line.unwrap().split("-")
            .map(|s| s.to_string()).collect::<Vec<String>>();
        if vertices[0].as_str() == "end" {
            graph.push_edge_directed(vertices[1].clone(), vertices[0].clone())
        } else if vertices[1].as_str() == "end" {
            graph.push_edge_directed(vertices[0].clone(), vertices[1].clone())
        } else if vertices[0].as_str() == "start" {
            graph.push_edge_directed(vertices[0].clone(), vertices[1].clone())
        } else if vertices[1].as_str() == "start" {
            graph.push_edge_directed(vertices[1].clone(), vertices[0].clone())
        } else {
            graph.push_edge(vertices[0].clone(), vertices[1].clone())
        }
    });



    fn traverse_graph(key: &String, graph: &Graph<String>) -> i32 {
        let mut paths: i32 = 0;

        let mut new_graph = graph.clone();
        if key.chars().next().unwrap().is_lowercase() {
            new_graph.remove_vertex(key);
        }
        match key.as_str() {
            "end" => paths += 1,
            _ => new_graph.receive_adjacency_vector(key).iter()
                .for_each(|k| paths += traverse_graph(k, &new_graph)),
        };
        paths
    }

    traverse_graph(&"start".to_string(), &graph)
}

pub fn _part_two(filename: &str) -> i32 {
    let lines = read_lines(filename);
    let mut graph = Graph::new();
    lines.into_iter().for_each(|line| {
        let vertices = line.unwrap().split("-")
            .map(|s| s.to_string()).collect::<Vec<String>>();
        if vertices[0].as_str() == "end" {
            graph.push_edge_directed(vertices[1].clone(), vertices[0].clone())
        } else if vertices[1].as_str() == "end" {
            graph.push_edge_directed(vertices[0].clone(), vertices[1].clone())
        } else if vertices[0].as_str() == "start" {
            graph.push_edge_directed(vertices[0].clone(), vertices[1].clone())
        } else if vertices[1].as_str() == "start" {
            graph.push_edge_directed(vertices[1].clone(), vertices[0].clone())
        } else {
            graph.push_edge(vertices[0].clone(), vertices[1].clone())
        }
    });

    fn traverse_graph(key: &String, graph: &Graph<String>, unvisited_lowercase: &HashSet<String>, double_visit: bool) -> i32 {
        let mut paths: i32 = 0;

        let mut new_graph = graph.clone();
        let mut new_unvisited_lowercase = unvisited_lowercase.clone();
        let mut new_double_visit = double_visit;

        if key.as_str() == "end" || key.as_str() == "start" {
            new_graph.remove_vertex(key);
        } else if key.chars().next().unwrap().is_lowercase() {
            if new_unvisited_lowercase.contains(key) {
                new_unvisited_lowercase.remove(key);
            } else if double_visit == false {
                return paths;
            } else {
                new_graph.remove_vertex(key);
                new_double_visit = false;
            }
        }
        match key.as_str() {
            "end" => paths += 1,

            _ => new_graph.receive_adjacency_vector(key).iter()
                .for_each(|k| paths += traverse_graph(k, &new_graph, &new_unvisited_lowercase, new_double_visit))
        };
        paths
    }

    let unvisited = graph.vertices.clone().into_iter()
        .filter(|v| match v.as_str() {
            "end" => false,
            "start" => false,
            _ => v.chars().next().unwrap().is_lowercase()
        })
        .collect::<HashSet<String>>();

    traverse_graph(&"start".to_string(), &graph, &unvisited, true)
}