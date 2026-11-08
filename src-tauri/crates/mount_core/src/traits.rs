use crate::domain::Path;

pub trait PathPart {
    fn __get(&self) -> String;
}


impl PathPart for String {
    fn __get(&self) -> String {
        self.clone()
    }
}

impl PathPart for &str {
    fn __get(&self) -> String {
        self.to_string()
    }
}

impl PathPart for Path {
    fn __get(&self) -> String {
        self.get()
    }
}



#[macro_export]
macro_rules! make_path {
    () => {Path::empty()};
    ($($x:expr),* $(,)?)=>{{
        let vec = vec![$($crate::traits::PathPart::__get(&$x)), *];
        #[cfg(windows)]
        Path::new(vec.join("\\"))
        #[cfg(not(windows))]
        Path::new(vec.join("/"))
    }};
}
