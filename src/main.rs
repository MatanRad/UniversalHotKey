use uhk_input::input::InputManager;
use uhk_input::events::InputEvent;

fn main() {
    let manager = InputManager; 
    let event = manager.dispatch();

    match event {
        InputEvent::KeyboardDownEvent(keycode) => {
            println!("KeyDown: {}", keycode);
        }
        _ => {}
    }
}
