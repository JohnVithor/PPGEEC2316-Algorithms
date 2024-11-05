#[derive(Debug)]
pub struct Interval {
    start: usize,
    end: usize,
    user_id: Option<i32>,
}

pub trait IntervalManager {
    fn add_user(&mut self, user: i32, size: usize) -> Option<&Interval>;
    fn remove_user(&mut self, user: i32);
    fn get_interval(&self, user: i32) -> Option<&Interval>;
}

// pub mod b_tree_interval;
// pub mod hash_interval;
// pub mod linked_list_interval;
pub mod vec_deque_interval;
pub mod vec_interval;
