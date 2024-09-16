use collaborative_editor_server::{DocumentState, Edit};

#[test]
fn test_apply_edit_insert_success() {
    let mut doc = DocumentState::new();
    let edit = Edit {
        position: 0,
        insert: Some("Hello".to_string()),
        delete: None,
        version: 0,
    };

    let result = doc.apply_edit(&edit);
    assert!(result.is_ok());
    assert_eq!(doc.content, "Hello");
    assert_eq!(doc.version, 1);
}

#[test]
fn test_apply_edit_delete_success() {
    let mut doc = DocumentState::new();
    doc.content = "Hello, World!".to_string();
    doc.version = 0;

    let edit = Edit {
        position: 6, // Adjusted position to include the space
        insert: None,
        delete: Some(7), // Adjusted delete length to remove " World!"
        version: 0,
    };

    let result = doc.apply_edit(&edit);
    assert!(result.is_ok());
    assert_eq!(doc.content, "Hello,");
    assert_eq!(doc.version, 1);
}

#[test]
fn test_apply_edit_version_mismatch() {
    let mut doc = DocumentState::new();
    doc.content = "Hello".to_string();
    doc.version = 1;

    let edit = Edit {
        position: 5,
        insert: Some(", World!".to_string()),
        delete: None,
        version: 0, // Incorrect version
    };

    let result = doc.apply_edit(&edit);
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), "Version mismatch");
    assert_eq!(doc.content, "Hello"); // Content should remain unchanged
    assert_eq!(doc.version, 1); // Version should remain unchanged
}

#[test]
fn test_apply_edit_invalid_insert_position() {
    let mut doc = DocumentState::new();
    doc.content = "Hello".to_string();
    doc.version = 0;

    let edit = Edit {
        position: 10, // Invalid position
        insert: Some(", World!".to_string()),
        delete: None,
        version: 0,
    };

    let result = doc.apply_edit(&edit);
    assert!(result.is_err());
    assert_eq!(
        result.unwrap_err(),
        "Insert position is not a valid UTF-8 boundary."
    );
    assert_eq!(doc.content, "Hello"); // Content should remain unchanged
    assert_eq!(doc.version, 0); // Version should remain unchanged
}

#[test]
fn test_apply_edit_invalid_delete_range() {
    let mut doc = DocumentState::new();
    doc.content = "Hello, World!".to_string();
    doc.version = 0;

    let edit = Edit {
        position: 7,
        insert: None,
        delete: Some(20), // Delete beyond the end
        version: 0,
    };

    let result = doc.apply_edit(&edit);
    assert!(result.is_err());
    assert_eq!(
        result.unwrap_err(),
        "Delete range is not valid UTF-8 boundaries."
    );
    assert_eq!(doc.content, "Hello, World!"); // Content should remain unchanged
    assert_eq!(doc.version, 0); // Version should remain unchanged
}

#[test]
fn test_apply_edit_consecutive_edits() {
    let mut doc = DocumentState::new();
    let edit1 = Edit {
        position: 0,
        insert: Some("Hello".to_string()),
        delete: None,
        version: 0,
    };
    let edit2 = Edit {
        position: 5,
        insert: Some(", World!".to_string()),
        delete: None,
        version: 1,
    };

    assert!(doc.apply_edit(&edit1).is_ok());
    assert_eq!(doc.content, "Hello");
    assert_eq!(doc.version, 1);

    assert!(doc.apply_edit(&edit2).is_ok());
    assert_eq!(doc.content, "Hello, World!");
    assert_eq!(doc.version, 2);
}
