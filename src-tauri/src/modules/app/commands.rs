

#[tauri::command]
pub fn show_win(window: tauri::Window) {
    let res = window.show();
    match res {
        Err(e)=>println!("Error: {:?}", e),
        Ok(e)=>println!("Ok: {:?}", e),
    }
}