#include "utils.hpp"

std::vector<int> create_desc_array(unsigned int n) {
    std::vector<int> arr = std::vector<int>(n);
    for (auto it = arr.begin(); it < arr.end(); ++it){
        *it = n--;
    }
    return arr;
}

void fill_random(std::vector<int> &arr, int seed) {
    std::srand(seed);
    std::generate(arr.begin(), arr.end(), std::rand);
}

void print_array(std::vector<int> &arr) {
    for (auto it = arr.begin(); it < arr.end(); ++it){
        std::cout << *it << " ";
    }
    std::cout << std::endl;
}

void validate(std::vector<int> &arr) {
    for (auto it = arr.begin(); it < arr.end()-1; ++it){
        if (*it > *(it + 1)) {
            std::cout << "Erro de ordenação: " << *it << " < " << *(it + 1) << std::endl;
            return;
        }
    }
    std::cout << "Ordenação correta" << std::endl;
}