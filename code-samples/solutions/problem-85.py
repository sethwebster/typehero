class LazySeq:
    def __init__(self, iterable):
        self._iterable = iter(iterable)

    def map(self, fn):
        def generator():
            for item in self._iterable:
                yield fn(item)
        return LazySeq(generator())

    def filter(self, predicate):
        def generator():
            for item in self._iterable:
                if predicate(item):
                    yield item
        return LazySeq(generator())

    def take(self, n):
        def generator():
            for i, item in enumerate(self._iterable):
                if i >= n:
                    break
                yield item
        return LazySeq(generator())

    def collect(self):
        return list(self._iterable)

# Usage:
# seq = LazySeq(range(1000000))
# result = seq.map(lambda x: x * 2).filter(lambda x: x % 3 == 0).take(5).collect()
# # Only evaluates first 5 matching elements, not entire range
