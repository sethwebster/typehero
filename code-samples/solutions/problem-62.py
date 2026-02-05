def first_duplicate(arr):
    seen = set()
    for item in arr:
        if item in seen:
            return item
        seen.add(item)
    return None
