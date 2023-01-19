use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct School {
    name: String,
    field: String,
}

impl School {
    pub fn from_opt_str(school: Option<String>) -> Option<Self> {
        println!("School: {:?}", school);
        school.map(
            |school| serde_json::from_str(&school).unwrap() // TODO: handle error
        )
    }
}

impl ToString for School {
    fn to_string(&self) -> String {
        serde_json::to_string(&self).unwrap()
    }
}

impl Default for School {
    fn default() -> Self {
        Self {
            name: String::default(),
            field: String::default(),
        }
    }
}