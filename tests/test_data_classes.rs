use biobase_rs::r::any_missing::any_missing;
use biobase_rs::r::data_classes::{Miame, ScalarObject, Value};
use biobase_rs::r::methods_miame::miame;
use biobase_rs::r::methods_scalar_object::mk_scalar;

#[test]
fn test_new() {
    assert_eq!(Miame::default(), miame());
    assert_eq!(
        mk_scalar(Value::Integer(4)),
        Some(ScalarObject::Integer(Some(4)))
    );
    assert!(mk_scalar(Value::Null).is_none());
    assert!(any_missing(&[Value::Integer(1), Value::Null]));
}

#[test]
fn test_miame_construction() {
    let mut direct = Miame::default();
    direct.name = "mytest".to_string();

    let mut constructed = miame();
    constructed.name = "mytest".to_string();

    assert_eq!(direct, constructed);
}
