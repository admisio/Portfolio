use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Grade {
    subject: String,
    semester: String,
    value: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct GradeList(Vec<Grade>);

impl GradeList {
    pub fn from_opt_str(grades: Option<String>) -> Option<Self> {
        grades.map(
            |grades| serde_json::from_str(&grades).unwrap() // TODO: handle error
        )
    }
}

impl Default for GradeList {
    fn default() -> Self {
        Self(vec![])
    }
}

impl From<Vec<Grade>> for GradeList {
    fn from(grades: Vec<Grade>) -> Self {
        Self(grades)
    }
}

impl ToString for GradeList {
    fn to_string(&self) -> String {
        serde_json::to_string(&self.0).unwrap()
    }
}