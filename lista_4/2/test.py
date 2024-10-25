import random
import time
import statistics
import numpy as np


def quickselect_partition(arr, left, right, pivot_index):
    pivot_value = arr[pivot_index]
    arr[pivot_index], arr[right] = arr[right], arr[pivot_index]
    store_index = left

    for i in range(left, right):
        if arr[i] < pivot_value:
            arr[store_index], arr[i] = arr[i], arr[store_index]
            store_index += 1

    arr[right], arr[store_index] = arr[store_index], arr[right]
    return store_index


def randomized_select(arr, left, right, k):
    if left == right:
        return arr[left]

    pivot_index = random.randint(left, right)
    pivot_index = quickselect_partition(arr, left, right, pivot_index)

    if k == pivot_index:
        return arr[k]
    elif k < pivot_index:
        return randomized_select(arr, left, pivot_index - 1, k)
    else:
        return randomized_select(arr, pivot_index + 1, right, k)


def median_of_medians(arr, left, right, k):
    if right - left <= 5:
        arr[left : right + 1] = sorted(arr[left : right + 1])
        return arr[k]

    # Split into groups of 5 and find medians
    groups = []
    for i in range(left, right + 1, 5):
        group = arr[i : min(i + 5, right + 1)]
        median = statistics.median(group)
        groups.append(median)

    # Find median of medians
    pivot_value = statistics.median(groups)

    # Partition around pivot
    pivot_index = left
    for i in range(left, right + 1):
        if arr[i] == pivot_value:
            pivot_index = i
            break

    pivot_index = quickselect_partition(arr, left, right, pivot_index)

    if k == pivot_index:
        return arr[k]
    elif k < pivot_index:
        return median_of_medians(arr, left, pivot_index - 1, k)
    else:
        return median_of_medians(arr, pivot_index + 1, right, k)


# Benchmark function
def benchmark_selection(sizes=[1000, 10000, 100000, 1000000]):
    results = []

    for size in sizes:
        # Generate data
        data = list(range(size))
        random.shuffle(data)
        k = size // 2  # Find median

        # Test randomized select
        arr1 = data.copy()
        start = time.time()
        rand_result = randomized_select(arr1, 0, len(arr1) - 1, k)
        rand_time = time.time() - start

        # Test median of medians
        arr2 = data.copy()
        start = time.time()
        mom_result = median_of_medians(arr2, 0, len(arr2) - 1, k)
        mom_time = time.time() - start

        results.append(
            {"size": size, "randomized_time": rand_time, "mom_time": mom_time}
        )

    return results


# Run benchmark
results = benchmark_selection()
print("\nBenchmark Results:")
print("Array Size | Randomized Select (s) | Median of Medians (s)")
print("-" * 55)
for r in results:
    print(f"{r['size']:^10} | {r['randomized_time']:^18.4f} | {r['mom_time']:^18.4f}")
