const CARGO_MANFIFST_DIR: &'static str = env!("CARGO_MANIFEST_DIR");

pub(crate) fn data_dir() -> String {
    return CARGO_MANFIFST_DIR.to_owned() + "/tests/data";
}

#[cfg(feature = "runtime-async-std")]
pub(crate) use async_std::test as async_test;

#[cfg(feature = "runtime-smol")]
pub(crate) use smol_potat::test as async_test;
