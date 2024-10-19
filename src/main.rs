use rand::{prelude::SliceRandom, rngs::StdRng, seq::IteratorRandom, SeedableRng};
use std::{collections::HashSet, process::exit};

#[derive(PartialEq)]
enum GraphGenerationResult {
    Successful,
    Unsuccessful,
}

struct Graph {
    nodes_n: usize,
    edges_n: usize,
    edges: Vec<Vec<Option<bool>>>,
}

impl Graph {
    fn path_exists(&self, nodes: HashSet<usize>, current_node: usize) -> bool {
        if nodes.len() == 1 {
            let only_node = nodes.iter().nth(0).unwrap();

            return self.edges[current_node][*only_node].unwrap()
                && self.edges[*only_node][0].unwrap();
        }

        for node in &nodes {
            if self.edges[current_node][*node].unwrap() {
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

    fn debug(&self) {
        let mut degrees = vec![0; self.nodes_n];

        for i in 0..self.nodes_n {
            for j in 0..self.nodes_n {
                match self.edges[i][j] {
                    Some(edge) => {
                        if edge {
                            degrees[i] += 1;
                        }
                    }
                    None => {
                        eprintln!("None.");
                        exit(1);
                    }
                }
            }
        }

        let mut e = 0;

        for i in 0..self.nodes_n {
            if degrees[i] < 2 {
                eprintln!("Degree < 2.");
                exit(1);
            }

            e += degrees[i];
        }

        let m = e / 2;
        if self.edges_n != m {
            eprintln!("Wrong edges_n.");
            exit(1);
        }
    }

    fn safety(&self) {
        // Safety measures
        if self.nodes_n < 3 {
            eprintln!("The number of nodes can't be higher than 2.");
            exit(1);
        }

        // en.wikipedia.org/wiki/Combination
        if self.edges_n > (self.nodes_n * (self.nodes_n - 1)) / 2 {
            eprintln!("There can't be that many edges.");
            exit(1);
        }

        if self.edges_n < self.nodes_n {
            eprintln!("There can't be that few edges.");
            exit(1);
        }
    }

    fn new(nodes_n: usize, edges_n: usize) -> Self {
        let mut edges = vec![vec![None; nodes_n]; nodes_n];

        for i in 0..nodes_n {
            edges[i][i] = Some(false)
        }

        Self {
            nodes_n,
            edges_n,
            edges,
        }
    }

    fn manage_edge(&mut self, i: usize, j: usize, value: bool) {
        self.edges[i][j] = Some(value);
        self.edges[j][i] = Some(value);
    }

    fn generate(&mut self, rng: &mut StdRng) -> GraphGenerationResult {
        let mut nodes_shuffled = (0..self.nodes_n).collect::<Vec<_>>();
        nodes_shuffled.shuffle(rng);

        for i in nodes_shuffled {
            let already_connected = (0..self.nodes_n)
                .filter(|j| match self.edges[i][*j] {
                    Some(edge) => edge,
                    None => false,
                })
                .collect::<Vec<_>>();

            match already_connected.len() {
                0 => {
                    let other_edges = (0..self.nodes_n).filter(|j| *j != i);
                    let random_pair = other_edges.choose_multiple(rng, 2);

                    self.manage_edge(i, random_pair[0], true);
                    self.manage_edge(i, random_pair[1], true);
                }
                1 => {
                    let r = already_connected[0];
                    let other_edges = (0..self.nodes_n).filter(|j| *j != i && *j != r);
                    let random_edge = other_edges.choose(rng).unwrap();

                    self.manage_edge(i, random_edge, true);
                }
                _ => {}
            }
        }

        //for i in 0..self.nodes_n {
        //    let other_edges = (0..self.nodes_n).filter(|j| *j != i);
        //    let random_pair = other_edges.choose_multiple(rng, 2);
        //
        //    self.manage_edge(i, random_pair[0], true);
        //    self.manage_edge(i, random_pair[1], true);
        //}

        let mut real_edges_n = 0;

        for i in 0..self.nodes_n {
            for j in i + 1..self.nodes_n {
                if let Some(edge) = self.edges[i][j] {
                    if edge {
                        real_edges_n += 1;
                    }
                }
            }
        }

        if real_edges_n > self.edges_n {
            GraphGenerationResult::Unsuccessful
        } else {
            for i in 0..self.nodes_n {
                for j in 0..self.nodes_n {
                    if self.edges[i][j].is_none() {
                        self.edges[i][j] = Some(false);
                    }
                }
            }

            if real_edges_n == self.edges_n {
                GraphGenerationResult::Successful
            } else {
                let mut non_existent = Vec::new();

                for i in 0..self.nodes_n {
                    for j in 0..self.nodes_n {
                        if i < j && !self.edges[i][j].unwrap() {
                            non_existent.push((i, j));
                        }
                    }
                }

                let random = non_existent
                    .iter()
                    .choose_multiple(rng, self.edges_n - real_edges_n);

                for edge in random {
                    self.manage_edge(edge.0, edge.1, true);
                }

                GraphGenerationResult::Successful
            }
        }
    }
}

const N: usize = 10_000;

fn main() {
    let mut rng = StdRng::from_rng(&mut rand::thread_rng()).unwrap();

    let mut graph = Graph::new(10, 15);
    graph.safety();

    let mut successful_n = 0;
    let mut hamiltonian_n = 0;

    for _ in 0..N {
        if graph.generate(&mut rng) == GraphGenerationResult::Successful {
            graph.debug();

            successful_n += 1;
            if graph.is_hamiltonian() {
                hamiltonian_n += 1;
            }
        }
    }

    println!(
        "successful = {:?} hamiltonian = {:?}",
        successful_n, hamiltonian_n
    );

    //println!("{:?}", graph.is_hamiltonian());
    //println!("{:?}", graph.edges);

    //loop {
    //    if graph.generate(&mut rng) == GraphGenerationResult::Successful {
    //        break;
    //    }
    //}

    //graph.debug();
}
