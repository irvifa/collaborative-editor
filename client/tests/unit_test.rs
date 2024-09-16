use collaborative_editor_client::{
    calculate_retry_delay, deserialize_edit, parse_user_input, serialize_edit, Edit,
};
use std::time::Duration;

#[test]
fn test_parse_user_input_insert_success() {
    let input = "5,hello";
    let expected = Edit {
        position: 5,
        insert: Some("hello".to_string()),
        delete: None,
    };
    let result = parse_user_input(input).unwrap();
    assert_eq!(result, expected);
}

#[test]
fn test_parse_user_input_delete_success() {
    let input = "3,delete2";
    let expected = Edit {
        position: 3,
        insert: None,
        delete: Some(2),
    };
    let result = parse_user_input(input).unwrap();
    assert_eq!(result, expected);
}

#[test]
fn test_parse_user_input_invalid_format() {
    let input = "invalid input";
    let result = parse_user_input(input);
    assert!(result.is_err());
    assert_eq!(
        result.err().unwrap(),
        "Invalid input format. Use 'position,insert' or 'position,delete'"
    );
}

#[test]
fn test_parse_user_input_invalid_position() {
    let input = "abc,hello";
    let result = parse_user_input(input);
    assert!(result.is_err());
    assert_eq!(
        result.err().unwrap(),
        "Invalid position. Please enter a number."
    );
}

#[test]
fn test_parse_user_input_invalid_delete_count() {
    let input = "5,deleteabc";
    let result = parse_user_input(input);
    assert!(result.is_err());
    assert_eq!(
        result.err().unwrap(),
        "Invalid delete count. Please enter a number."
    );
}

#[test]
fn test_serialize_edit_success() {
    let edit = Edit {
        position: 5,
        insert: Some("hello".to_string()),
        delete: None,
    };
    let json = serialize_edit(&edit).unwrap();
    let expected_json = r#"{"position":5,"insert":"hello","delete":null}"#;
    assert_eq!(json, expected_json);
}

#[test]
fn test_deserialize_edit_success() {
    let json_str = r#"{"position":5,"insert":"hello","delete":null}"#;
    let expected = Edit {
        position: 5,
        insert: Some("hello".to_string()),
        delete: None,
    };
    let result = deserialize_edit(json_str).unwrap();
    assert_eq!(result, expected);
}

#[test]
fn test_deserialize_edit_invalid_json() {
    let json_str = r#"{"position":5,"insert":"hello""#; // Missing closing brace
    let result = deserialize_edit(json_str);
    assert!(result.is_err());
}

#[test]
fn test_calculate_retry_delay() {
    assert_eq!(calculate_retry_delay(1), Duration::from_secs(2));
    assert_eq!(calculate_retry_delay(2), Duration::from_secs(4));
    assert_eq!(calculate_retry_delay(3), Duration::from_secs(8));
}

#[test]
fn test_parse_user_input_empty_insert() {
    let input = "5,";
    let expected = Edit {
        position: 5,
        insert: Some("".to_string()),
        delete: None,
    };
    let result = parse_user_input(input).unwrap();
    assert_eq!(result, expected);
}

#[test]
fn test_parse_user_input_negative_position() {
    let input = "-1,hello";
    let result = parse_user_input(input);
    assert!(result.is_err());
    assert_eq!(
        result.err().unwrap(),
        "Invalid position. Please enter a number."
    );
}

#[test]
fn test_parse_user_input_zero_delete() {
    let input = "5,delete0";
    let expected = Edit {
        position: 5,
        insert: None,
        delete: Some(0),
    };
    let result = parse_user_input(input).unwrap();
    assert_eq!(result, expected);
}

#[test]
fn test_parse_user_input_invalid_delete_syntax() {
    let input = "5,delete";
    let result = parse_user_input(input);
    assert!(result.is_err());
    assert_eq!(
        result.err().unwrap(),
        "Invalid delete count. Please enter a number."
    );
}
