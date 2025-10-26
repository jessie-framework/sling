use clap::Parser;
use sling_args::Args;
use std::sync::LazyLock;

pub static GLOBALS: LazyLock<Args> = LazyLock::new(Args::parse);
