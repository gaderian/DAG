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

pub struct DAG<T1:Clone, T2:Clone> {
    vertices: Vec<Vertex<T1>>,
    edges: Vec<Edge<T2>>,
    next_id: ID,
}

impl <T1:Clone, T2:Clone> DAG<T1,T2> {
    pub fn new() -> DAG<T1, T2> {
        DAG {
            vertices: Vec::new(),
            edges: Vec::new(),
            next_id: 0
        }
    }

    fn topologogal_order(&self) -> Vec<ID> {
        let mut no_incomming: Vec<Vertex<T1>> = Vec::new();
        let mut remaining_v: Vec<Vertex<T1>> = self.vertices[..].to_owned();
        let mut remaining_e: Vec<Edge<T2>> = self.edges[..].to_owned();
        vec![]
    }
}

impl <T1:Clone, T2:Clone> DAGInterface<T1,T2> for DAG<T1, T2> {

    fn add_vertex(&mut self, w: T1) -> ID {
        self.vertices.push(Vertex{id: self.next_id, weight: w});
        self.next_id += 1;
        self.next_id-1
    }

    fn add_edge(&mut self, a: ID, b: ID, w: T2) -> Result<bool, &'static str> {
        if a == b {
            return Err("false");
        }

        for i in 0..self.edges.len() {
            println!("{:?}", self.edges[i].from);
        }

        self.edges.push(Edge {from: a, to: b, weight: w});
        Ok(true)
    }
}
        

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let dag: DAG<u8,u8> = DAG::new();
        assert!(dag.vertices.is_empty());
        assert!(dag.edges.is_empty());
    }

    #[test]
    fn test_add_vertex() {
        let mut dag: DAG<u8,u8> = DAG::new();
        let id = dag.add_vertex(8);

        assert_eq!(1, dag.vertices.len());
        assert_eq!(Some(Vertex {id: id, weight: 8}), dag.vertices.pop());
    }

    #[test]
    fn test_add_edge() {
        let mut dag: DAG<u8,u8> = DAG::new();
        let a = dag.add_vertex(5);
        let b = dag.add_vertex(8);

        dag.add_edge(a, b, 10);
        assert_eq!(1, dag.edges.len());

        let tmp = Edge { from: a, to: b, weight: 10 };
        assert_eq!(tmp, dag.edges.pop().unwrap());
    }

    #[test]
    fn test_edge_to_self() {
        let mut dag: DAG<u8,u8> = DAG::new();
        let a = dag.add_vertex(5);

        //Err is the desired result from add.
        match dag.add_edge(a,a,19) {
            Err(_) => assert_eq!(1, 1),
            Ok(_) => assert_eq!(1, 2)
        }
    }
}
