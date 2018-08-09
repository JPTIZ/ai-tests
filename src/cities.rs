use failure;
use failure::ResultExt;
use serde_json;
use std::collections::HashMap;
use std::fs;

#[derive(Deserialize, Debug)]
struct City {
    location: (f64, f64),
    neighbours: Vec<String>,
}

#[derive(Deserialize, Debug)]
pub struct Data(HashMap<String, City>);

pub fn from_file(path: &str) -> Result<Data, failure::Error> {
    let contents = fs::read_to_string(path).context(path.to_string())?;

    let cities = serde_json::from_str(&contents)?;

    Ok(cities)
}
