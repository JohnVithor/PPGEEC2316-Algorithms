use algorithms::algorithms::sort::{mergesort::mergesort, quicksort::quicksort};
use rand::{distributions::Uniform, Rng, SeedableRng};

fn main() {
    let rng = rand::rngs::StdRng::seed_from_u64(0);
    let mut arr = rng
        .sample_iter(Uniform::new(i32::MIN, i32::MAX))
        .take(20_000_000)
        .collect::<Vec<i32>>();
    // println!("{:?}", arr);
    let now = std::time::Instant::now();
    quicksort(&mut arr);
    println!("{:?}", now.elapsed().as_secs_f64());
    // println!("{:?}", arr);
    for i in 0..arr.len() - 1 {
        assert!(
            arr[i] <= arr[i + 1],
            "{} {} > {} {}",
            arr[i],
            i,
            arr[i + 1],
            i + 1
        );
    }
}
