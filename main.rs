extern crate wasm_example;

use wasm_example::intvect::IntTree;
use wasm_example::intvect::IntVector;

use std::{env, fs};
use std::str::FromStr;

extern crate regex;
use regex::Regex;

extern crate time;
use time::OffsetDateTime;


fn get_vmsize() -> usize {
    let status = fs::read_to_string("/proc/self/status").unwrap();
    let vmsize_re = Regex::new(r"VmSize:\s*([0-9]+) kB").unwrap();
    let vmsize = vmsize_re.captures(&status).unwrap().get(1).unwrap().as_str();
    usize::from_str(vmsize).unwrap()
}

fn get_array_length() -> usize {
    let args: Vec<String> = env::args().collect();
    let number_of_zero: &str = &args[1];
    let number_of_zero = number_of_zero.parse::<u32>().unwrap();

    10_usize.pow(number_of_zero)
}

fn start_measure() -> (usize, time::OffsetDateTime) {
    let a = get_vmsize();
    let b = OffsetDateTime::now_utc();

    (a, b)
}

fn stop_measure(t: (usize, time::OffsetDateTime)) -> (usize, f64) {
    let b = OffsetDateTime::now_utc();
    let a = get_vmsize();

    (a - t.0, (b - t.1).as_seconds_f64())
}


fn measure<F, D, M>(b: &bool,
    name: &str, size: usize, f : F, m : M)
    where F: Fn() -> D,
          M: Fn(&D) -> i64,
          D: wasm_example::intvect::SumComputer
{
    let blou = start_measure();
    let random_values = wasm_example::intvect::RandomValues::new(size);
    let (m_gen, t_gen) = stop_measure(blou);

    let blou = start_measure();
    let mut k = f();
    k.fill_with_v(&random_values);
    let (m_fill, t_fill) = stop_measure(blou);

    let blou = start_measure();
    let value = m(&k);
    let (m_median, t_median) = stop_measure(blou);
    
    if *b {
    println!("RS,{},{},{},{},{},{},{},{},{}", name, value, size, m_gen, m_fill, m_median, t_gen, t_fill, t_median);
    }
}

fn main() {
    let size = get_array_length();

    let n = [false, true];

    for b in n.iter() {
        measure(b, "IntVector-V", size, 
            || IntVector::new(),
            |iv| iv.sum_inf_to_v()
        );

        measure(b, "IntVector-T", size, 
            || IntVector::new(),
            |iv| iv.sum_inf_to_t()
        );

        measure(b, "IntVector-CV", size, 
            || IntVector::new(),
            |iv| {
                let iv = IntVector::copy(iv);
                iv.sum_inf_to_v()
            }
        );

        measure(b, "IntVector-CT", size, 
            || IntVector::new(),
            |iv| {
                let iv = IntTree::using(iv);
                iv.sum_inf_to_t()
            }
        );


        measure(b, "IntTree-V", size, 
            || IntTree::new(),
            |iv| iv.sum_inf_to_v()
        );

        measure(b, "IntTree-T", size, 
            || IntTree::new(),
            |iv| iv.sum_inf_to_t()
        );

        measure(b, "IntTree-CV", size, 
            || IntTree::new(),
            |iv| {
                let iv = IntVector::using(iv);
                iv.sum_inf_to_v()
            }
        );

        measure(b, "IntTree-CT", size, 
            || IntTree::new(),
            |iv| {
                let iv = IntTree::copy(iv);
                iv.sum_inf_to_t()
            }
        );
    }
}
