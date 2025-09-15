use enigo::{
    Button,
    Direction::{Press, Release},
    Enigo, Mouse, Settings,
};

#[tauri::command]
pub fn click() {
    let mut enigo = Enigo::new(&Settings::default()).unwrap();
    enigo.button(Button::Left, Press).unwrap();
    enigo.button(Button::Left, Release).unwrap();
}
