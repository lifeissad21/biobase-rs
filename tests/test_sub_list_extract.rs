use std::collections::BTreeMap;

use biobase_rs::r::data_classes::Value;
use biobase_rs::r::tools::sub_list_extract;

#[test]
fn test_sub_list_extract_basic_api() {
    let values = (1..=20)
        .map(|idx| {
            BTreeMap::from([
                ("i".to_string(), Value::Integer(4)),
                ("d".to_string(), Value::Number(1.23)),
                ("l".to_string(), Value::Bool(false)),
                ("n".to_string(), Value::Null),
                ("na".to_string(), Value::Number(f64::NAN)),
                ("s".to_string(), Value::Text("foo".to_string())),
                ("idx".to_string(), Value::Integer(idx)),
            ])
        })
        .collect::<Vec<_>>();

    assert_eq!(
        sub_list_extract(&values, "i").unwrap(),
        vec![Value::Integer(4); 20]
    );
    assert_eq!(
        sub_list_extract(&values, "s").unwrap(),
        vec![Value::Text("foo".to_string()); 20]
    );
    assert!(sub_list_extract(&values, "missing").is_err());
    assert!(sub_list_extract::<Value>(&[], "foo").unwrap().is_empty());
}
