pub fn radix_sort(arr: &mut [usize]) {
    let mut max = arr[0];
    for &x in arr.iter() {
        if x > max {
            max = x;
        }
    }
    let mut exp = 1;
    while max / exp > 0 {
        let mut counter = [0; 10];
        for &x in arr.iter() {
            counter[(x / exp) % 10] += 1;
        }
        for i in 1..10 {
            counter[i] += counter[i - 1];
        }
        let mut output = vec![0; arr.len()];
        for &x in arr.iter().rev() {
            output[counter[(x / exp) % 10] - 1] = x;
            counter[(x / exp) % 10] -= 1;
        }
        arr.copy_from_slice(&output[..arr.len()]);
        exp *= 10;
    }
}
