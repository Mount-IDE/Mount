use crate::modules::contexts::filesystem::app::utils::PathPart;
use crate::modules::shared::kernel::values::PlatformType;

#[inline]
pub fn get_os() -> String {
    if cfg!(target_os = "windows") {
        return "windows".to_string();
    }
    if cfg!(target_os = "macos") {
        return "macos".to_string();
    }
    "linux".to_string()
}

#[inline]
pub fn get_platform() -> PlatformType {
    if cfg!(windows) {
        return PlatformType::WINDOWS;
    }
    if cfg!(target_os = "linux") {
        return PlatformType::LINUX;
    }
    PlatformType::MACOS
}

pub fn shell_default() -> String {
    if cfg!(windows) {
        return "cmd".__get();
    }
    if cfg!(target_os = "linux") {
        return "sh".__get();
    }
    "zsh".__get()
}
