use biobase_rs::r::tools::check_class;

#[test]
fn test_check_data_s3_extended_data_frame() {
    assert!(check_class("data.frame", &["data.frame"]).is_ok());
    assert!(check_class("bar", &["data.frame", "bar"]).is_ok());

    let err = check_class("bar", &["data.frame"]).unwrap_err();
    assert!(err.contains("data.frame"));
}
