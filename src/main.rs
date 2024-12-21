use ::rand::random;
use macroquad::prelude::*;
use serde::{Deserialize, Serialize};
use std::{f32::consts::PI, process::exit, time::Instant};

const NODES_N: usize = 11;
static MAX_EDGES_N: usize = (NODES_N * (NODES_N - 1)) / 2;
static EDGES_THRESHOLD: usize = (MAX_EDGES_N as f32 * 0.5) as usize;

const GRAPHS_N: usize = 100000;

const GRAPH_RADIUS: f32 = 500.0;
const NODE_RADIUS: f32 = 10.0;

#[derive(Serialize, Deserialize)]
struct Graph {
    nodes_n: usize,
    edges: Vec<Vec<bool>>,
}

#[derive(Serialize, Deserialize)]
struct Info {
    nodes_n: usize,
    graphs_n: usize,
}

#[derive(Serialize, Deserialize)]
struct Content {
    info: Info,
    graphs: Vec<Vec<Vec<bool>>>,
}

#[derive(PartialEq, Clone, Copy)]
enum GraphKind {
    Hamiltonian,
    NonHamiltonian,
}

impl GraphKind {
    const ALL: [Self; 2] = [Self::Hamiltonian, Self::NonHamiltonian];
}

impl Graph {
    fn get_path(&self, mut path: Vec<usize>, current_node: usize) -> Vec<usize> {
        if current_node == 0 && path.len() == NODES_N {
            path.push(0);
            return path;
        }

        if path.contains(&current_node) {
            return vec![];
        }

        path.push(current_node);

        for node in 0..NODES_N {
            if self.edges[current_node][node] {
                let path = self.get_path(path.clone(), node);

                if !path.is_empty() {
                    return path;
                }
            }
        }

        vec![]
    }

    fn get_cycle(&self) -> Vec<usize> {
        self.get_path(Vec::with_capacity(NODES_N), 0)
    }

    fn safety(&self) {
        // Safety measures
        if self.nodes_n < 3 {
            eprintln!("The number of nodes can't be lower than 3.");
            exit(1);
        }
    }

    fn new(nodes_n: usize) -> Self {
        let edges = vec![vec![false; nodes_n]; nodes_n];

        Self { nodes_n, edges }
    }

    fn manage_edge(&mut self, i: usize, j: usize, value: bool) {
        self.edges[i][j] = value;
        self.edges[j][i] = value;
    }

    fn generate(&mut self) {
        for i in 0..self.nodes_n {
            for j in i + 1..self.nodes_n {
                self.manage_edge(i, j, random::<bool>());
            }
        }
    }

    fn generate_with_given_kind(&mut self, kind: GraphKind, improvements: bool) {
        loop {
            self.generate();

            if improvements {
                match kind {
                    GraphKind::Hamiltonian => {
                        if self
                            .edges
                            .iter()
                            .flatten()
                            .map(|x| *x as usize)
                            .sum::<usize>()
                            / 2
                            > EDGES_THRESHOLD
                        {
                            continue;
                        }
                    }
                    GraphKind::NonHamiltonian => {
                        if (0..NODES_N)
                            .any(|i| self.edges[i].iter().map(|x| *x as usize).sum::<usize>() < 2)
                        {
                            continue;
                        }
                    }
                }
            }

            if self.get_cycle().is_empty() != (kind == GraphKind::Hamiltonian) {
                break;
            } else {
                *self = Graph::new(NODES_N);
            }
        }
    }
}

enum Mode {
    Demonstration,
    Generation,
}
const MODE: Mode = Mode::Generation;

#[macroquad::main("BasicShapes")]
async fn main() {
    match MODE {
        Mode::Demonstration => {
            set_fullscreen(true);
            next_frame().await;

            let mut nodes = Vec::with_capacity(NODES_N);
            let gap = (2.0 * PI) / NODES_N as f32;

            for i in 0..NODES_N {
                let x = GRAPH_RADIUS * (i as f32 * gap).cos() + screen_width() / 2.0;
                let y = GRAPH_RADIUS * (i as f32 * gap).sin() + screen_height() / 2.0;
                nodes.push(vec2(x, y))
            }

            let mut graph = Graph::new(NODES_N);
            graph.safety();

            let mut kind = GraphKind::Hamiltonian;

            let mut cycle = graph.get_cycle();

            loop {
                if is_key_pressed(KeyCode::R) {
                    graph.generate_with_given_kind(kind, true);

                    kind = match kind {
                        GraphKind::Hamiltonian => GraphKind::NonHamiltonian,
                        GraphKind::NonHamiltonian => GraphKind::Hamiltonian,
                    };

                    cycle = graph.get_cycle();
                }

                for node_pos in &nodes {
                    draw_circle(node_pos.x, node_pos.y, NODE_RADIUS, WHITE);
                }

                for i in 0..NODES_N {
                    for j in i + 1..NODES_N {
                        if graph.edges[i][j] {
                            draw_line(nodes[i].x, nodes[i].y, nodes[j].x, nodes[j].y, 5.0, WHITE);
                        }
                    }
                }

                if !cycle.is_empty() && is_key_down(KeyCode::E) {
                    for i in 0..NODES_N {
                        let node = cycle[i];
                        let next_node = cycle[i + 1];

                        draw_line(
                            nodes[node].x,
                            nodes[node].y,
                            nodes[next_node].x,
                            nodes[next_node].y,
                            15.0,
                            DARKGREEN,
                        );
                    }
                }

                next_frame().await
            }
        }
        Mode::Generation => {
            let mut hamiltonian_graphs = Vec::new();
            let mut non_hamiltonian_graphs = Vec::new();

            let start = Instant::now();

            for kind in GraphKind::ALL {
                for _ in 0..GRAPHS_N {
                    let mut graph = Graph::new(NODES_N);
                    graph.safety();
                    graph.generate_with_given_kind(kind, true);

                    match kind {
                        GraphKind::Hamiltonian => hamiltonian_graphs.push(graph.edges),
                        GraphKind::NonHamiltonian => non_hamiltonian_graphs.push(graph.edges),
                    }
                }
            }

            println!("{:?}", start.elapsed().as_secs_f32());

            for (graphs, name) in [
                (hamiltonian_graphs, "hamiltonian_graphs"),
                (non_hamiltonian_graphs, "non_hamiltonian_graphs"),
            ] {
                let content = Content {
                    info: Info {
                        nodes_n: NODES_N,
                        graphs_n: GRAPHS_N,
                    },
                    graphs: graphs.clone(),
                };

                std::fs::write(
                    format!("{}.json", name),
                    serde_json::to_string_pretty(&content).unwrap(),
                )
                .unwrap()
            }
        }
    }
}
