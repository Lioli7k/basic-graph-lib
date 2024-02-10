use std::{
    collections::{HashMap, HashSet, VecDeque},
    fmt::{Debug, Display},
};

mod serde;

pub type GraphId = u64;

#[derive(Debug, Clone)]
pub struct Graph<T> {
    nodes: HashMap<GraphId, T>,
    edges: HashSet<Edge>,
}

impl<T> Graph<T> {
    pub fn new() -> Self {
        Self {
            nodes: HashMap::new(),
            edges: HashSet::new(),
        }
    }

    pub fn add_node(&mut self, id: GraphId, value: T) {
        self.nodes.entry(id).or_insert(value);
    }

    pub fn get_node(&self, id: GraphId) -> Option<GraphNode<&T>> {
        self.nodes.get(&id).map(|value| GraphNode {
            id,
            value,
            neighbours: self
                .edges
                .iter()
                .filter_map(|edge| if edge.from == id { Some(edge.to) } else { None })
                .collect(),
        })
    }

    pub fn delete_node(&mut self, id: GraphId) {
        self.edges.retain(|edge| edge.from != id && edge.to != id);
        self.nodes.remove(&id);
    }

    pub fn add_edge(&mut self, from: GraphId, to: GraphId) {
        if self.nodes.contains_key(&from) && self.nodes.contains_key(&to) {
            self.edges.insert(Edge { from, to });
        }
    }

    pub fn delete_edge(&mut self, from: GraphId, to: GraphId) {
        self.edges.retain(|edge| edge.from != from || edge.to != to);
    }

    pub fn bfs(&self, source: GraphId)
    where
        T: Display,
    {
        let mut visited = HashSet::new();
        let mut queue = VecDeque::from([source]);
        while !queue.is_empty() {
            let id = queue.pop_front().unwrap_or_default();
            if !visited.contains(&id) {
                visited.insert(id);
                let node = if let Some(node) = self.get_node(id) {
                    node
                } else {
                    eprintln!("Error: Tried to access nonexistent node");
                    continue;
                };

                println!(
                    "ID: {}\nValue: {}\nNeighbours: {}\n",
                    node.id,
                    &node.value,
                    node.neighbours
                        .iter()
                        .map(ToString::to_string)
                        .collect::<Vec<String>>()
                        .join(", ")
                );

                queue.extend(node.neighbours);
            }
        }
    }
}

impl<T> Default for Graph<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T, const N: usize, const M: usize> From<([(GraphId, T); N], [(GraphId, GraphId); M])>
    for Graph<T>
{
    fn from((nodes, edges): ([(GraphId, T); N], [(GraphId, GraphId); M])) -> Self {
        let mut graph = Graph::new();
        for (id, value) in nodes {
            graph.add_node(id, value);
        }
        for (from, to) in edges {
            graph.add_edge(from, to);
        }

        graph
    }
}

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
struct Edge {
    from: GraphId,
    to: GraphId,
}

#[derive(Debug, Clone)]
pub struct GraphNode<T> {
    id: GraphId,
    value: T,
    neighbours: Vec<GraphId>,
}

impl<T> GraphNode<T> {
    pub fn id(&self) -> &GraphId {
        &self.id
    }

    pub fn value(&self) -> &T {
        &self.value
    }

    pub fn neighbour_ids(&self) -> &[GraphId] {
        &self.neighbours
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_graph_string() {
        let graph: Graph<String> = Graph::new();
        assert!(graph.nodes.is_empty(), "Expected nodes to be empty");
        assert!(graph.edges.is_empty(), "Expected edges to be empty");
    }

    #[test]
    fn create_graph_i32() {
        let graph: Graph<i32> = Graph::new();
        assert!(graph.nodes.is_empty(), "Expected nodes to be empty");
        assert!(graph.edges.is_empty(), "Expected edges to be empty");
    }

    #[test]
    fn create_graph_from_empty() {
        let graph: Graph<String> = Graph::from(([], []));
        assert!(graph.nodes.is_empty(), "Expected nodes to be empty");
        assert!(graph.edges.is_empty(), "Expected edges to be empty");
    }

    #[test]
    fn create_graph_from_nodes_and_edges() {
        let graph: Graph<String> = get_test_graph();
        assert_eq!(graph.nodes.len(), 7, "Nodes count mismatch");
        assert_eq!(graph.edges.len(), 10, "Edges count mismatch");
    }

    #[test]
    fn add_node() {
        let mut graph: Graph<i32> = Graph::new();
        graph.add_node(1, 2);
        assert_eq!(graph.nodes.len(), 1, "Expected only 1 node to be created");
        assert_eq!(
            graph.nodes.get(&1),
            Some(&2),
            "Stored value doesn't match original"
        );
    }

    #[test]
    fn add_node_discard_if_exists() {
        let mut graph: Graph<i32> = Graph::new();
        graph.add_node(1, 2);
        graph.add_node(1, 3);
        assert_eq!(graph.nodes.len(), 1, "Expected only 1 node to be created");
        assert_eq!(
            graph.nodes.get(&1),
            Some(&2),
            "Stored value doesn't match original"
        );
    }

    #[test]
    fn get_node_existing() {
        let graph: Graph<String> = get_test_graph();
        let node = graph.get_node(7);
        assert!(node.is_some(), "Node is empty");

        let node = node.unwrap();
        assert_eq!(node.id, 7, "Node ID doesn't match");
        assert_eq!(node.value, "September", "Node value doesn't match");
        assert_eq!(
            HashSet::from_iter(node.neighbours),
            HashSet::from([1, 5, 6]),
            "Node neighbours doesn't match"
        );
    }

    #[test]
    fn get_node_nonexistent() {
        let graph: Graph<String> = get_test_graph();
        let node = graph.get_node(9);
        assert!(node.is_none(), "Expected node to be empty");
    }

    #[test]
    fn delete_node_existing() {
        let mut graph: Graph<i32> = Graph::new();
        graph.add_node(1, 2);
        graph.delete_node(1);
        assert!(graph.nodes.is_empty(), "Expected node to be deleted");
    }

    #[test]
    fn delete_node_with_edges() {
        let mut graph: Graph<String> = get_test_graph();
        graph.delete_node(7);
        assert_eq!(graph.nodes.len(), 6, "Nodes count mismatch");
        assert_eq!(graph.edges.len(), 7, "Edges count mismatch");
    }

    #[test]
    fn delete_node_nonexistent() {
        let mut graph: Graph<i32> = Graph::new();
        graph.add_node(1, 2);
        graph.delete_node(9);
        assert_eq!(graph.nodes.len(), 1, "Node is unexpectedly deleted");
    }

    #[test]
    fn add_edge_valid() {
        let mut graph: Graph<String> = get_test_graph();
        graph.add_edge(2, 4);
        assert_eq!(graph.edges.len(), 11, "Edges count didn't increased");
    }

    #[test]
    fn add_edge_invalid_from() {
        let mut graph: Graph<String> = get_test_graph();
        graph.add_edge(9, 4);
        assert_eq!(graph.edges.len(), 10, "Edges count changed");
    }

    #[test]
    fn add_edge_invalid_to() {
        let mut graph: Graph<String> = get_test_graph();
        graph.add_edge(2, 9);
        assert_eq!(graph.edges.len(), 10, "Edges count changed");
    }

    #[test]
    fn delete_edge_existing() {
        let mut graph: Graph<String> = get_test_graph();
        graph.delete_edge(1, 2);
        assert_eq!(graph.edges.len(), 9, "Edges count mismatch");
    }

    #[test]
    fn delete_edge_nonexistent() {
        let mut graph: Graph<String> = get_test_graph();
        graph.delete_edge(2, 4);
        assert_eq!(graph.edges.len(), 10, "Edges count changed");
    }

    fn get_test_graph() -> Graph<String> {
        Graph::from((
            [
                (1, "January".to_string()),
                (2, "March".to_string()),
                (3, "April".to_string()),
                (4, "May".to_string()),
                (5, "December".to_string()),
                (6, "June".to_string()),
                (7, "September".to_string()),
            ],
            [
                (1, 2),
                (3, 2),
                (4, 3),
                (5, 1),
                (5, 3),
                (6, 3),
                (6, 1),
                (7, 5),
                (7, 6),
                (7, 1),
            ],
        ))
    }
}
