mod interface;
use interface::DAGInterface;

#[derive(PartialEq,Debug)]
struct Edge<T> {
    from: u64,
    to: u64,
    weight: T,
}

pub struct DAG<T> {
    vertices: Vec<T>,
    edges: Vec<Edge<T>>,
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
        self.edges.push(Edge {from: a, to: b, weight: w});
    }
}
        

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let dag: DAG<u8> = DAG::new();
        assert!(dag.vertices.is_empty());
        assert!(dag.edges.is_empty());
    }

    #[test]
    fn test_add_vertex() {
        let mut dag: DAG<u8> = DAG::new();
        dag.add_vertex(8);

        assert_eq!(1, dag.vertices.len());
        assert_eq!(Some(8), dag.vertices.pop()); 
    }

    #[test]
    fn test_add_edge() {
        let mut dag: DAG<u8> = DAG::new();
        let a = dag.add_vertex(5);
        let b = dag.add_vertex(8);

        dag.add_edge(a, b, 10);
        assert_eq!(1, dag.edges.len());

        let tmp = Edge { from: a, to: b, weight: 10 };
        assert_eq!(tmp, dag.edges.pop().unwrap());
    }
}
