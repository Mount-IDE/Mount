use crate::domain::settings::Settings;
use std::sync::{Mutex, OnceLock};

pub mod config;
pub mod settings;

pub static SETTINGS: OnceLock<Mutex<Settings>> = OnceLock::new();
