pub struct Vertex<T> {
    data: T,
    edges: Vec<usize>,
}

pub struct Graph<T> {
    vertices: Vec<Vertex<T>>,
}
