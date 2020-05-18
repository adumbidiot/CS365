use std::collections::BinaryHeap;

#[derive(Copy, Clone, Eq, PartialEq)]
struct State {
    cost: usize,
    position: usize,
}

impl Ord for State {
    fn cmp(&self, other: &State) -> std::cmp::Ordering {
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| self.position.cmp(&other.position))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &State) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug)]
struct Edge {
    node: usize,
    cost: usize,
}

#[derive(Debug)]
struct Graph {
    nodes: Vec<String>,
    list: Vec<Vec<Edge>>,
}

impl Graph {
    pub fn new() -> Self {
        Self {
            nodes: Vec::new(),
            list: Vec::new(),
        }
    }

    pub fn get_node_name(&self, i: usize) -> Option<&str> {
        self.nodes.get(i).map(|s| s.as_str())
    }

    pub fn get_node(&mut self, name: &str) -> Option<usize> {
        self.nodes.iter().position(|n| n == name)
    }

    pub fn get_or_insert_node(&mut self, name: &str) -> usize {
        if let Some(n) = self.get_node(name) {
            n
        } else {
            let ret = self.nodes.len();
            self.nodes.push(name.into());
            self.list.push(Vec::new());
            ret
        }
    }

    pub fn add_bidirectional_edge(&mut self, src: usize, dest: usize, cost: usize) {
        self.list[src].push(Edge { node: dest, cost });
        self.list[dest].push(Edge { node: src, cost });
    }
}

#[derive(Debug)]
struct Path {
    path: Vec<usize>,
    distance: Vec<usize>,
    cost: usize,
}

fn load_graph(mut input: &str) -> Option<Graph> {
    input = input.trim();
    let mut graph = Graph::new();

    for line in input.lines() {
        let mut iter = line.split(' ');
        let src = iter.next()?;
        let dest = iter.next()?;
        let cost = iter.next()?.parse::<usize>().ok()?;

        let src = graph.get_or_insert_node(src);
        let dest = graph.get_or_insert_node(dest);

        graph.add_bidirectional_edge(src, dest, cost);
    }

    Some(graph)
}

fn find_shortest_path(graph: &Graph, start: usize, end: usize) -> Option<Path> {
    let mut distance: Vec<_> = (0..graph.list.len()).map(|_| None).collect();
    let mut parent: Vec<_> = (0..graph.list.len()).map(|_| None).collect();

    let mut heap = BinaryHeap::new();
    distance[start] = Some(0);
    heap.push(State {
        cost: 0,
        position: start,
    });

    while let Some(State { cost, position }) = heap.pop() {
        if distance[position].map_or(false, |distance| cost > distance) {
            continue;
        }

        for edge in graph.list[position].iter() {
            let next = State {
                cost: cost + edge.cost,
                position: edge.node,
            };

            if distance[next.position].map_or(true, |distance| next.cost < distance) {
                heap.push(next);
                distance[next.position] = Some(next.cost);
                parent[next.position] = Some(position);
            }
        }
    }

    let cost = distance[end]?;
    let mut edge_index = parent[end]?;
    let mut path = vec![edge_index];
    let mut dist = vec![cost];

    while let Some(index) = parent[edge_index] {
        path.push(index);
        dist.push(distance[edge_index]?);
        edge_index = index;
    }
    dist.push(distance[edge_index]?);

    path.reverse();
    dist.reverse();

    Some(Path {
        cost,
        path,
        distance: dist,
    })
}

fn main() {
    let data = match std::fs::read_to_string("input.txt") {
        Ok(f) => f,
        Err(e) => {
            eprintln!("Failed to open 'input.txt': {:#?}", e);
            return;
        }
    };

    let mut graph = match load_graph(&data) {
        Some(d) => d,
        None => {
            eprintln!("Failed to parse input graph");
            return;
        }
    };

    let start = graph.get_or_insert_node("a");
    let end = graph.get_or_insert_node("z");
    let path = find_shortest_path(&graph, start, end);

    match path {
        Some(path) => {
            println!("Located a minimum path of cost: {}", path.cost);
            print!("a (0) -> ");
            for (node_index, cost) in path.path.iter().zip(path.distance.iter()).skip(1) {
                print!(
                    "{} ({}) -> ",
                    graph.get_node_name(*node_index).unwrap(),
                    cost
                );
            }
            println!("z ({})", path.distance.last().unwrap());
        }
        None => {
            eprintln!("There is no path from 'a' to 'z'.");
        }
    }
}
