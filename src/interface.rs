pub trait DAGInterface<T> {
    fn add_vertex(&mut self, w: T) -> u64;

    fn add_edge(&mut self, a: u64, b: u64, w: T) -> Result<bool, &'static str>;

    fn topological_order(&self) -> Result<Vec<u64>, &'static str>;

    fn weight_of_longest_path<F1, F2>(&self, from: u64, to: u64, v_sum: &F1, e_sum: &F2) -> Option<T>
        where F1: Fn(T) -> T,
              F2: Fn(T) -> T;
}
