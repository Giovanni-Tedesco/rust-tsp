use std::collections::HashMap;

use crate::city::{City};

pub type CityMap = HashMap<usize, City>;


#[derive(Debug, Clone)]
pub struct Cycle {
    pub cycle_vec: Vec<usize>,
    town_map: CityMap
}

impl Cycle {

    pub fn from(order: Vec<usize>, city_map: CityMap) -> Self {
        Cycle {
            cycle_vec: order,
            town_map: city_map
        }
    }

    pub fn from_cities(cities: &Vec<City>) -> Cycle {
        let mut town_map: HashMap<usize, City> = HashMap::new();
        let mut cycle: Vec<usize> = Vec::new();

        cycle.push(cities[0].number);

        for city in cities {
            cycle.push(city.next);
            town_map.insert(city.number, *city);
        }

        return Cycle {
            town_map: town_map,
            cycle_vec: cycle
        }

    }

    // pub fn swap(&mut self, other: &mut Self, idx: usize) -> (Self, Self) {

    //     let temp = self.cycle_vec[idx];
    //     self.cycle_vec[idx] = other.cycle_vec[idx];
    //     other.cycle_vec[idx] = temp;



    // }

    pub fn to_vec(&self) -> Vec<City> {
        let mut ret = Vec::new();
        
        for (i, item) in self.cycle_vec.iter().enumerate() {
            let mut city = match self.town_map.get(item) {
                Some(city) => {
                    *city
                }
                None => {
                    let v = self.cycle_vec.get(0).unwrap();
                    *self.town_map.get(v).unwrap()
                }
            };

            let next = match self.cycle_vec.get(i + 1) {
                Some(item) => {
                    *item
                }
                None => {
                    *self.cycle_vec.get(0).unwrap()
                }
            };

            city.next = next;

            ret.push(city);

        }
        return ret;

    }

}