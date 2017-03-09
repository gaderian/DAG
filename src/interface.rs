pub trait DAGInterface<T1: Clone, T2:Clone> {
    fn add_vertex(&mut self, w: T1) -> u64;

    fn add_edge(&mut self, a: u64, b: u64, w: T2) -> Result<bool, &'static str>;
}
