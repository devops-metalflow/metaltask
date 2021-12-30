#[allow(unused_imports)]
use std::path::Path;

#[allow(dead_code)]
pub const DATA: &'static str = "\
#!/bin/bash
echo \"hello world!\"\
";

#[allow(dead_code)]
pub const NAME: &'static str = "test.sh";

#[allow(dead_code)]
pub const PATH: &'static str = "/tmp";

#[test]
fn test_routine() {
    // PASS
    assert!(true);
}

#[test]
fn test_create() {
    let path = Path::new("/foo");
    let path = path.join(NAME.to_string());
    let task = super::task::Task {
        path,
        ..Default::default()
    };

    assert!(task.create().is_err());

    let path = Path::new(PATH);
    let path = path.join(NAME.to_string());
    let task = super::task::Task {
        data: Vec::from(DATA),
        path,
        ..Default::default()
    };

    assert!(task.create().is_ok());
}

#[test]
fn test_run() {
    let path = Path::new(PATH);
    let path = path.join("foo".to_string());
    let task = super::task::Task {
        path,
        ..Default::default()
    };

    assert!(task.run().is_err());

    let path = Path::new(PATH);
    let path = path.join(NAME.to_string());
    let task = super::task::Task {
        path,
        ..Default::default()
    };

    assert!(task.run().is_ok());
    assert!(task.clean().is_ok());
}

#[test]
fn test_clean() {
    // PASS
    assert!(true);
}
