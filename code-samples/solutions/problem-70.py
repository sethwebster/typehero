def rotate_array(arr, k):
    if not arr:
        return arr
    k = k % len(arr)
    return arr[-k:] + arr[:-k]
