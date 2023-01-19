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

    pub fn group_by_semester(&self) -> (GradeList, GradeList, GradeList) {
        let mut first_semester = GradeList::default();
        let mut second_semester = GradeList::default();
        let mut third_semester = GradeList::default();

        for grade in &self.0 {
            match grade.semester.as_str() {
                "1/8" => first_semester.0.push(grade.clone()),
                "2/8" => second_semester.0.push(grade.clone()),
                "1/9" => third_semester.0.push(grade.clone()),
                _ => panic!("Invalid semester"),
            }
        }

        (first_semester, second_semester, third_semester)
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