use crate::modules::shared::kernel::values::Path;

pub fn split_path(path: &Path) -> Vec<String> {
    if cfg!(target_os = "windows") {
        let splited = path
            .get()
            .split('\\')
            .map(|e| e.to_string())
            .collect::<Vec<String>>();
        splited
    } else {
        let splited = path
            .get()
            .split('/')
            .map(|e| e.to_string())
            .collect::<Vec<String>>();
        splited
    }
}

pub trait PathPart {
    fn __get(&self) -> String;
}

impl PathPart for &str {
    fn __get(&self) -> String {
        self.to_string()
    }
}

impl PathPart for String {
    fn __get(&self) -> String {
        self.clone()
    }
}

pub fn make_path_string<T: PathPart>(paths: Vec<T>) -> String {
    if paths.len() == 0 {
        return "".to_string();
    }
    let mut res = String::new();
    if cfg!(target_os = "windows") {
        res = paths
            .iter()
            .map(|e| e.__get())
            .collect::<Vec<String>>()
            .join("\\");
    } else {
        res = paths
            .iter()
            .map(|e| e.__get())
            .collect::<Vec<String>>()
            .join("/");
    }
    // println!("path end {}", res.clone());
    res
}

pub fn make_path<T: PathPart>(path: Vec<T>) -> Path {
    Path(make_path_string(path))
}

macro_rules! path_from {
    () => {
        Path::empty()
    };
    ( $($x:expr), * $(,)?)=>{
        make_path( vec![$($x),*])
    };
}

pub(crate) use path_from;
