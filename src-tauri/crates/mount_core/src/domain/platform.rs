use serde::{Deserialize, Serialize};
use crate::traits::PathPart;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename = "lowercase")]
pub enum PlatformType {
    WINDOWS,
    MACOS,
    LINUX,
    ALL,
}

impl Into<String> for PlatformType {
    fn into(self) -> String {
        match self {
            PlatformType::WINDOWS => "windows".__get(),
            PlatformType::MACOS => "macos".__get(),
            PlatformType::LINUX => "linux".__get(),
            PlatformType::ALL => "all".__get(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Platform {
    ARRAY(Vec<PlatformType>),
    SINGLE(PlatformType),
}

impl Default for Platform {
    fn default() -> Self {
        Self::current()
    }
}

impl From<String> for Platform {
    fn from(value: String) -> Self {
        Self::SINGLE(match value.to_lowercase().as_str() {
            "windows" => PlatformType::WINDOWS,
            "linux" => PlatformType::LINUX,
            _ => PlatformType::MACOS,
        })
    }
}

impl PartialEq for Platform {
    fn eq(&self, other: &Self) -> bool {
        if let Self::SINGLE(v) = self {
            if let Self::SINGLE(v2) = other {
                return v == v2;
            }
        }
        if let Self::ARRAY(v) = self {
            if let Self::ARRAY(v2) = other {
                return v == v2;
            }
        }
        false
    }
}


impl Platform {
    pub fn unix_like() -> Self {
        Self::ARRAY(vec![PlatformType::LINUX, PlatformType::MACOS])
    }
    pub fn all() -> Self {
        Self::SINGLE(PlatformType::ALL)
    }

    pub fn arr(arr: Vec<PlatformType>) -> Self {
        Self::ARRAY(arr)
    }

    pub fn windows() -> Self {
        Self::SINGLE(PlatformType::WINDOWS)
    }

    #[allow(unused)]
    pub fn macos() -> Self {
        Self::SINGLE(PlatformType::MACOS)
    }

    #[allow(unused)]
    pub fn linux() -> Self {
        Self::SINGLE(PlatformType::LINUX)
    }

    pub fn is_correct(&self) -> bool {
        match self {
            Platform::ARRAY(v) => {
                for i in v {
                    if i.is_correct() {
                        return true;
                    }
                }
                false
            }
            Platform::SINGLE(v) => v.is_correct(),
        }
    }

    pub fn current() -> Self {
        if cfg!(target_os = "windows") {
            return Self::windows();
        }
        if cfg!(target_os = "macos") {
            return Self::macos();
        }
        Self::linux()
    }
}


impl PlatformType {
    pub fn is_correct(&self) -> bool {
        if *self == Self::LINUX && cfg!(target_os = "linux") {
            return true;
        }
        if *self == Self::MACOS && cfg!(target_os = "macos") {
            return true;
        }
        if *self == Self::WINDOWS && cfg!(target_os = "windows") {
            return true;
        }
        if *self == Self::ALL {
            return true;
        }
        false
    }

    #[allow(unused)]
    pub fn current() -> Self {
        if cfg!(target_os = "windows") {
            return Self::WINDOWS;
        }
        if cfg!(target_os = "macos") {
            return Self::MACOS;
        }
        Self::LINUX
    }
}

