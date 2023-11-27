

use std::fs::File;
use std::io::{self, prelude::*, BufReader};


fn read_input(day: &i8) -> Result<Vec<Vec<String>>, Box<dyn std::error::Error>> {
    let file = File::open("data.txt")?;
    let reader = BufReader::new(file);

    let mut data = Vec::new();

    for line in reader.lines() {
        let line = line?;
        let words: Vec<String> = line.split_whitespace()
            .map(|s| s.to_string())
            .collect();

        data.push(words);
    }

    println!("{:?}", data);
    Ok(data)
}

fn manhattan_distance((x1, y1): (&i32, &i32), (x2, y2): (&i32, &i32)) -> i32 {
    let x_diff = (x1 - x2).abs();
    let y_diff = (y1 - y2).abs();

    x_diff + y_diff
}

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

enum Direction {
    North,
    South,
    East,
    West,
}

fn move_point(mut point: Point, direction: Direction) -> Point {
    match direction {
        Direction::North => point.y += 1,
        Direction::South => point.y -= 1,
        Direction::East => point.x += 1,
        Direction::West => point.x -= 1,
    }

    point
}

// Set up for the FW algorithm
// pub const INF: i32 = i32::MAX / 2;
// let mut graph = vec![vec![INF; num_vertices]; num_vertices];
// then add the rest of the

// Create a matrix A0 of dimension n*n where n is the number of vertices. The row and the column are indexed as i and j respectively. i and j are the vertices of the graph.
// Each cell A[i][j] is filled with the distance from the ith vertex to the jth vertex. If there is no path from ith vertex to jth vertex, the cell is left as infinity.
// eg
// G = [[0, 3, INF, 5],
//      [2, 0, INF, 4],
//      [INF, 1, 0, INF],
//      [INF, INF, 2, 0]]
pub fn floyd_warshall(mut graph: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let num_vertices = graph.len();

    for k in 0..num_vertices{
        for i in 0..num_vertices {
            for j in 0..num_vertices {
                graph[i][j] = graph[i][j].min(graph[i][k] + graph[k][j]);
            }
        }
    }
    graph
}