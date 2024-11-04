use std::collections::HashSet;

use super::{Interval, IntervalManager};

pub struct HashIntervalManager {
    data: HashSet<Interval>,
}

impl IntervalManager for HashIntervalManager {
    fn add_user(&mut self, user: i32, size: usize) -> Option<&Interval> {
        todo!()
    }

    fn remove_user(&mut self, user: i32) {
        todo!()
    }

    fn get_interval(&self, user: i32) -> Option<&Interval> {
        todo!()
    }
}
