extern crate wasm_example;

use wasm_example::intvect::IntTree;
use wasm_example::intvect::IntVector;
use wasm_example::intvect::SumComputer;

use std::{env, fs};
use std::str::FromStr;

extern crate regex;
use regex::Regex;

extern crate time;
use time::OffsetDateTime;

extern crate rand;
use crate::rand::Rng;


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

fn get_random_vector(array_length: usize) -> Vec<i64> {
    let mut rng = rand::thread_rng();

    let mut numbers = Vec::<i64>::with_capacity(array_length);
    for _ in 0..array_length {
        numbers.push(rng.gen_range(0, 1000000));
    }

    numbers
}

fn measure<F, D, M>(
    name: &str, numbers: &Vec<i64>, f : F, m : M)
    where F: Fn() -> D,
          M: Fn(&D) -> i64,
          D: wasm_example::intvect::SumComputer
{
    let m0 = get_vmsize();
    let t0 = OffsetDateTime::now_utc();

    let mut k = f();
    k.fill_with_vec(numbers);

    let t1 = OffsetDateTime::now_utc();
    let m1 = get_vmsize();
    let t2 = OffsetDateTime::now_utc();

    let value = m(&k);

    let t3 = OffsetDateTime::now_utc();
    let m3 = get_vmsize();

    let time_fill = (t1 - t0).as_seconds_f64();
    let time_median = (t3 - t2).as_seconds_f64();
    let mem_fill = m1 - m0;
    let mem_median = m3 - m1;
    
    println!("RS,{},{},{},{},{},{},{}", name, value, numbers.len(), time_fill, mem_fill, time_median, mem_median);
}

fn main() {
    let size = get_array_length();
    let numbers = get_random_vector(size);

    measure("IntVector-V", &numbers, 
        || IntVector::new(),
        |iv| iv.sum_inf_to_v()
    );

    measure("IntVector-T", &numbers, 
        || IntVector::new(),
        |iv| iv.sum_inf_to_t()
    );

    measure("IntVector-CV", &numbers, 
        || IntVector::new(),
        |iv| {
            let iv = IntVector::copy(iv);
            iv.sum_inf_to_v()
        }
    );

    measure("IntVector-CT", &numbers, 
        || IntVector::new(),
        |iv| {
            let iv = IntTree::using(iv);
            iv.sum_inf_to_t()
        }
    );


    measure("IntTree-V", &numbers, 
        || IntTree::new(),
        |iv| iv.sum_inf_to_v()
    );

    measure("IntTree-T", &numbers, 
        || IntTree::new(),
        |iv| iv.sum_inf_to_t()
    );

    measure("IntTree-CV", &numbers, 
        || IntTree::new(),
        |iv| {
            let iv = IntVector::using(iv);
            iv.sum_inf_to_v()
        }
    );

    measure("IntTree-CT", &numbers, 
        || IntTree::new(),
        |iv| {
            let iv = IntTree::copy(iv);
            iv.sum_inf_to_t()
        }
    );
}
