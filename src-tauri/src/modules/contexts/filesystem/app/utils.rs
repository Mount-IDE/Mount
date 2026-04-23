use crate::modules::shared::kernel::values::Path;

pub fn split_path(path: &Path) -> Vec<String>{
    if cfg!(target_os = "windows") {
        let splited = path.get().split('\\').
            map(|e|e.to_string())
            .collect::<Vec<String>>();
        splited
    }
    else {
        let splited = path.get().split('/')
            .map(|e|e.to_string())
            .collect::<Vec<String>>();
        splited
    }
}



pub fn make_path_string(paths: Vec<&str>) ->String{
    if paths.len()==0{
        return "".to_string();
    }
    let mut res = String::new();
    for i in 0..paths.len()-1 {
        println!("path element {i}");
        if cfg!(target_os = "windows") {
            res = format!("{}\\", paths[i]);
        }else{
            res=format!("{}/", paths[i])     
        }
    }
    res+=paths[paths.len()-1];
    res
}

pub fn make_path(path: Vec<&str>) -> Path {
    Path(make_path_string(path))
}