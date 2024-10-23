fn randomized_partition<T: Ord>(arr: &mut [T]) -> usize {
    let len = arr.len();
    arr.swap(fastrand::usize(0..len), len - 1);
    let mut i = 0;
    for j in 0..len - 1 {
        if arr[j] <= arr[len - 1] {
            arr.swap(i, j);
            i += 1;
        }
    }
    arr.swap(i, len - 1);
    i
}

pub fn randomized_select_kth<T: Ord>(arr: &mut [T], i: usize) -> &T {
    if arr.len() == 1 {
        return &arr[0];
    }
    let q = randomized_partition(arr);
    let k = q + 1;
    match i.cmp(&k) {
        std::cmp::Ordering::Equal => &arr[q],
        std::cmp::Ordering::Less => randomized_select_kth(&mut arr[..q], i),
        std::cmp::Ordering::Greater => randomized_select_kth(&mut arr[q + 1..], i - k),
    }
}
