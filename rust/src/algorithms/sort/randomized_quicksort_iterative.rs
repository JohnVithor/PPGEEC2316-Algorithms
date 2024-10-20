fn partition<T: Ord>(arr: &mut [T]) -> usize {
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

pub fn quicksort<T: Ord + Copy>(arr: &mut [T]) {
    if arr.len() > 1 {
        let mut stack = vec![(0, arr.len() - 1)];

        while let Some((l, h)) = stack.pop() {
            let p = partition(&mut arr[l..=h]) + l;

            if p > l {
                stack.push((l, p - 1));
            }

            if p < h {
                stack.push((p + 1, h));
            }
        }
    }
}
