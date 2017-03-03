mod interface;
use interface::DAGInterface;

type ID = u64;

#[derive(PartialEq,Debug,Clone)]
struct Edge<T> {
    from: ID,
    to: ID,
    weight: T,
}

#[derive(PartialEq,Debug,Clone)]
struct Vertex<T> {
    id: ID,
    weight: T,
}

pub struct DAG<T:Clone> {
    vertices: Vec<Vertex<T>>,
    edges: Vec<Edge<T>>,
    next_id: ID,
}

impl <T:Clone> DAG<T> {
    pub fn new() -> DAG<T> {
        DAG {
            vertices: Vec::new(),
            edges: Vec::new(),
            next_id: 0
        }
    }

    fn topologogal_order(&self) -> Vec<ID> {
        let mut no_incomming: Vec<Vertex<T>> = Vec::new();
        let mut remaining_v: Vec<Vertex<T>> = self.vertices[..].to_owned();
        let mut remaining_e: Vec<Edge<T>> = self.edges[..].to_owned();
        vec![]
    }
}

impl <T:Clone> DAGInterface<T> for DAG<T> {

    fn add_vertex(&mut self, w: T) -> ID {
        self.vertices.push(Vertex{id: self.next_id, weight: w});
        self.next_id += 1;
        self.next_id-1
    }

    fn add_edge(&mut self, a: ID, b: ID, w: T) {
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
        let id = dag.add_vertex(8);

        assert_eq!(1, dag.vertices.len());
        assert_eq!(Some(Vertex {id: id, weight: 8}), dag.vertices.pop());
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
