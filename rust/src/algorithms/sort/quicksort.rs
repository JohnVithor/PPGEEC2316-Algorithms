fn partition<T: Ord>(arr: &mut [T]) -> usize {
    let len = arr.len();
    // arr.swap(len / 2, len - 1);
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

pub fn quicksort<T: Ord>(arr: &mut [T]) {
    if arr.len() > 1 {
        let pivot = partition(arr);
        quicksort(&mut arr[0..pivot]);
        quicksort(&mut arr[pivot + 1..]);
    }
}
