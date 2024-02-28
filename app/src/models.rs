//Generated file [Do not change]

use autogen_macros::generate_controller;
use serde::{Serialize,Deserialize};

#[allow(non_snake_case)]
#[derive(Debug,Clone,Serialize,Deserialize)]
pub struct Location {
    pub id: i32,
    pub state: String,
}

#[allow(non_snake_case)]
#[derive(Debug,Clone,Serialize,Deserialize)]
pub struct Candidate {
    pub id: i32,
    pub Name: String,
    pub Surname: String,
    pub Email: String,
}

#[allow(non_snake_case)]
#[derive(Debug,Clone,Serialize,Deserialize)]
pub struct Job {
    pub Id: i32,
    pub StartDate: chrono::NaiveDateTime,
    pub EndDate: chrono::NaiveDateTime,
    pub LocationId: i32,
}

#[allow(non_snake_case)]
#[derive(Debug,Clone,Serialize,Deserialize)]
pub struct Shifts {
    pub Id: i32,
    pub CandidateId: i32,
    pub StartDate: chrono::NaiveDate,
    pub StartTime: chrono::NaiveTime,
    pub EndDate: chrono::NaiveDate,
    pub EndTime: chrono::NaiveTime,
}

