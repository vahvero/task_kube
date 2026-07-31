#[derive(PartialEq)]
pub enum TaskState {
    Completed,
    InProgress,
    Pending,
}

impl PartialEq<String> for TaskState {
    fn eq(&self, other: &String) -> bool {
        other == &self.to_string()
    }
}

impl TryFrom<&str> for TaskState {
    type Error = String;
    fn try_from(value: &str) -> Result<TaskState, String> {
        match value.to_uppercase().as_str() {
            "COMPLETED" => Ok(TaskState::Completed),
            "IN-PROGRESS" => Ok(TaskState::InProgress),
            "PENDING" => Ok(TaskState::Pending),
            _ => Err(format!("Invalid TaskState: {value}")),
        }
    }
}

impl ToString for TaskState {
    fn to_string(&self) -> String {
        match self {
            Self::Completed => "COMPLETED".to_owned(),
            Self::Pending => "PENDING".to_owned(),
            Self::InProgress => "IN-PROGRESS".to_owned(),
        }
    }
}
