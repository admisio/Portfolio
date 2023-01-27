use serde::{Serialize, Deserialize};
use validator::{Validate, ValidationError};

use crate::error::ServiceError;


fn validate_semester(semester: &str) -> Result<(), ValidationError> {
    match semester {
        "1/8" | "2/8" | "1/9" | "2/9" => Ok(()),
        _ => Err(ValidationError::new("Invalid semester"))
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate, PartialEq, Eq)]
pub struct Grade {
    #[validate(length(min = 1, max = 255))]
    subject: String,
    #[validate(length(min = 1, max = 255), custom = "validate_semester")]
    semester: String,
    #[validate(range(min = 1, max = 5))]
    value: i32,
}


impl Grade {
    pub fn validate_self(&self) -> Result<(), ServiceError> {
        self.validate()
            .map_err(ServiceError::ValidationError)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct GradeList(Vec<Grade>);

impl GradeList {
    pub fn validate_self(&self) -> Result<(), ServiceError> {
        self.0.iter()
            .map(|grade| grade.validate_self())
            .collect::<Result<Vec<_>, _>>()
            .map(|_| ())
    }

    pub fn from_opt_str(grades: Option<String>) -> Option<Self> {
        grades.map(
            |grades| serde_json::from_str(&grades).unwrap() // TODO: handle error
        )
    }

    pub fn group_by_semester(&self) -> Result<(GradeList, GradeList, GradeList, GradeList), ServiceError> {
        let mut first_semester = GradeList::default();
        let mut second_semester = GradeList::default();
        let mut third_semester = GradeList::default();
        let mut fourth_semester = GradeList::default();

        for grade in &self.0 {
            match grade.semester.as_str() {
                "1/8" => first_semester.0.push(grade.clone()),
                "2/8" => second_semester.0.push(grade.clone()),
                "1/9" => third_semester.0.push(grade.clone()),
                "2/9" => fourth_semester.0.push(grade.clone()),
                _ => return Err(ServiceError::FormatError),
            }
        }

        Ok(
            (first_semester, second_semester, third_semester, fourth_semester)
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