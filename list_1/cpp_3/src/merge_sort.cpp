#include <iostream>
#include <vector>
#include <iomanip>
#include <chrono>
#include "utils.hpp"

using Iterator = std::vector<int>::iterator;

void merge(Iterator left_begin, Iterator left_end, Iterator right_begin, Iterator right_end, Iterator ret) {
    while (left_begin != left_end && right_begin < right_end) {
        *ret++ = (*left_begin < *right_begin) ? *left_begin++ : *right_begin++;
    }

    while (left_begin != left_end){
        *ret++ = *left_begin++;
    }
    while (right_begin < right_end){
        *ret++ = *right_begin++;
    }
}

void merge_sort(Iterator begin, Iterator end) {
    unsigned int size = std::distance(begin, end);
    unsigned int mid = size / 2;
    if (mid == 0) {
        return;
    }
    merge_sort(begin, begin + mid);
    merge_sort(begin + mid, end);

    std::vector<int> aux = std::vector<int>(begin, end);

    merge(aux.begin(), aux.begin()+mid, aux.begin() + mid, aux.end(), begin);
}


int main(int argc, char** argv) {
    if (argc != 3) {
        std::cout << "Uso: " << argv[0] << " <n> <seed> (n > 1 e seed >=0)" << std::endl;
        return 1;
    }
    int n = atoi(argv[1]);
    int seed = atoi(argv[2]);

    if (n <= 1 || seed < 0) {
        std::cout << "Uso: " << argv[0] << " <n> <seed> (n > 1 e seed >=0)" << std::endl;
        return 1;
    }

    std::chrono::steady_clock::time_point start_time;
    std::chrono::steady_clock::time_point end_time;

    std::vector<int> arr = create_desc_array(n);
    
    start_time = std::chrono::steady_clock::now();
    merge_sort(arr.begin(), arr.end());
    end_time = std::chrono::steady_clock::now();
    double time_taken_worse = std::chrono::duration_cast<std::chrono::nanoseconds>(end_time - start_time).count() / 1e+9;

    start_time = std::chrono::steady_clock::now();
    merge_sort(arr.begin(), arr.end());
    end_time = std::chrono::steady_clock::now();
    double time_taken_best = std::chrono::duration_cast<std::chrono::nanoseconds>(end_time - start_time).count() / 1e+9;

    fill_random(arr, seed);

    start_time = std::chrono::steady_clock::now();
    merge_sort(arr.begin(), arr.end());
    end_time = std::chrono::steady_clock::now();
    double time_taken_avg = std::chrono::duration_cast<std::chrono::nanoseconds>(end_time - start_time).count() / 1e+9;

    std::cout << std::fixed << std::setprecision(9) <<  time_taken_worse << "," << time_taken_best << "," << time_taken_avg << std::endl;

    return 0;
}
