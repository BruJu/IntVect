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


// function randomArray(length) {
//     let a = [];
// 
//     for (let i = 0 ; i != length ; ++i) {
//         a.push(Math.floor(Math.random() * 1000000));
//     }
// 
//     return a;
// }
// 
// const randomlyGeneratedARray = randomArray(array_length);

function start_measure() {
    const mem = get_vmsize();
    const time = performance.now();
    return [mem, time];
}

function stop_measure(start_values) {
    const time = performance.now();
    const mem = get_vmsize();
    return [mem - start_values[0], (time - start_values[1]) / 1000];
}


function benchmarkMedianSearch(pr, n, s, generator, medianSearcher) {
    let gen_tmp = start_measure();
    let random_values = new wasm.RandomValues(s);
    let gen_meas = stop_measure(gen_tmp);

    let fill_tmp = start_measure();
    let dataStructure = generator();

    dataStructure.fill_with_v(random_values);
    let fill_meas = stop_measure(fill_tmp);

    let med_tmp = start_measure();
    let v = medianSearcher(dataStructure);
    let med_meas = stop_measure(med_tmp);

    if (pr)
    console.log(`JS,${n},${v},`+array_length
    + `,${gen_meas[0]},${fill_meas[0]},${med_meas[0]}`
    + `,${gen_meas[1]},${fill_meas[1]},${med_meas[1]}`);
}


let i = 0;

while (i != 2) {

benchmarkMedianSearch(i == 1,
    "IntVector-V", array_length, () => new wasm.IntVector(),
    intVect => intVect.sum_inf_to_v()
);

benchmarkMedianSearch(i == 1,
    "IntVector-T", array_length, () => new wasm.IntVector(),
    intVect => intVect.sum_inf_to_t()
);

benchmarkMedianSearch(i == 1,
    "IntVector-CV", array_length, () => new wasm.IntVector(),
    intVect => {
        let iv = wasm.IntVector.copy(intVect);
        return iv.sum_inf_to_v();
    }
);

benchmarkMedianSearch(i == 1,
    "IntVector-CT", array_length, () => new wasm.IntVector(),
    intVect => {
        let iv = wasm.IntTree.using(intVect);
        return iv.sum_inf_to_t();
    }
);



benchmarkMedianSearch(i == 1,
    "IntTree-V", array_length, () => new wasm.IntTree(),
    intVect => intVect.sum_inf_to_v()
);

benchmarkMedianSearch(i == 1,
    "IntTree-T", array_length, () => new wasm.IntTree(),
    intVect => intVect.sum_inf_to_t()
);

benchmarkMedianSearch(i == 1,
    "IntTree-CV", array_length, () => new wasm.IntTree(),
    intVect => {
        let iv = wasm.IntVector.using(intVect);
        return iv.sum_inf_to_v();
    }
);

benchmarkMedianSearch(i == 1, 
    "IntTree-CT", array_length, () => new wasm.IntTree(),
    intVect => {
        let iv = wasm.IntTree.copy(intVect);
        return iv.sum_inf_to_t();
    }
);

i = i + 1;
}

