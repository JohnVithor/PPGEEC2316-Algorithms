use super::{Interval, IntervalManager};

pub struct VecIntervalManager {
    data: Vec<Interval>,
}

impl IntervalManager for VecIntervalManager {
    fn add_user(&mut self, user: i32, size: usize) -> Option<&Interval> {
        for i in 0..self.data.len() {
            if self.data[i].user_id.is_none() && self.data[i].end - self.data[i].start >= size {
                let interval = Interval {
                    start: self.data[i].start + size,
                    end: self.data[i].end,
                    user_id: None,
                };
                self.data[i].end = self.data[i].start + size;
                self.data[i].user_id = Some(user);
                self.data.insert(i + 1, interval);
                return Some(&self.data[i]);
            }
        }
        None
    }

    fn remove_user(&mut self, user: i32) {
        for i in 0..self.data.len() {
            if self.data[i].user_id == Some(user) {
                self.data[i].user_id = None;
                if i > 0 && self.data[i - 1].user_id.is_none() {
                    self.data[i - 1].end = self.data[i].end;
                    self.data.remove(i);
                }
                if i < self.data.len() - 1 && self.data[i + 1].user_id.is_none() {
                    self.data[i + 1].start = self.data[i].start;
                    self.data.remove(i);
                }
            }
        }
    }

    fn get_interval(&self, user: i32) -> Option<&Interval> {
        self.data.iter().find(|&i| i.user_id == Some(user))
    }
}
