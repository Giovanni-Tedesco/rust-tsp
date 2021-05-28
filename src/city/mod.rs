use std::collections::HashMap;
use std::hash::Hash;

use rand::Rng;
use rand::distributions::{Distribution, Uniform};

use crate::cycle::CityMap;

#[derive(Copy, Clone, Debug)]
pub struct City {
    pub number: usize,
    pub x: f64,
    pub y: f64,
    pub next: usize
}

impl City {

    pub fn generate_test(n: usize) -> Vec<Self> {
        let mut rng = rand::thread_rng();

        let mut ret: Vec<City> = Vec::new();

        for i in 0..n {
            let x: f64 = rng.gen();
            let y: f64 = rng.gen();

            let next = if i == n - 1 {
                0
            } else {
                i + 1
            };
            
            let new_city = City {
                x: x * 100f64,
                y: y * 100f64,
                number: i,
                next: next
            };

            ret.push(new_city);
            
        }

        return ret;
    }


    /// Generate a map of n cities.
    pub fn generate_map(n: usize) ->  CityMap {
        todo!();
    }

    pub fn distance(&self, other: &Self) -> f64 {

        f64::sqrt((self.x - other.x).powf(2f64) + (self.y - other.y).powf(2f64))

    }


}