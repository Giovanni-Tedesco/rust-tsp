use super::Tour;

/// TODO: Implement a cache system for this.
pub fn fitness(t: &Tour) -> f64 {
    // Calculate the total length of the tour...
    let mut total = 0f64;

    for (i, city) in t.tour.iter().enumerate() {
        let next_city = match t.tour.get(i + 1) {
            Some(next_city) => {
                next_city
            }
            None => {
                *t.tour.get(0).as_ref().unwrap()
            }
        };

        total += city.distance(next_city);
    }

    return 1f64/total;
}