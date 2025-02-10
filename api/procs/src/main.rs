use proc_tests::AnswerFn;

#[derive(AnswerFn)]
struct Struct(i32);

fn main() {
    let _struct = Struct(43);

    assert_eq!(43, _struct.0);

    let _struct = _struct.update(44);

    assert_eq!(44, _struct.0)
}
