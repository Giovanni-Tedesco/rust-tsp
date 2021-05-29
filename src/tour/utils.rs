use std::collections::HashMap;

use rand::Rng;

use crate::city::City;

pub fn generate_random_permutation(n: usize) -> Vec<usize> {
    let mut v: Vec<f64> = Vec::new(); 
    let mut rng = rand::thread_rng();

    for _ in 0..n {
        let x: f64 = rng.gen();
        v.push(x);
    }

    let mut indicies: Vec<usize> = (0..n).collect();
    indicies.sort_by(|&a, &b| v[a].partial_cmp(&v[b]).unwrap());
    
    return indicies;
}


pub fn generate_map(order: Vec<usize>) -> HashMap<usize, City> {
    todo!();
}