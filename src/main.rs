#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate exitfailure;
extern crate failure;

use exitfailure::ExitFailure;
use failure::ResultExt;
use std::{collections::HashMap, env, fs};

#[derive(Deserialize, Debug)]
struct City {
    location: (f64, f64),
    neighbours: Vec<String>,
}

#[derive(Deserialize, Debug)]
struct Data(HashMap<String, City>);

fn load_cities(path: &str) -> Result<Data, failure::Error> {
    let contents = fs::read_to_string(path).context(path.to_string())?;

    let cities = serde_json::from_str(&contents)?;

    Ok(cities)
}

fn main() -> Result<(), ExitFailure> {
    let path = env::args().nth(1).unwrap();

    let contents = load_cities(&path)?;
    println!("File contents:\n{:?}", contents);

    Ok(())
}
