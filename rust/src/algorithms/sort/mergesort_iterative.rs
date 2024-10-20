use std::fmt::Debug;

fn merge<T: Ord + Copy>(left: &[T], right: &[T], ret: &mut [T]) {
    let mut l = 0usize;
    let mut r = 0usize;
    let mut i = 0usize;
    while l < left.len() && r < right.len() {
        ret[i] = if left[l] < right[r] {
            l += 1;
            left[l - 1]
        } else {
            r += 1;
            right[r - 1]
        };
        i += 1;
    }
    if l < left.len() {
        while l < left.len() {
            ret[i] = left[l];
            i += 1;
            l += 1;
        }
    } else {
        while r < right.len() {
            ret[i] = right[r];
            i += 1;
            r += 1;
        }
    }
}

pub fn mergesort<T: Ord + Copy + Debug>(arr: &mut [T]) {
    if arr.len() <= 1 {
        return;
    }
    let mut curr_size = 1;
    let mut buffer = vec![arr[0]; arr.len()];
    while curr_size < arr.len() {
        let mut left = 0;
        while left < arr.len() - 1 {
            let mid = std::cmp::min(left + curr_size, arr.len() - 1);
            let right = std::cmp::min(left + 2 * curr_size - 1, arr.len() - 1);
            merge(
                &arr[left..mid],
                &arr[mid..=right],
                &mut buffer[left..=right],
            );
            arr[left..=right].copy_from_slice(&buffer[left..=right]);
            left += 2 * curr_size;
        }
        curr_size *= 2;
    }
}
