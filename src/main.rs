use std::error::Error;
use std::io;
use std::process;
use std::collections::HashSet;

use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[allow(non_snake_case)]
struct CourseRequirements {
    CRS_CODE: String,
    CRS_STARTTERM: String,
    CRS_PREREQ: String,
    CRS_COREQ: String,
    CRS_PREREQNOTE: String,
    CRS_COREQNOTE: String,
    ADMIN_CAMPUS_CD: String
}

fn main() {
    if let Err(err) = run() {
        println!("{}", err);
        process::exit(1);
    }
}

fn run() -> Result<(), Box<dyn Error>> {
    let mut rdr = csv::Reader::from_reader(io::stdin());
    let mut vec: Vec<CourseRequirements> = Vec::new();

    for result in rdr.deserialize() {
        let record: CourseRequirements = result?;
        vec.push(record);
    }

    // println!("Original length: {}", vec.len());

    let has_prereqs = filter_prereqs(vec);

    // println!("New length: {}", has_prereqs.len());

    let uniq = filter_uniq(has_prereqs);

    for record in uniq.iter() {
        println!("{}", record);
    }

    Ok(())
}

fn filter_prereqs(vec: Vec<CourseRequirements>) -> Vec<CourseRequirements> {
    vec
        .into_iter()
        .filter(|course| course.CRS_PREREQ != " ")
        .collect()
}

fn filter_uniq(vec: Vec<CourseRequirements>) -> Vec<String> {
    vec.into_iter()
        .map(|course| course.CRS_PREREQ)
        .collect::<HashSet<_>>()
        .into_iter()
        .collect()
}