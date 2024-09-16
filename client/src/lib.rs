use serde::{Serialize, Deserialize};
use std::time::Duration;

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Edit {
    pub position: usize,
    pub insert: Option<String>,
    pub delete: Option<usize>,
}

pub fn parse_user_input(input: &str) -> Result<Edit, String> {
    // Function body...
    let parts: Vec<&str> = input.trim().split(',').collect();
    if parts.len() != 2 {
        return Err("Invalid input format. Use 'position,insert' or 'position,delete'".to_string());
    }

    let position: usize = parts[0]
        .parse()
        .map_err(|_| "Invalid position. Please enter a number.".to_string())?;

    let (insert, delete) = if parts[1].starts_with("delete") {
        let delete_count: usize = parts[1][6..]
            .trim()
            .parse()
            .map_err(|_| "Invalid delete count. Please enter a number.".to_string())?;
        (None, Some(delete_count))
    } else {
        (Some(parts[1].to_string()), None)
    };

    Ok(Edit {
        position,
        insert,
        delete,
    })
}

pub fn serialize_edit(edit: &Edit) -> Result<String, serde_json::Error> {
    serde_json::to_string(edit)
}

pub fn deserialize_edit(json_str: &str) -> Result<Edit, serde_json::Error> {
    serde_json::from_str(json_str)
}

pub fn calculate_retry_delay(retry_count: u32) -> Duration {
    Duration::from_secs(2u64.pow(retry_count))
}
