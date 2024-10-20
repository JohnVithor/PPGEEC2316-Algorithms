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

pub fn mergesort<T: Ord + Copy>(arr: &mut [T]) {
    if arr.len() <= 1 {
        return;
    }
    let mid = arr.len() / 2;
    let mut buffer = vec![arr[0]; arr.len()];
    mergesort(&mut arr[..mid]);
    mergesort(&mut arr[mid..]);
    merge(&arr[..mid], &arr[mid..], &mut buffer);
    arr.copy_from_slice(&buffer);
}
