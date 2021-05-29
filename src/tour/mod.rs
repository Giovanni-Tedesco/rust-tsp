pub mod fitness;
pub mod utils;

use crate::city::City;
use genetic::AlgorithmParams;
use rand::Rng;
use rand::distributions::Uniform;
use rand::prelude::Distribution;

use crate::cycle::Cycle;

use genetic::GeneticCustom;


#[derive(Debug)]
pub struct Tour {
    pub start: City,
    pub tour: Vec<City>
}

impl Tour {
    fn new(cities: Vec<i8>) -> Self {
        todo!();
    }

    /// Regular cross over.
    fn cross_over(&self, other: &Self, co_factor: f64) -> (Self, Self) {
        let mut p_1 = self.gene().clone();
        let mut p_2 = other.gene().clone();

        let mut rng = rand::thread_rng();

        for i in 1..p_1.cycle_vec.len() {
            let x: f64 = rng.gen();

            if x > co_factor {
                let temp = p_1.cycle_vec[i];
                p_1.cycle_vec[i] = p_2.cycle_vec[i];
                p_2.cycle_vec[i] = temp;
            }
        }
        
        return (Tour::from_gene(&p_1), Tour::from_gene(&p_2));
        
    }

    /// Swap genomes at a given half point.
    fn splice() -> (Self, Self) {
        todo!();
    }

    /// Pick two cities at random and swap them.
    fn mutate(&self, mutation_rate: f64) -> Self {
        let mut rng = rand::thread_rng();

        let mut p_1 = self.gene().clone();

        // TODO: Update this to check that they are not the same size...
        // TODO: Maybe use the mutation rate somehow to assist in picking things to change.
        let uniform_dist = Uniform::from(1..p_1.cycle_vec.len());
        let idx_1 = uniform_dist.sample(&mut rng);
        let idx_2 = uniform_dist.sample(&mut rng);

        p_1.cycle_vec.swap(idx_1, idx_2);

        return Tour::from_gene(&p_1);

    }

}

impl GeneticCustom<Cycle> for Tour {
    fn gene(&self) -> Cycle {
        Cycle::from_cities(&self.tour)
    }

    fn from_gene(chromosome: &Cycle) -> Self {
        let cities = chromosome.to_vec();
        let start = cities[0];

        return Tour {
            start: start,
            tour: cities
        }
       
    }

    fn generate_random() -> Self {
        todo!();
    }

    fn mutate(&self, other: &Self, params: &AlgorithmParams) -> (Self, Self) {
        let (child_1, child_2) = self.cross_over(other, params.co_factor);

        let mutated_child_1 = child_1.mutate(params.mutation_rate);
        let mutated_child_2 = child_2.mutate(params.mutation_rate);

        return (mutated_child_1, mutated_child_2);
    }
}

