mod common;

#[test]
fn it_work() {
    assert!(true)
}

#[test]
fn it_adds_two() {
    let s = common::setup();
    assert_eq!("hello world", s)
}