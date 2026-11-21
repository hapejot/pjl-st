"use strict";

function integerLoop() {
    let i = 0;
    let sum = 0;
    while (i < 1000) {
        i += 1;
        sum += i;
    }
    return sum;
}

function blockCalls() {
    let i = 0;
    const block = value => value + 1;
    while (i < 1000) {
        i = block(i);
    }
    return i;
}

class BenchmarkCounter {
    increment(value) {
        return value + 1;
    }
}

function methodSends() {
    let i = 0;
    const counter = new BenchmarkCounter();
    while (i < 1000) {
        i = counter.increment(i);
    }
    return i;
}

function dictionary() {
    const values = new Map();
    let sum = 0;
    for (let i = 1; i <= 100; i++) {
        values.set(i, i);
    }
    for (let n = 0; n < 10; n++) {
        for (let i = 1; i <= 100; i++) {
            sum += values.get(i);
        }
    }
    return sum;
}

function arrayCollectInject() {
    const values = new Array(100);
    for (let i = 1; i <= 100; i++) {
        values[i - 1] = i;
    }
    return values.map(each => each * 2).reduce((sum, each) => sum + each, 0);
}

const workloads = [
    ["integer loop (1000)", integerLoop, 500500],
    ["block calls (1000)", blockCalls, 1000],
    ["method sends (1000)", methodSends, 1000],
    ["dictionary (1000 reads)", dictionary, 50500],
    ["array collect/inject (100)", arrayCollectInject, 10100],
];

function main() {
    const args = process.argv.slice(2);
    if (args.length !== 3 || args.some(arg => !/^\d+$/.test(arg))) {
        throw new Error("Expected samples, iterations and warmup as non-negative integers");
    }
    const [samples, iterations, warmup] = args.map(Number);
    if (!args.every(arg => Number.isSafeInteger(Number(arg))) || samples === 0 || iterations === 0) {
        throw new Error("samples and iterations must be positive safe integers");
    }
    console.log(`Node.js ${process.version} (V8 ${process.versions.v8}; ${process.execPath})`);
    for (const [name, operation, expected] of workloads) {
        function validate(result) {
            if (result !== expected) {
                throw new Error(`${name}: expected ${expected}, got ${result}`);
            }
        }
        for (let i = 0; i < warmup; i++) {
            validate(operation());
        }
        const times = [];
        for (let sample = 0; sample < samples; sample++) {
            let elapsed = 0;
            for (let i = 0; i < iterations; i++) {
                const start = process.hrtime.bigint();
                const result = operation();
                elapsed += Number(process.hrtime.bigint() - start);
                validate(result);
            }
            times.push(elapsed / iterations);
        }
        times.sort((a, b) => a - b);
        const midpoint = Math.floor(times.length / 2);
        const median = times.length % 2 ? times[midpoint] : (times[midpoint - 1] + times[midpoint]) / 2;
        if (median <= 0) {
            throw new Error("Measured duration is zero");
        }
        console.log(`  ${name.padEnd(36)} ${(median / 1e6).toFixed(6).padStart(12)} ms  ${(1e9 / median).toFixed(1).padStart(12)} runs/s`);
    }
}

main();
