def find_index(arr, predicate):
    for i, item in enumerate(arr):
        if predicate(item):
            return i
    return -1

# Usage:
# find_index([1, 2, 3, 4], lambda x: x > 2) -> 2
