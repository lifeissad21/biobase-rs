use biobase_rs::r::environment::{Environment, copy_env, copy_env_with_all_names};

#[test]
fn basic_test() {
    let mut env1 = Environment::new();
    env1.insert("a".to_string(), vec![1, 2, 3]);
    env1.insert("b".to_string(), vec![4]);
    env1.insert(".hidden".to_string(), vec![31]);

    let mut visible_only = Environment::new();
    copy_env(&env1, &mut visible_only);
    assert_eq!(visible_only["a"], vec![1, 2, 3]);
    assert_eq!(visible_only["b"], vec![4]);
    assert!(!visible_only.contains_key(".hidden"));

    let mut all_names = Environment::new();
    copy_env_with_all_names(&env1, &mut all_names, true);
    assert_eq!(all_names[".hidden"], vec![31]);

    env1.get_mut("a").unwrap()[0] = 10;
    assert_eq!(all_names["a"][0], 1);
}
