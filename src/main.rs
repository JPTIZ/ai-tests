extern crate exitfailure;
extern crate failure;

#[macro_use]
extern crate serde_derive;
extern crate serde_json;

#[macro_use]
extern crate structopt;

use exitfailure::ExitFailure;
use std::path::PathBuf;
use structopt::StructOpt;

mod cities;

#[derive(StructOpt, Debug)]
#[structopt(name = "wee", about = "Heuristics tests for paths through cities.")]
struct Args {
    #[structopt(parse(from_os_str))]
    filename: PathBuf,
}

fn main() -> Result<(), ExitFailure> {
    let args = Args::from_args();

    let contents = cities::from_file(
        &args
            .filename
            .into_os_string()
            .into_string()
            .expect("Valid filename"),
    )?;
    println!("File contents:\n{:?}", contents);

    Ok(())
}
