const CARGO_MANFIFST_DIR: &'static str = env!("CARGO_MANIFEST_DIR");

pub fn data_dir() -> String {
    return CARGO_MANFIFST_DIR.to_owned() + "/tests/data";
}
