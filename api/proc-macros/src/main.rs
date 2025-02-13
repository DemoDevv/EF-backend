use api_proc_macros::AnswerFn;

#[derive(AnswerFn)]
struct Structe {
    data: i32,
}

fn main() {
    let _struct = Structe { data: 43 };

    assert_eq!(43, _struct.data);

    // let _struct = _struct.update(44);

    // assert_eq!(44, _struct.0)
}
