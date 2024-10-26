use algorithms::algorithms::selection::median_of_medians::select_kth;

fn main() {
    let mut arr = vec![12, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1, 2];
    let k = 5; // Find the 5th smallest element
    let result = select_kth(&mut arr, k);
    println!("The {}th smallest element is: {}", k, result);
}
