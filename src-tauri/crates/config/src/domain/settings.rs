use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use mount_core::domain::Path;
use mount_core::traits::PathPart;
//use ts_rs::TS;

///
///
///
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Settings {
    #[serde(default)]
    pub doctype: String,
    #[serde(default)]
    pub version: String,
    #[serde(default)]
    pub general: GeneralSettings,
    #[serde(default)]
    pub appearance: Appearance,
    #[serde(default)]
    pub run: Run,
    #[serde(default)]
    pub licenses: HashMap<String, SettingsLicenseEntry>,
}

#[derive(Serialize, Default, Deserialize, Clone, Debug)]
pub struct SettingsLicenseEntry {
    pub filename: String,
    pub entries_input: Option<HashMap<String, Vec<String>>>, // UI params -> text in license
    pub entries_var: Option<HashMap<String, Vec<String>>>,   // var name -> text in license
}

impl Settings {
    pub fn new() -> Settings {
        Self {
            doctype: String::from("opie/mount"),
            version: String::from("1.0.0"),
            general: GeneralSettings::new(),
            appearance: Appearance::new(),
            run: Run::default(),
            licenses: HashMap::from([
                ("Apache-2.0".__get(), SettingsLicenseEntry {
                    filename: "apache-2.0.txt".to_string(),
                    entries_input: Some(HashMap::from([
                        ("Year".__get(), vec!["[yyyy]".__get()]),
                    ])),
                    entries_var: Some(
                        HashMap::from([
                            ("project-authors".__get(), vec!["[name of copyright owner]".__get()])]
                        )
                    ),
                }),
                ("BSD-3.0".__get(), SettingsLicenseEntry {
                    filename: "bsd-3.0.txt".__get(),
                    entries_input: Some(HashMap::from([
                        ("Year".__get(), vec!["[year]".__get()]),
                    ])),
                    entries_var: Some(
                        HashMap::from([
                            ("project-authors".__get(), vec!["[authors]".__get()])
                        ])
                    ),
                }),
                ("GPL-3.0".__get(), SettingsLicenseEntry {
                    filename: "gpl-3.0.txt".__get(),
                    entries_input: Some(HashMap::from([
                        ("Year".__get(), vec!["<year>".__get()]),
                    ])),
                    entries_var: Some(
                        HashMap::from([
                            ("project-authors".__get(), vec!["<name of authors>".__get(), "<name of author>".__get()]),
                            ("project-name".__get(), vec!["<program>".__get(), "<one line to give the program's name and a brief idea of what it does.>".__get()]),
                        ])
                    ),
                }), ("LGPL-3.0".__get(), SettingsLicenseEntry {
                    filename: "gpl-3.0.txt".__get(),
                    entries_input: Some(HashMap::from([
                        ("Year".__get(), vec!["<year>".__get()]),
                    ])),
                    entries_var: Some(
                        HashMap::from([
                            ("project-authors".__get(), vec!["<name of authors>".__get(), "<name of author>".__get()]),
                            ("project-name".__get(), vec!["<program>".__get(), "<one line to give the program's name and a brief idea of what it does.>".__get()]),
                        ])
                    ),
                }), ("MIT".__get(), SettingsLicenseEntry {
                    filename: "mit.txt".__get(),
                    entries_input: Some(HashMap::from([
                        ("Year".__get(), vec!["[year]".__get()]),
                    ])),
                    entries_var: Some(
                        HashMap::from([
                            ("project-authors".__get(), vec!["[authors]".__get()]),
                        ])
                    ),
                }),
            ]),
        }
    }
}
///
///
///
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Run {
    pub shells: Vec<String>,
}

impl Default for Run {
    fn default() -> Self {
        Self {
            shells: vec![
                "bash".to_string(),
                "sh".to_string(),
                "zsh".to_string(),
                "ksh".to_string(),
                "cmd".to_string(),
                "powershell".to_string(),
            ],
        }
    }
}

///
///
///
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GeneralSettings {
    pub path_to_projects: Path,
    pub project_groups: Vec<String>,
}

impl Default for GeneralSettings {
    fn default() -> Self {
        Self {
            path_to_projects: Path("".to_string()),
            project_groups: vec![String::from("general")],
        }
    }
}

impl GeneralSettings {
    pub fn new() -> GeneralSettings {
        Self {
            path_to_projects: Path(String::new()),
            project_groups: Vec::new(),
        }
    }
}

///
///
///
///
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Appearance {
    pub theme: String,
    pub lang: String,
    pub font: String,
    pub font_size: u64,
}

impl Default for Appearance {
    fn default() -> Self {
        Self {
            theme: "opie.dark".to_string(),
            lang: "en".to_string(),
            font: "Jetbrains Mono".to_string(),
            font_size: 16,
        }
    }
}

impl Appearance {
    pub fn new() -> Appearance {
        Self {
            theme: String::from("opie.dark"),
            lang: String::new(),
            font: String::new(),
            font_size: 16,
        }
    }
}

