fn partition_around<T: Ord>(arr: &mut [T], x: usize) -> usize {
    let size = arr.len();
    arr.swap(x, size - 1);
    let mut i = 0;
    for j in 0..size - 1 {
        if arr[j] <= arr[size - 1] {
            arr.swap(i, j);
            i += 1;
        }
    }
    arr.swap(i, size - 1);
    i
}

fn ref_sort_5<T: Ord>(a: &mut T, b: &mut T, c: &mut T, d: &mut T, e: &mut T) {
    if *a > *b {
        std::mem::swap(a, b);
    }
    if *c > *d {
        std::mem::swap(c, d);
    }
    if *c > *e {
        std::mem::swap(c, e);
    }
    if *d > *e {
        std::mem::swap(d, e);
    }
    if *a > *c {
        std::mem::swap(a, c);
    }
    if *b > *d {
        std::mem::swap(b, d);
    }
    if *b > *c {
        std::mem::swap(b, c);
    }
    if *d > *e {
        std::mem::swap(d, e);
    }
    if *c > *d {
        std::mem::swap(c, d);
    }
}

fn sort_5<T: Ord>(arr: &mut [T], arr_start: usize) {
    let arr = &mut arr[arr_start..];
    let size = arr.len();
    let g = size / 5;

    if let [a, b, c, d, e] = arr.chunks_exact_mut(g).collect::<Vec<_>>().as_mut_slice() {
        for i in 0..g {
            ref_sort_5(&mut a[i], &mut b[i], &mut c[i], &mut d[i], &mut e[i]);
        }
    } else {
        panic!("Expected a slice of 5 elements");
    }
}

pub fn select_kth<T: Ord + Copy>(arr: &mut [T], i: usize) -> &T {
    let mut size = arr.len();
    let mut arr_start = 0;
    let mut i_mut = i;

    while size % 5 != 0 {
        for j in 1..size {
            if arr[arr_start] > arr[arr_start + j] {
                arr.swap(arr_start, arr_start + j);
            }
        }
        if i_mut == 1 {
            return &arr[arr_start];
        }
        arr_start += 1;
        size -= 1;
        i_mut -= 1;
    }

    let g = size / 5;

    sort_5(arr, arr_start);

    let &x = select_kth(
        &mut arr[arr_start + 2 * g..arr_start + 3 * g],
        if g % 2 == 0 { g / 2 } else { g / 2 + 1 },
    );

    let x_pos = arr[arr_start..arr_start + size]
        .iter()
        .position(|&val| val == x)
        .unwrap();

    let q = partition_around(&mut arr[arr_start..arr_start + size], x_pos);

    match i_mut.cmp(&(q + 1)) {
        std::cmp::Ordering::Equal => &arr[arr_start + q],
        std::cmp::Ordering::Less => select_kth(&mut arr[arr_start..arr_start + q], i_mut),
        std::cmp::Ordering::Greater => select_kth(
            &mut arr[arr_start + q + 1..arr_start + size],
            i_mut - (q + 1),
        ),
    }
}
