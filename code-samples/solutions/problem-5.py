def count_chars(s):
    """Count occurrences of each character"""
    counts = {}
    for char in s:
        counts[char] = counts.get(char, 0) + 1
    return counts
