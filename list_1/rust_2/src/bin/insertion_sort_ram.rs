use std::env::args;

use rand::{prelude::Distribution, SeedableRng};

fn insertion_sort(arr: &mut [i32]) -> usize {
    let mut count = 1; // inicialização da variável i
    for i in 1..arr.len() {
        count += 3; // comparação com arr.len(), acesso a arr[i] e atribuição a key
        let key = arr[i];
        count += 1; // atribuição a j
        let mut j = i;
        while j > 0 && arr[j - 1] > key {
            count += 4; // comparação j > 0, subtração j-1, acesso arr[j-1] e comparação arr[j-1] > key
            count += 4; // subtração j-1, acesso arr[j-1] e arr[j], além da atribuição arr[j-1] a arr[j]
            arr[j] = arr[j - 1];
            count += 2; // decremento e atribuição ao j
            j -= 1;
        }
        count += 4; // comparação j > 0, subtração j-1, acesso arr[j-1] e comparação arr[j-1] > key ao sair do laço
        count += 2; // acesso arr[j] e atribuição key a arr[j]
        arr[j] = key;
        count += 2; // incremento e atribuição do i
    }
    count += 1; // comparação i < arr.len() ao sair do laço
    count
}

fn main() {
    let prog_name = args().next().unwrap();
    let n = args()
        .nth(1)
        .unwrap_or_else(|| panic!("Uso: {prog_name} <n> <seed> (n > 1 e seed >=0)"))
        .parse::<usize>()
        .expect("<n> deve ser um número natural maior que 1");
    if n < 2 {
        panic!("<n> deve ser um número natural maior que 1");
    }
    let seed = args()
        .nth(2)
        .unwrap_or_else(|| panic!("Uso: {prog_name} <n> <seed> (n > 1 e seed >=0)"))
        .parse::<u64>()
        .expect("<seed> deve ser um número natural");

    let rng = rand::rngs::SmallRng::seed_from_u64(seed);
    let uniform = rand::distributions::Uniform::new(0, n as i32);

    let mut arr = uniform.sample_iter(rng).take(n).collect::<Vec<i32>>();

    let count = insertion_sort(&mut arr);

    println!("{}", count);
}
