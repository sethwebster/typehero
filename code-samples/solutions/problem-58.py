def array_difference(first, second):
    second_set = set(second)
    return [item for item in first if item not in second_set]

# Example: array_difference([1, 2, 3, 4], [2, 4]) == [1, 3]
