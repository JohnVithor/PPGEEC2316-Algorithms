use std::collections::BTreeSet;

use super::{Interval, IntervalManager};

pub struct BTreeIntervalManager {
    data: BTreeSet<Interval>,
}

impl IntervalManager for BTreeIntervalManager {
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
