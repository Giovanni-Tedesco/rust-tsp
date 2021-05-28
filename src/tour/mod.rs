pub mod fitness;

use crate::city::City;
use rand::Rng;

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
        let p_1 = self.gene();
        let p_2 = other.gene();
        
        todo!();

        

    }

    /// Swap genomes at a given half point.
    fn splice() -> (Self, Self) {
        todo!();
    }

    /// Pick two cities at random and swap them.
    fn mutate() -> Self {
        todo!();
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
        todo!()
    }

    fn mutate(&self, other: &Self) -> Self {
        todo!()
    }
}

