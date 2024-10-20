fn array_max_value(arr: &[u32]) -> u32 {
    let mut max = arr[0];
    for &x in arr.iter() {
        if x > max {
            max = x;
        }
    }
    max
}

pub fn count_sort(arr: &mut [u32]) {
    let k = (array_max_value(arr) + 1) as usize;
    let mut counter = vec![0; k];
    for &x in arr.iter() {
        counter[x as usize] += 1;
    }
    let mut i = 0usize;
    let mut j = 0usize;
    while j < k {
        if counter[j] != 0 {
            arr[i] = j as u32;
            i += 1;
            counter[j] -= 1;
        } else {
            j += 1;
        }
    }
}
