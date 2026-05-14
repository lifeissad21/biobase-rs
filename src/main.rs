fn main() {
    println!(
        "biobase-rs data module loaded: {} amino acids, {} gene covariate rows",
        biobase_rs::data::AA_MAP.len(),
        biobase_rs::data::GENE_COV.len()
    );
}
