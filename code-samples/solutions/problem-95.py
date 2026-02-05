import asyncio
from typing import AsyncIterator, TypeVar

T = TypeVar('T')

class AsyncIteratorWithCleanup:
    def __init__(self, source: AsyncIterator[T], cleanup_fn):
        self.source = source
        self.cleanup_fn = cleanup_fn
        self.cleaned_up = False

    async def __aiter__(self):
        return self

    async def __anext__(self):
        try:
            return await self.source.__anext__()
        except (StopAsyncIteration, GeneratorExit, asyncio.CancelledError):
            await self.cleanup()
            raise

    async def cleanup(self):
        if not self.cleaned_up:
            self.cleaned_up = True
            if self.cleanup_fn:
                await self.cleanup_fn()

    async def __aexit__(self, exc_type, exc_val, exc_tb):
        await self.cleanup()

# Usage
async def fetch_data():
    try:
        for i in range(100):
            await asyncio.sleep(0.1)
            yield i
    finally:
        print("Iterator cleanup")

async def main():
    async for item in AsyncIteratorWithCleanup(
        fetch_data(),
        cleanup_fn=lambda: print("Explicit cleanup")
    ):
        if item > 5:
            break  # Cleanup still runs
