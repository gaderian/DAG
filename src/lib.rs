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

    fn split_remaining_v(vec: Vec<Vertex<T>>, edges: & Vec<Edge<T>>) -> (Vec<Vertex<T>>, Vec<Vertex<T>>) {
        let v_iter = vec.clone().into_iter();
        let remaining = vec.into_iter().filter(|&Vertex{id: my_id, weight: _}| {
                                     for i in 0..(edges.len()) {
                                         let Edge { from: _, to: destination, weight: _} = edges[i];
                                         if my_id == destination {
                                             return true
                                         }
                                     };
                                     false}).collect();
        let filtered = v_iter.filter(|&Vertex{id: my_id, weight: _}| {
                                     for i in 0..(edges.len()) {
                                         let Edge { from: _, to: destination, weight: _} = edges[i];
                                         if my_id == destination {
                                             return false
                                         }
                                     };
                                     true}).collect();
        (filtered, remaining)
    }

    pub fn topological_order(&self) -> Result<Vec<ID>, &'static str> {
        let mut result: Vec<ID> = Vec::new();
        let mut no_incomming: Vec<Vertex<T>> = Vec::new();
        let mut remaining_v: Vec<Vertex<T>> = self.vertices.clone();
        let mut remaining_e: Vec<Edge<T>> = self.edges.clone();

        let (mut hey, remaining) = DAG::split_remaining_v(remaining_v, &remaining_e);
        remaining_v = remaining;

        no_incomming.append(&mut hey);

        while let Some(current_node) = no_incomming.pop() {
            let Vertex{id: my_id, weight: _} = current_node;
            result.push(my_id);
            let mut i = 0;
            while i < remaining_e.len() {
                let Edge { from: origin, to: _, weight: _} = remaining_e[i];
                if origin == my_id {
                    remaining_e.swap_remove(i);
                    continue;
                }
                i += 1;
            }
            let (mut hey, remaining) = DAG::split_remaining_v(remaining_v, &remaining_e);
            remaining_v = remaining;
            no_incomming.append(&mut hey);
        }

        if !remaining_e.is_empty() {
            return Err("There exists a cycle!");
        }

        Ok(result)
    }

}

impl <T:Clone> DAGInterface<T> for DAG<T> {

    fn add_vertex(&mut self, w: T) -> ID {
        self.vertices.push(Vertex{id: self.next_id, weight: w});
        self.next_id += 1;
        self.next_id-1
    }

    fn add_edge(&mut self, a: ID, b: ID, w: T) -> Result<bool, &'static str> {
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

    #[test]
    fn test_edge_to_self() {
        let mut dag: DAG<u8> = DAG::new();
        let a = dag.add_vertex(5);

        //Err is the desired result from add.
        if let Ok(_) = dag.add_edge(a,a,19) {
            panic!("Should return error on edge to itself");
        }
    }

    #[test]
    fn test_topological() {
        let mut dag: DAG<u8> = DAG::new();
        let a = dag.add_vertex(5);
        let b = dag.add_vertex(8);

        dag.add_edge(a, b, 10);
        assert_eq!(Ok(vec![a,b]), dag.topological_order()); 

        let c = dag.add_vertex(10);
        
        dag.add_edge(c, a, 5);
        dag.add_edge(c, b, 7);
        assert_eq!(Ok(vec![c,a,b]), dag.topological_order());
    }
}
