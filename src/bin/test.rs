use genetic::GeneticCustom;
use travelling_salesman::*;

use travelling_salesman::city::City;
use travelling_salesman::tour::fitness::fitness;

fn main() { 

    println!("Hello world!");

    let test = City::generate_test(10);

    // print!("{:?}", test);

    let t = tour::Tour{
        start: test[0],
        tour: test,
    };

    let mut gene = t.gene();
    gene.cycle_vec = vec![6, 2, 3, 4, 1, 5, 7, 8, 9, 10];

    println!("{:?}", gene);
    let x = tour::Tour::from_gene(&gene);

    println!("{:?}", fitness(&x));

    let c1 = City {
        x: -7.0,
        y: -4.0,
        number: 1,
        next: 2,
    };

    let c2 = City {
        x: 17f64,
        y: 6.5,
        number: 1,
        next: 2,
    };

}