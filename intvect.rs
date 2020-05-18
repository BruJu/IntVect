
extern crate wasm_bindgen;
extern crate js_sys;

use wasm_bindgen::prelude::*;

use std::collections::BTreeMap;

extern crate rand;
use rand::Rng;


/*
A JsValue doesn't actually live in Rust right now but actually in a
table owned by the wasm-bindgen generated JS glue code. Eventually
the ownership will transfer into wasm directly and this will likely
become more efficient, but for now it may be slightly slow.
*/

#[wasm_bindgen]
pub struct RandomValues { values: Vec<i64> }

#[wasm_bindgen]
pub struct IntVector { values: Vec<i64> }

#[wasm_bindgen]
pub struct IntTree { values: BTreeMap<i64, i64> }

pub trait SumComputer {
    fn sum_inf_to_vector(&self, value: i64) -> i64;
    fn sum_inf_to_tree(&self, value: i64) -> i64;
    fn fill_with_v(&mut self, random_values: &RandomValues);
}

#[wasm_bindgen]
impl IntVector {
    #[wasm_bindgen(constructor)]
    pub fn new() -> IntVector {
        IntVector { values: vec!() }
    }

    pub fn using(tree: &IntTree) -> IntVector {
        let mut v = vec!();

        for (&value, &occ) in tree.values.iter() {
            for _ in 0..occ {
                v.push(value);
            }
        }

        IntVector { values: v }
    }

    pub fn fill_with_v(&mut self, random_values: &RandomValues) {
        for x in random_values.values.iter() {
            self.values.push(*x);
        }

        self.values.sort();
    }

    pub fn copy(other: &IntVector) -> IntVector {
        let mut v = vec!();
        for va in other.values.iter() {
            v.push(*va);
        }
        IntVector { values: v }
    }

//    pub fn fill(&mut self, passed_values: &js_sys::Array) {
//        for x in passed_values.keys() {
//            let x = x.unwrap().as_f64().unwrap() as i64;
//            self.values.push(x);
//        }
//
//        self.values.sort();
//    }

    pub fn sum_inf_to_v(&self) -> i64 {
        self.sum_inf_to_vector(100000)
    }

    pub fn sum_inf_to_t(&self) -> i64 {
        self.sum_inf_to_tree(100000)
    }
}

impl SumComputer for IntVector {
    fn sum_inf_to_vector(&self, value: i64) -> i64 {
        let mut s:i64 = 0;

        for x in self.values.iter() {
            if x >= &value {
                break;
            }

            s += x;
        }

        s
    }

    fn sum_inf_to_tree(&self, value: i64) -> i64 {
        let tree = IntTree::using(self);
        tree.sum_inf_to_tree(value)
    }

    fn fill_with_v(&mut self, random_values: &RandomValues) {
        IntVector::fill_with_v(self, random_values);
    }
}

impl SumComputer for IntTree {
    fn sum_inf_to_vector(&self, value: i64) -> i64 {
        let vect = IntVector::using(self);
        vect.sum_inf_to_vector(value)
    }

    fn sum_inf_to_tree(&self, value: i64) -> i64 {
        let mut s = 0;

        for (&v, &occurrences) in self.values.range(..value) {
            s += v * occurrences;
        }

        s
    }

    fn fill_with_v(&mut self, random_values: &RandomValues) {
        IntTree::fill_with_v(self, random_values);
    }
}

#[wasm_bindgen]
impl IntTree {
    #[wasm_bindgen(constructor)]
    pub fn new() -> IntTree {
        IntTree { values: BTreeMap::new() }
    }

    pub fn using(vect: &IntVector) -> IntTree {
        let mut t = IntTree::new();
        
        for x in vect.values.iter() {
            *t.values.entry(*x).or_insert(0) += 1;
        }

        t
    }

    pub fn copy(other: &IntTree) -> IntTree {
        let mut tree = BTreeMap::new();
        for (&v, &o) in other.values.iter() {
            tree.insert(v, o);
        }
        IntTree { values: tree }
    }

//    pub fn fill(&mut self, passed_values: &js_sys::Array) {
//        for x in passed_values.keys() {
//            let x = x.unwrap().as_f64().unwrap() as i64;
//            *self.values.entry(x).or_insert(0) += 1;
//        }
//    }

    pub fn sum_inf_to_v(&self) -> i64 {
        self.sum_inf_to_vector(100000)
    }

    pub fn sum_inf_to_t(&self) -> i64 {
        self.sum_inf_to_tree(100000)
    }

    pub fn fill_with_v(&mut self, random_values: &RandomValues) {
        for x in random_values.values.iter() {
            *self.values.entry(*x).or_insert(0) += 1;
        }
    }
}





#[wasm_bindgen]
impl RandomValues {
    #[wasm_bindgen(constructor)]
    pub fn new(array_length: usize) -> RandomValues {
        let mut rng = rand::thread_rng();

        let mut numbers = Vec::<i64>::with_capacity(array_length);
        for _ in 0..array_length {
            numbers.push(rng.gen_range(0, 1000000));
        }
    
        RandomValues { values: numbers }
    }
}