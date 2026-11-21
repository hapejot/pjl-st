import platform
import statistics
import sys
import time
from functools import reduce


def integer_loop():
    i = 0
    total = 0
    while i < 1000:
        i += 1
        total += i
    return total


def block_calls():
    i = 0
    block = lambda value: value + 1
    while i < 1000:
        i = block(i)
    return i


class BenchmarkCounter:
    def increment(self, value):
        return value + 1


def method_sends():
    i = 0
    counter = BenchmarkCounter()
    while i < 1000:
        i = counter.increment(i)
    return i


def dictionary():
    values = {}
    total = 0
    for i in range(1, 101):
        values[i] = i
    for _ in range(10):
        for i in range(1, 101):
            total += values[i]
    return total


def array_collect_inject():
    values = [None] * 100
    for i in range(1, 101):
        values[i - 1] = i
    collected = list(map(lambda each: each * 2, values))
    return reduce(lambda total, each: total + each, collected, 0)


WORKLOADS = (
    ("integer loop (1000)", integer_loop, 500500),
    ("block calls (1000)", block_calls, 1000),
    ("method sends (1000)", method_sends, 1000),
    ("dictionary (1000 reads)", dictionary, 50500),
    ("array collect/inject (100)", array_collect_inject, 10100),
)


def main():
    if len(sys.argv) != 4:
        raise ValueError("Expected samples, iterations and warmup")
    samples, iterations, warmup = map(int, sys.argv[1:])
    if samples <= 0 or iterations <= 0 or warmup < 0:
        raise ValueError("samples and iterations must be positive; warmup must be non-negative")
    print(f"{platform.python_implementation()} {platform.python_version()} ({sys.executable})", flush=True)
    for name, operation, expected in WORKLOADS:
        def validate(result):
            if result != expected:
                raise ValueError(f"{name}: expected {expected}, got {result}")

        for _ in range(warmup):
            validate(operation())
        times = []
        for _ in range(samples):
            elapsed = 0
            for _ in range(iterations):
                start = time.perf_counter_ns()
                result = operation()
                elapsed += time.perf_counter_ns() - start
                validate(result)
            times.append(elapsed / iterations)
        median = statistics.median(times)
        if median <= 0:
            raise ValueError("Measured duration is zero")
        print(f"  {name:<36} {median / 1e6:>12.6f} ms  {1e9 / median:>12.1f} runs/s", flush=True)


if __name__ == "__main__":
    main()
