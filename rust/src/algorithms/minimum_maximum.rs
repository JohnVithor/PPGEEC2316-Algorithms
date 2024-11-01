pub fn minimum_maximum_naive<T: PartialOrd + Copy>(arr: &[T]) -> Option<(T, T)> {
    if arr.is_empty() {
        return None;
    }
    let mut min = arr[0];
    let mut max = arr[0];
    for &x in arr {
        if x < min {
            min = x;
        }
        if x > max {
            max = x;
        }
    }
    Some((min, max))
}

pub fn minimum_maximum<T: PartialOrd + Copy>(arr: &[T]) -> Option<(T, T)> {
    if arr.is_empty() {
        return None;
    }

    let mut min;
    let mut max;
    let i = if arr.len() % 2 == 0 {
        if arr[0] > arr[1] {
            min = arr[1];
            max = arr[0];
        } else {
            min = arr[0];
            max = arr[1];
        }
        2
    } else {
        min = arr[0];
        max = arr[0];
        1
    };
    for chunk in arr[i..].chunks_exact(2) {
        let i = chunk[0];
        let j = chunk[1];
        if i > j {
            if i > max {
                max = i;
            }
            if j < min {
                min = j;
            }
        } else {
            if j > max {
                max = j;
            }
            if i < min {
                min = i;
            }
        }
    }
    Some((min, max))
}
