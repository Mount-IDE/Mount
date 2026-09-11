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

impl PathPart for Path {
    fn __get(&self) -> String {
        self.get().clone()
    }
}

impl<T: Into<String> + Clone> PathPart for Vec<T> {
    fn __get(&self) -> String {
        let res = self
            .iter()
            .map(|e| e.clone().into())
            .collect::<Vec<String>>();
        if cfg!(target_os = "windows") {
            return res.join("\\");
        }
        res.join("/")
    }
}

#[deprecated]
#[allow(unused)]
pub fn make_path_string<T: PathPart>(paths: Vec<T>) -> String {
    todo!();
    /*if paths.len() == 0 {
        return "".to_string();
    }
    if cfg!(target_os = "windows") {
        paths
            .iter()
            .map(|e| e.__get())
            .collect::<Vec<String>>()
            .join("\\")
    } else {
        paths
            .iter()
            .map(|e| e.__get())
            .collect::<Vec<String>>()
            .join("/")
    }*/
}
#[deprecated]
#[allow(unused)]
pub fn make_path<T: PathPart>(path: Vec<T>) -> Path {
    todo!();
    //Path(make_path_string(path))
}

macro_rules! path_from {
    () => {
        Path::empty()
    };
    ( $($x:expr), * $(,)?)=>{
       {
           let a= vec![$($x.__get() ),*];
            if cfg!(target_os = "windows") {
               Path(a.join("\\"))
            } else {
                Path(a.join("/"))
            }

       }
    };
    ($x:expr)=>{
        Path($x.__get())
    }
}

pub(crate) use path_from;
