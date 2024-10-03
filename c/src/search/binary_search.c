#include "search.h"

SearchResult binary_search(int* arr, unsigned int start, unsigned int end, int x) {
    SearchResult r;
    if (start <= end) {    
        int mid = (start + end) / 2;
        if (arr[mid] == x){
            r.index = mid;
            r.found = 1;
            return r;
        }
        if (x < arr[mid]) {
            return binary_search(arr, start, mid - 1, x);
        } else {
            return binary_search(arr, mid + 1, end, x);
        }
	} else {
        r.found = 0;
		return r;
	}
}