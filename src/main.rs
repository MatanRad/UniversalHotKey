use uhk_input::input::{InputManager, IDispatcher};
use uhk_input::events::InputEvent;

fn main() {
    let mut manager = InputManager::new().unwrap(); 

    loop {
        let event = manager.dispatch().unwrap();

        match event {
            Some(InputEvent::KeyboardDownEvent(keycode)) => {
                println!("KeyDown: {}", keycode);
            },
            Some(InputEvent::KeyboardUpEvent(keycode)) => {
                println!("KeyUp: {}", keycode);
            },
            Some(InputEvent::KeyboardHeldEvent(keycode)) => {
                println!("KeyHeld: {}", keycode);
            },
            _ => {}
        }
    }
    
}
