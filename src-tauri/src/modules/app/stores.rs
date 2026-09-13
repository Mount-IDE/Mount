use crate::modules::contexts::toolchains::domain::Toolchain;
use std::sync::{Arc, Mutex, OnceLock};

pub static TOOLCHAINS: OnceLock<Arc<Mutex<Vec<Toolchain>>>> = OnceLock::new();
