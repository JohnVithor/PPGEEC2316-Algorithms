use crate::data_structures::matrix::Matrix;

pub struct Vertex<T> {
    data: T,
}

pub struct Graph<'a, T> {
    vertices: Matrix<'a, Vertex<T>>,
}
