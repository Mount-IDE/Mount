pub fn t_error()->String{"stop-all".to_string()}

pub fn t_platform_def()->String{
    if cfg!(target_os = "windows") {
        return "win".to_string()
    }
    if cfg!(target_os = "linux") {
        return "linux".to_string()
    }
    if cfg!(target_os = "macos") {
        return "macos".to_string()
    }
    return "!win".to_string()
}

pub fn t_shell_def()->String{
    if cfg!(target_os = "windows") {
        return "cmd".to_string()
    }
    "sh".to_string()
}