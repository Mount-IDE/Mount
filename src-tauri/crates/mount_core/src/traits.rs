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

impl PathPart for Vec<String> {
    fn __get(&self) -> String {
        const SEP: &str = if cfg!(windows) { "\\" } else { "/" };
        self.join(SEP)
    }
}

#[macro_export]
macro_rules! make_path {
    () => {$crate::domain::Path::empty()};
    ($($x:expr),* $(,)?)=>{{
        let vec = vec![$($crate::traits::PathPart::__get(&$x)), *];
        if cfg!(windows){
            $crate::domain::Path::new(vec.join("\\"))
        }else{
            $crate::domain::Path::new(vec.join("/"))
        }
    }};
    ($x: expr)=>{$crate::domain::Path::new($x)};

}
