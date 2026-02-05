use std::collections::{HashMap, VecDeque};

struct Graph {
    adj: HashMap<String, Vec<String>>,
}

impl Graph {
    fn new() -> Self {
        Self { adj: HashMap::new() }
    }

    fn add_edge(&mut self, from: String, to: String) {
        self.adj.entry(from).or_insert_with(Vec::new).push(to);
    }

    fn bfs(&self, start: &str, end: &str) -> Option<Vec<String>> {
        let mut queue = VecDeque::new();
        let mut visited = HashMap::new();

        queue.push_back(start.to_string());
        visited.insert(start.to_string(), None);

        while let Some(node) = queue.pop_front() {
            if node == end {
                return Some(self.reconstruct_path(&visited, end));
            }

            if let Some(neighbors) = self.adj.get(&node) {
                for neighbor in neighbors {
                    if !visited.contains_key(neighbor) {
                        visited.insert(neighbor.clone(), Some(node.clone()));
                        queue.push_back(neighbor.clone());
                    }
                }
            }
        }

        None
    }

    fn reconstruct_path(&self, visited: &HashMap<String, Option<String>>, end: &str) -> Vec<String> {
        let mut path = vec![end.to_string()];
        let mut current = end;

        while let Some(Some(parent)) = visited.get(current) {
            path.push(parent.clone());
            current = parent;
        }

        path.reverse();
        path
    }
}
