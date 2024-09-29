#include <iostream>
#include <algorithm>
#include <vector>

using Iterator = std::vector<int>::iterator;

void insertion_sort(Iterator begin, Iterator end) {
    for (Iterator it = begin + 1; it < end; ++it) {
        int key = *it;
        Iterator jt = it;
        while (jt > begin && *(jt-1) > key) {
            *jt = *(jt-1);
            --jt;
        }
        *jt = key;
    }
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

    std::vector<int> arr = std::vector<int>(n); 
    std::srand(seed);
    std::generate(arr.begin(), arr.end(), std::rand);
    
    std::chrono::steady_clock::time_point start_time = std::chrono::steady_clock::now();
    insertion_sort(arr.begin(), arr.end());
    std::chrono::steady_clock::time_point end_time = std::chrono::steady_clock::now();
    double time_taken_avg = std::chrono::duration_cast<std::chrono::nanoseconds>(end_time - start_time).count() / 1e+9;

    std::cout << std::fixed << std::setprecision(9) << time_taken_worse << "," << time_taken_best << "," << time_taken_avg << std::endl;

    return 0;
}
