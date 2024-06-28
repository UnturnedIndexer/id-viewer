use std::path::PathBuf;

pub mod asset;

pub trait Parser<T> {
    fn parse<P: Into<PathBuf>>(path: P) -> anyhow::Result<T>;
}
