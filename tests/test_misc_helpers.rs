use biobase_rs::r::data_classes::Value;
use biobase_rs::r::environment::{Environment, multiassign};
use biobase_rs::r::tools::{list_len, valid_msg};

#[test]
fn test_miscellaneous_helpers_from_inst_suite() {
    let names = vec!["a".to_string(), "b".to_string()];
    let values = vec![Value::Integer(1), Value::Integer(2)];
    let mut env = Environment::new();
    multiassign(&names, &values, &mut env).unwrap();
    assert_eq!(env["a"], Value::Integer(1));
    assert!(multiassign(&names, &values[..1], &mut env).is_err());

    assert_eq!(list_len(&[vec![1, 2], vec![], vec![3]]), vec![2, 0, 1]);
    assert_eq!(
        valid_msg(Vec::new(), Err(vec!["bad object".to_string()])),
        vec!["bad object".to_string()]
    );
}
