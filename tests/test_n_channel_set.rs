mod common;

use biobase_rs::r::data_classes::{AssayData, ESet, NChannelSet};
use biobase_rs::r::methods_n_channel_set::{channel, channel_names, select_channels};

#[test]
fn test_n_channel_set_selects_and_reads_channels() {
    let mut assay_data = AssayData::new();
    assay_data.insert("R".to_string(), common::matrix(1, 2, &[1.0, 2.0]));
    assay_data.insert("G".to_string(), common::matrix(1, 2, &[3.0, 4.0]));
    let nchannel = NChannelSet {
        base: ESet::new(assay_data),
    };

    assert_eq!(channel_names(&nchannel), vec!["G", "R"]);
    assert_eq!(channel(&nchannel, "R").unwrap().values(), &[1.0, 2.0]);

    let selected = select_channels(&nchannel, &["G"]);
    assert_eq!(channel_names(&selected), vec!["G"]);
    assert!(channel(&selected, "R").is_none());
}
