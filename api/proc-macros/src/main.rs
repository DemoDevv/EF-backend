use api_model_traits::update::Updatable;
use api_proc_macros::Updatable;

#[derive(Updatable, Clone)]
struct User {
    name: String,
    #[updatable]
    age: i32,
}

struct UpdatableUser {
    age: Option<i32>,
}

fn main() {
    let _struct = User {
        name: "John".to_string(),
        age: 43,
    };

    assert_eq!(43, _struct.age);

    let _updatable_struct = UpdatableUser { age: Some(42) };
    let _updated_struct = _struct.perform_update(_updatable_struct).unwrap();

    assert_eq!("John", _updated_struct.name);
    assert_eq!(42, _updated_struct.age);
}
