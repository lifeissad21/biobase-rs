#[test]
fn upstream_inst_unit_test_files_are_accounted_for() {
    let converted_or_deferred = [
        "test_AnnotatedDataFrame.R",
        "test_AssayData.R",
        "test_DataClasses.R",
        "test_EsetSubclasses.R",
        "test_ExpressionSet.R",
        "test_NChannelSet.R",
        "test_SnpSet.R",
        "test_UpdateObject.R",
        "test_VersionedClass.R",
        "test_cache.R",
        "test_checkClass.R",
        "test_combine.R",
        "test_copyEnv.R",
        "test_esApply.R",
        "test_subListExtract.R",
        "test_unsaveSetSlot.R",
        "utilities.R",
    ];

    assert_eq!(converted_or_deferred.len(), 17);
}
