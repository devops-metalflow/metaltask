#[allow(unused_imports)]
use std::path::Path;

#[allow(dead_code)]
pub const DATA1: &'static str = "\
#!/bin/bash
echo \"hello world!\"\
";

#[allow(dead_code)]
pub const DATA2: &'static str = "\
#!/bin/bash
ech \"hello world!\"\
";

#[allow(dead_code)]
pub const NAME1: &'static str = "test1.sh";

#[allow(dead_code)]
pub const NAME2: &'static str = "test2.sh";

#[allow(dead_code)]
pub const PATH: &'static str = "/tmp";

#[test]
fn test_routine() {
    // PASS
    assert!(true);
}

#[test]
fn test_create1() {
    let path = Path::new("/foo");
    let path = path.join(NAME1.to_string());
    let task = super::task::Task {
        path,
        ..Default::default()
    };

    assert!(task.create().is_err());

    let path = Path::new(PATH);
    let path = path.join(NAME1.to_string());
    let task = super::task::Task {
        data: Vec::from(DATA1),
        path,
        ..Default::default()
    };

    assert!(task.create().is_ok());
}

#[test]
fn test_create2() {
    let path = Path::new("/foo");
    let path = path.join(NAME2.to_string());
    let task = super::task::Task {
        path,
        ..Default::default()
    };

    assert!(task.create().is_err());

    let path = Path::new(PATH);
    let path = path.join(NAME2.to_string());
    let task = super::task::Task {
        data: Vec::from(DATA2),
        path,
        ..Default::default()
    };

    assert!(task.create().is_ok());
}

#[test]
fn test_run1() {
    let path = Path::new(PATH);
    let path = path.join("foo".to_string());
    let task = super::task::Task {
        path,
        ..Default::default()
    };

    assert!(task.run().is_err());

    let path = Path::new(PATH);
    let path = path.join(NAME1.to_string());
    let task = super::task::Task {
        path,
        ..Default::default()
    };

    assert!(task.run().is_ok());
    assert!(task.clean().is_ok());
}

#[test]
fn test_run2() {
    let path = Path::new(PATH);
    let path = path.join("foo".to_string());
    let task = super::task::Task {
        path,
        ..Default::default()
    };

    assert!(task.run().is_err());

    let path = Path::new(PATH);
    let path = path.join(NAME2.to_string());
    let task = super::task::Task {
        path,
        ..Default::default()
    };

    assert!(task.run().is_err());
    assert!(task.clean().is_ok());
}

#[test]
fn test_clean() {
    // PASS
    assert!(true);
}
