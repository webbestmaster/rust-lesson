#[derive(Debug)]
struct City {
    name: String,
    population: i64,
    country: String,
}

fn city_population_descending(city: &City) -> i64 {
    -city.population
}

fn sort_cities_1(cities: &mut Vec<City>) {
    cities.sort_by_key(city_population_descending);
}

fn sort_cities_2(cities: &mut Vec<City>) {
    cities.sort_by_key(|city| {
        -city.population
    });
}

fn main() {
    let city_1 = City {
        name: "City A".to_string(),
        population: 1000,
        country: "Country A".to_string()
    };

    let city_2 = City {
        name: "City B".to_string(),
        population: 2000,
        country: "Country B".to_string()
    };

    let city_3 = City {
        name: "City C".to_string(),
        population: 3000,
        country: "Country C".to_string()
    };

    let mut city_list = vec!(city_2, city_1, city_3);

    println!("{:?}", city_list);

    sort_cities_1(&mut city_list);
    println!("{:?}", city_list);

    sort_cities_2(&mut city_list);
    println!("{:?}", city_list);
}
