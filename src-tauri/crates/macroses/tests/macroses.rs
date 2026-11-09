use macroses::join;

#[test]
pub fn join() {
    let res = join!("::", "str", "str", "str");
    assert_eq!(res, "str::str::str");
}
