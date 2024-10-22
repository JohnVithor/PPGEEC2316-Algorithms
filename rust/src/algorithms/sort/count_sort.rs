fn array_max_value(arr: &[usize]) -> usize {
    let mut max = arr[0];
    for &x in arr.iter() {
        if x > max {
            max = x;
        }
    }
    max
}

pub fn count_sort(arr: &mut [usize]) {
    let k = array_max_value(arr) + 1;
    let mut counter = vec![0usize; k];
    for &x in arr.iter() {
        counter[x] += 1;
    }
    let mut i = 0usize;
    let mut j = 0usize;
    while j < k {
        if counter[j] != 0 {
            arr[i] = j;
            i += 1;
            counter[j] -= 1;
        } else {
            j += 1;
        }
    }
}
