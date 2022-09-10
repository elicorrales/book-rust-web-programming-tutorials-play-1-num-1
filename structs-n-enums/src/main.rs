#![allow(unused)]

use std::{collections::hash_map::HashMap, io::SeekFrom};

#[derive(Debug)]
enum FlexibleDataType{
    SomeString(String),
    SomeNumber(isize)
}

#[derive(Debug)]
struct LocationInfo {
    state_name: FlexibleDataType,
    state_code:FlexibleDataType,
    temperature:FlexibleDataType,
    population:FlexibleDataType,
}

struct Locations {
    locations:HashMap<String, LocationInfo>
}

impl Locations {
    fn new() -> Locations {
        Locations{ locations:HashMap::new()}
    }

    fn add(&mut self, city:&str, state_name:&str, state_code:&str, temp:isize, popu:usize) {
        let mut location_info = LocationInfo{
            state_name: FlexibleDataType::SomeString(state_name.to_string()),
            state_code: FlexibleDataType::SomeString(state_code.to_string()),
            temperature: FlexibleDataType::SomeNumber(temp),
            population: FlexibleDataType::SomeNumber(popu as isize)
        };
        self.locations.insert(city.to_string(), location_info);
    }

    fn get_city_info(&self, city:&str) -> Option<&LocationInfo> {
        self.locations.get(city)
    }
}
fn main() {

    let mut my_locations = Locations::new();

    my_locations.add("Miami", "Florida", "FL", 89, 8_234_343);
    my_locations.add("New York City", "New York", "NY", 75, 28_234_111);
    my_locations.add("Los Angeles", "California", "CA", 70, 40_234_111);

    let city = "Atlanta";
    let city_info = my_locations.get_city_info(city);
    match city_info {
        Option::Some(ci) => { println!("{:?}", city_info); }
        _ => { println!("City {} not found.", city) }
    }


    let city = "Miami";
    let city_info = my_locations.get_city_info(city);
    match city_info {
        Option::Some(ci) => { println!("{:?}", city_info); }
        _ => { println!("City {} not found.", city) }
    }


}
