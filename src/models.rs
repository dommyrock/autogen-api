//Generated file [Do not change]

#[allow(non_snake_case)]
#[derive(Debug,Clone,serde::Deserialize)]
#[generate_controller]
pub struct Location {
    pub id: i32,
    pub state: String,
}

//Generated file [Do not change]

#[allow(non_snake_case)]
#[derive(Debug,Clone,serde::Deserialize)]
#[generate_controller]
pub struct Candidate {
    pub id: i32,
    pub Name: String,
    pub Surname: String,
    pub Email: String,
}

//Generated file [Do not change]

#[allow(non_snake_case)]
#[derive(Debug,Clone,serde::Deserialize)]
#[generate_controller]
pub struct Job {
    pub Id: i32,
    pub StartDate: chrono::NaiveDateTime,
    pub EndDate: chrono::NaiveDateTime,
    pub LocationId: i32,
}

//Generated file [Do not change]

#[allow(non_snake_case)]
#[derive(Debug,Clone,serde::Deserialize)]
#[generate_controller]
pub struct Shifts {
    pub Id: i32,
    pub CandidateId: i32,
    pub StartDate: chrono::NaiveDate,
    pub StartTime: chrono::NaiveTime,
    pub EndDate: chrono::NaiveDate,
    pub EndTime: chrono::NaiveTime,
}

