#!/usr/bin/env node

// Usage : ./intvect.js 7

const fs = require("fs");
const { performance } = require("perf_hooks");

function get_vmsize() {
    txt = fs.readFileSync("/proc/self/status", encoding="utf8");
    val = txt.match(/VmSize:\W*([0-9]+) kB/).pop();
    return Number.parseInt(val);
}

const wasm = require('../../Portable-Reasoning-in-Web-Assembly/wasm_example/wasm_example.js');


const array_length = (() => {
    let number_of_zero = parseInt(process.argv[2])
    return Math.pow(10, number_of_zero);
})();


function randomArray(length) {
    let a = [];

    for (let i = 0 ; i != length ; ++i) {
        a.push(Math.floor(Math.random() * 1000000));
    }

    return a;
}

const randomlyGeneratedARray = randomArray(array_length);

function benchmarkMedianSearch(n, s, generator, medianSearcher) {
    const memo_start = get_vmsize();
    const time_start = performance.now();
    let dataStructure = generator();
    dataStructure.fill(randomlyGeneratedARray);

    const time_fill = performance.now();
    const memo_fill = get_vmsize();

    let v = medianSearcher(dataStructure);

    const time_median = performance.now();
    const memo_median = get_vmsize();

    let res = [
        (time_fill - time_start) / 1000,
        memo_fill - memo_start,
        (time_median - time_fill) / 1000,
        memo_median - memo_fill
    ];

    console.log(`JS,${n},${v},`+array_length+`,${res[0]},${res[1]},${res[2]},${res[3]}`);
}


benchmarkMedianSearch(
    "IntVector-V", array_length, () => new wasm.IntVector(),
    intVect => intVect.sum_inf_to_v()
);

benchmarkMedianSearch(
    "IntVector-T", array_length, () => new wasm.IntVector(),
    intVect => intVect.sum_inf_to_t()
);

benchmarkMedianSearch(
    "IntVector-CV", array_length, () => new wasm.IntVector(),
    intVect => {
        let iv = wasm.IntVector.copy(intVect);
        return iv.sum_inf_to_v();
    }
);

benchmarkMedianSearch(
    "IntVector-CT", array_length, () => new wasm.IntVector(),
    intVect => {
        let iv = wasm.IntTree.using(intVect);
        return iv.sum_inf_to_t();
    }
);



benchmarkMedianSearch(
    "IntTree-V", array_length, () => new wasm.IntTree(),
    intVect => intVect.sum_inf_to_v()
);

benchmarkMedianSearch(
    "IntTree-T", array_length, () => new wasm.IntTree(),
    intVect => intVect.sum_inf_to_t()
);

benchmarkMedianSearch(
    "IntTree-CV", array_length, () => new wasm.IntTree(),
    intVect => {
        let iv = wasm.IntVector.using(intVect);
        return iv.sum_inf_to_v();
    }
);

benchmarkMedianSearch(
    "IntTree-CT", array_length, () => new wasm.IntTree(),
    intVect => {
        let iv = wasm.IntTree.copy(intVect);
        return iv.sum_inf_to_t();
    }
);
