use rand::{rngs::StdRng, seq::IteratorRandom, SeedableRng};
use serde::{Deserialize, Serialize};
use std::{cmp::Ordering, collections::HashSet, process::exit, time::Instant};

const NODES_N: usize = 10;
const EDGES_N: usize = 15;
const GRAPHS_N: usize = 100_000;

#[derive(Serialize, Deserialize)]
struct Graph {
    nodes_n: usize,
    edges_n: usize,
    edges: Vec<Vec<bool>>,
}

#[derive(Serialize, Deserialize)]
struct Info {
    nodes_n: usize,
    edges_n: usize,
    graphs_n: usize,
}

#[derive(Serialize, Deserialize)]
struct Content {
    info: Info,
    graphs: Vec<Vec<Vec<bool>>>,
}

impl Graph {
    fn path_exists(&self, nodes: HashSet<usize>, current_node: usize) -> bool {
        if nodes.is_empty() {
            return self.edges[current_node][0];
        }

        for node in &nodes {
            if self.edges[current_node][*node] {
                let mut new_set = nodes.clone();
                new_set.remove(node);

                if self.path_exists(new_set, *node) {
                    return true;
                }
            }
        }

        false
    }

    fn is_hamiltonian(&self) -> bool {
        self.path_exists((1..self.nodes_n).collect::<HashSet<_>>(), 0)
    }

    fn safety(&self) {
        // Safety measures
        if self.nodes_n < 3 {
            eprintln!("The number of nodes can't be lower than 3.");
            exit(1);
        }

        if self.edges_n < self.nodes_n {
            eprintln!("There can't be that few edges.");
            exit(1);
        }

        // en.wikipedia.org/wiki/Combination
        if self.edges_n > (self.nodes_n * (self.nodes_n - 1)) / 2 {
            eprintln!("There can't be that many edges.");
            exit(1);
        }
    }

    fn new(nodes_n: usize, edges_n: usize) -> Self {
        let edges = vec![vec![false; nodes_n]; nodes_n];

        Self {
            nodes_n,
            edges_n,
            edges,
        }
    }

    fn manage_edge(&mut self, i: usize, j: usize, value: bool) {
        self.edges[i][j] = value;
        self.edges[j][i] = value;
    }

    fn generate(&mut self, rng: &mut StdRng) {
        let mut edges_n;

        loop {
            // Make 2 random edges out of each node
            for i in 0..self.nodes_n {
                let other_edges = (0..self.nodes_n).filter(|j| *j != i);
                let random_pair = other_edges.choose_multiple(rng, 2);

                self.manage_edge(i, random_pair[0], true);
                self.manage_edge(i, random_pair[1], true);
            }

            edges_n = self
                .edges
                .iter()
                .flatten()
                .map(|y| *y as usize)
                .sum::<usize>()
                / 2;

            match edges_n.cmp(&self.edges_n) {
                Ordering::Equal => {
                    return;
                }
                Ordering::Less => {
                    let mut non_existent = Vec::new();

                    for i in 0..self.nodes_n {
                        for j in i + 1..self.nodes_n {
                            if !self.edges[i][j] {
                                non_existent.push((i, j));
                            }
                        }
                    }

                    let random = non_existent
                        .iter()
                        .choose_multiple(rng, self.edges_n - edges_n);

                    for (i, j) in random {
                        self.manage_edge(*i, *j, true);
                    }

                    return;
                }
                Ordering::Greater => {
                    // We could theoretically remove the unneeded edges,
                    // but it's easier to start over
                    *self = Graph::new(self.nodes_n, self.edges_n);
                }
            }
        }
    }
}

fn main() {
    let mut rng = StdRng::from_rng(&mut rand::thread_rng()).unwrap();

    //let mut hamiltonian_n = 0;

    let mut hamiltonian_graphs = Vec::with_capacity(NODES_N);
    let mut non_hamiltonian_graphs = Vec::with_capacity(EDGES_N);

    let start = Instant::now();

    loop {
        let mut graph = Graph::new(NODES_N, EDGES_N);
        graph.safety();
        graph.generate(&mut rng);

        if graph.is_hamiltonian() {
            if hamiltonian_graphs.len() < GRAPHS_N {
                hamiltonian_graphs.push(graph.edges);
            }
        } else {
            if non_hamiltonian_graphs.len() < GRAPHS_N {
                non_hamiltonian_graphs.push(graph.edges);
            }
        }

        if hamiltonian_graphs.len() == GRAPHS_N && non_hamiltonian_graphs.len() == GRAPHS_N {
            break;
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
                edges_n: EDGES_N,
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
