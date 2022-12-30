use std::{fs, process::Command};

#[test]
fn c_testsuite() {
    fs::read_dir("tests/c-testsuite")
        .unwrap()
        .map(|entry| entry.unwrap().path())
        .for_each(|path| {
            let source = fs::read_to_string(path).unwrap();
            mai::run(&source).unwrap();

            let status = Command::new("./a.out").status().unwrap();
            assert_eq!(status.code(), Some(0));
        });
}
