mod interface;
use interface::DAGInterface;

struct edge<T> {
    from: u64,
    to: u64,
    weight: T,
}

pub struct DAG<T> {
    vertices: Vec<T>,
    edges: Vec<edge<T>>,
}

impl <T> DAG<T> {
    fn new() -> DAG<T> {
        DAG {
            vertices: Vec::new(),
            edges: Vec::new()
        }
    }
}

impl <T> DAGInterface<T> for DAG<T> {

    fn add_vertex(&mut self, w: T) -> u64 {
        self.vertices.push(w);
        self.vertices.len() as u64
    }

    fn add_edge(&mut self, a: u64, b: u64, w: T) {
        self.edges.push(edge {from: a, to: b, weight: w});
    }
}
        

#[cfg(test)]
mod tests {

    #[test]
    fn test_dummy() {
        assert_eq!(1, 1);
    }
}
