use std::cmp::Ordering;
use crate::utils::read_lines;
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::f64::consts::PI;
use std::fmt::Debug;
use std::ops::{Add, Sub};
use image::{ImageBuffer, Rgb, RgbImage};
use num::traits::AsPrimitive;


#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct Position {
    x: i32,
    y: i32,
}

impl Position {
    fn new<T: AsPrimitive<i32>>(x: T, y: T) -> Self
        where i32: AsPrimitive<i32>
    {
        Position { x: x.as_(), y: y.as_() }
    }

    fn in_limits(&self, upper_bounds: Position) -> bool {
        self.x < upper_bounds.x && self.y < upper_bounds.y &&
            self.x >= 0 && self.y >= 0
    }
}

impl Add for Position {
    type Output = Position;

    fn add(self, rhs: Self) -> Self::Output {
        (self.x + rhs.x, self.y + rhs.y).into()
    }
}

impl Sub for Position {
    type Output = Position;

    fn sub(self, rhs: Self) -> Self::Output {
        (self.x - rhs.x, self.y - rhs.y).into()
    }
}

impl<T: AsPrimitive<i32>> From<(T, T)> for Position
    where i32: AsPrimitive<T>
{
    fn from(position: (T, T)) -> Self {
        Position::new(position.0, position.1)
    }
}

impl<T: AsPrimitive<T>> Into<(T, T)> for Position
    where i32: AsPrimitive<T>
{
    fn into(self) -> (T, T) {
        (self.x.as_(), self.y.as_())
    }
}


fn get_matrix_neighbors(zone: Position, upper_bounds: Position) -> Vec<Position> {
    let mut neighbors = vec![];
    for delta in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
        if (zone + delta.into()).in_limits(upper_bounds) {
            neighbors.push(zone + delta.into());
        }
    }
    neighbors
}

fn _colormap<Numeral: Copy + Debug + AsPrimitive<f64>>(val: Numeral, max_val: Numeral) -> Rgb<u8> {
    let mut factor: f64 = val.as_() / max_val.as_();
    factor = factor.max(0f64).min(1f64);

    Rgb([
        ((0.5f64 * PI * factor).sin() * 255f64) as u8,
        (1f64 + (0.5f64 * PI * (3f64 + factor)).cos() * factor * 255f64) as u8,
        (((2f64 * PI * factor).sin() + factor) * factor * 255f64) as u8,
    ])
}


#[derive(Debug, Copy, Clone, Eq, PartialEq)]
struct CostMapping {
    cost: i32,
    zone: Position,
}

impl CostMapping {
    fn new(cost: i32, zone: Position) -> Self {
        CostMapping { cost, zone }
    }
}

impl PartialOrd<Self> for CostMapping {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match self.cost.partial_cmp(&other.cost) {
            None => None,
            Some(order) => Some(order.reverse())
        }
    }
}

impl From<(i32, Position)> for CostMapping {
    fn from((cost, zone): (i32, Position)) -> Self {
        CostMapping { cost, zone }
    }
}

impl Ord for CostMapping {
    fn cmp(&self, other: &Self) -> Ordering {
        self.cost.cmp(&other.cost).reverse()
    }
}

#[derive(Debug)]
struct Zones {
    danger_level: HashMap<Position, i32>,
    adjacency: HashMap<Position, Vec<Position>>,
    cost_map: HashMap<Position, i32>,
    size: Position,
}

impl Zones {
    fn push_zone(&mut self, zone: Position, danger_level: i32, adjacened_zones: Vec<Position>) {
        self.danger_level.insert(zone, danger_level);
        self.adjacency.insert(zone, adjacened_zones);
    }

    fn new(map: Vec<Vec<i32>>) -> Zones {
        let mut graph = Zones {
            danger_level: HashMap::new(),
            adjacency: HashMap::new(),
            cost_map: HashMap::new(),
            size: (map[0].len(), map.len()).into(),
        };
        for y in 0..map.len() {
            for x in 0..map[0].len() {
                graph.push_zone((x, y).into(),
                                map[y][x],
                                get_matrix_neighbors((x, y).into(), (map.len(), map[0].len()).into()));
            }
        }
        graph
    }

    fn _new2(map: Vec<Vec<i32>>) -> Zones {
        let mut graph = Zones {
            danger_level: HashMap::new(),
            adjacency: HashMap::new(),
            cost_map: HashMap::new(),
            size: (map[0].len() * 5, map.len() * 5).into(),
        };

        for v in 0..5 {
            for u in 0..5 {
                for y in 0..map.len() {
                    for x in 0..map[0].len() {
                        graph.push_zone((x + (u * map[0].len()), y + (v * map.len())).into(),
                                        (map[y][x] + u as i32 + v as i32 - 1) % 9 + 1,
                                        get_matrix_neighbors(
                                            (x + (u * map[0].len()), y + (v * map.len())).into(),
                                            (map[0].len() * 5, map.len() * 5).into()));
                    }
                }
            }
        }
        graph
    }


    fn a_star(&mut self, start: Position, goal: Position) {
        let mut shortest_path_set = HashSet::new();
        let mut cost_priority_queue: BinaryHeap<CostMapping> = BinaryHeap::new();

        cost_priority_queue.push((0, start).into());
        self.cost_map.insert(start, 0);

        while !cost_priority_queue.is_empty() {
            let current_zone = cost_priority_queue.pop().unwrap().zone;
            let current_cost = self.cost_map.get(&current_zone).unwrap().clone();

            shortest_path_set.insert(current_zone);

            if shortest_path_set.contains(&goal) {
                break;
            }


            match self.adjacency.get(&current_zone) {
                Some(new_zones) => {
                    match new_zones.into_iter()
                        .filter(|&zone| !shortest_path_set.contains(zone))
                        .map(|&z| z)
                        .collect::<Vec<Position>>() {
                        zones if !zones.is_empty() =>
                            {
                                zones.iter().for_each(|zone|
                                    {
                                        let new_cost = current_cost + self.danger_level.get(zone).unwrap();
                                        // let heuristic = (goal.x - (*zone).x).abs() + (goal.y - (*zone).y).abs();
                                        let heuristic = 0;
                                        match self.cost_map.get_mut(zone) {
                                            Some(old_cost) =>
                                                if *old_cost > new_cost {
                                                    *old_cost = new_cost;
                                                    cost_priority_queue.push(CostMapping::new(new_cost + heuristic, *zone));
                                                }
                                            _ => {
                                                self.cost_map.insert(*zone, new_cost);
                                                cost_priority_queue.push(CostMapping::new(new_cost + heuristic, *zone));
                                            }
                                        }
                                    })
                            }
                        _ => continue,
                    }
                }
                _ => break,
            }
        }
    }

    fn _plot(&self, filename: &str) {
        let mut image: RgbImage =
            ImageBuffer::new(2 * self.size.x as u32 + 1, 2 * self.size.y as u32 + 1);


        self.danger_level.iter()
            .for_each(|(pos, val)|
                if self.cost_map.contains_key(pos) {
                    *image.get_pixel_mut(pos.x as u32 * 2 + 1, pos.y as u32 * 2 + 1)
                        = _colormap(*val, 10)
                }
            );


        self.cost_map.iter()
            .for_each(|(p, v)|
                *image.get_pixel_mut(p.x as u32 * 2 as u32, p.y as u32 * 2)
                    = _colormap(*v - p.x - p.y, 4 * (self.size.x + self.size.y)));

        image.save(filename).unwrap();
    }

    fn get_minimal_cost(&self, goal: &Position) -> i32 {
        match self.cost_map.get(goal) {
            Some(cost) => *cost,
            _ => -1,
        }
    }
}


pub fn part_one(filename: &str) -> i32 {
    let lines = read_lines(filename);
    let danger_level = lines.into_iter()
        .map(|line|
            line.unwrap().chars()
                .map(|c| c.to_string().parse::<i32>().unwrap())
                .collect::<Vec<i32>>())
        .collect::<Vec<Vec<i32>>>();

    let goal = (danger_level[0].len() - 1, danger_level.len() - 1);
    let mut cave = Zones::new(danger_level);

    cave.a_star((0, 0).into(), goal.into());

    cave.get_minimal_cost(&goal.into())
}

pub fn _part_two(filename: &str) -> i32 {
    let lines = read_lines(filename);
    let danger_level = lines.into_iter()
        .map(|line|
            line.unwrap().chars()
                .map(|c| c.to_string().parse::<i32>().unwrap())
                .collect::<Vec<i32>>())
        .collect::<Vec<Vec<i32>>>();

    let goal = (danger_level[0].len() * 5 - 1, danger_level.len() * 5 - 1);
    let mut cave = Zones::_new2(danger_level);
    cave._plot(&format!("day15.png"));
    cave.a_star((0, 0).into(), goal.into());

    cave.get_minimal_cost(&goal.into())
}