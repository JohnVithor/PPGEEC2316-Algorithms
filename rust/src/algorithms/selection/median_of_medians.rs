fn partition_around(arr: &mut [i32], x: usize) -> usize {
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

fn ref_sort_5(a: &mut i32, b: &mut i32, c: &mut i32, d: &mut i32, e: &mut i32) {
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

fn sort_5(arr: &mut [i32], arr_start: usize) {
    let arr = &mut arr[arr_start..];
    let size = arr.len();
    let g = size / 5;

    let (a, arr) = arr.split_at_mut(g);
    let (b, arr) = arr.split_at_mut(g);
    let (c, arr) = arr.split_at_mut(g);
    let (d, e) = arr.split_at_mut(g);
    for i in 0..g {
        let a = &mut a[i];
        let b = &mut b[i];
        let c = &mut c[i];
        let d = &mut d[i];
        let e = &mut e[i];
        ref_sort_5(a, b, c, d, e);
    }
}

pub fn select_kth(arr: &mut [i32], i: usize) -> i32 {
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
            return arr[arr_start];
        }
        arr_start += 1;
        size -= 1;
        i_mut -= 1;
    }

    let g = size / 5;

    sort_5(arr, arr_start);

    let x = select_kth(
        &mut arr[arr_start + 2 * g..arr_start + 3 * g],
        if g % 2 == 0 { g / 2 } else { g / 2 + 1 },
    );

    let x_pos = arr[arr_start..arr_start + size]
        .iter()
        .position(|&val| val == x)
        .unwrap();

    let q = partition_around(&mut arr[arr_start..arr_start + size], x_pos);

    match i_mut.cmp(&(q + 1)) {
        std::cmp::Ordering::Equal => arr[arr_start + q],
        std::cmp::Ordering::Less => select_kth(&mut arr[arr_start..arr_start + q], i_mut),
        std::cmp::Ordering::Greater => select_kth(
            &mut arr[arr_start + q + 1..arr_start + size],
            i_mut - (q + 1),
        ),
    }
}
