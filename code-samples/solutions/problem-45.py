def memoize(fn):
    cache = {}
    def wrapper(*args):
        key = args
        if key not in cache:
            cache[key] = fn(*args)
        return cache[key]
    return wrapper
