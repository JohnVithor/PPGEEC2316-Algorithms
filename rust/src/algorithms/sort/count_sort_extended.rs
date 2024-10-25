fn array_max_value(arr: &[i64]) -> (i64, i64) {
    let (mut min, mut max) = (arr[0], arr[0]);
    for &x in arr.iter().skip(1) {
        if x > max {
            max = x;
        }
        if x < min {
            min = x;
        }
    }
    (min, max)
}

pub fn count_sort(arr: &mut [i64]) {
    let (min, max) = array_max_value(arr);
    let offset = 0 - min;
    let k = (max + offset + 1) as usize;
    let mut counter = vec![0usize; k];
    for &x in arr.iter() {
        counter[(x + offset) as usize] += 1;
    }
    let mut i = 0usize;
    let mut j = 0usize;
    while j < k {
        if counter[j] != 0 {
            arr[i] = j as i64 + offset;
            i += 1;
            counter[j] -= 1;
        } else {
            j += 1;
        }
    }
}
