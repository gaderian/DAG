pub trait DAGInterface<T> {
    fn add_vertex(&mut self, w: T) -> u64;

    fn add_edge(&mut self, a: u64, b: u64, w: T);
}
