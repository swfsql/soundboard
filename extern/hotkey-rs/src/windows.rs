use std::collections::HashMap;
use std::mem;
use winapi::shared::windef::HWND;
use winapi::um::winuser;

pub mod modifiers {
    use winapi::um::winuser;
    pub const ALT: u32 = winuser::MOD_ALT as u32;
    pub const CONTROL: u32 = winuser::MOD_CONTROL as u32;
    pub const SHIFT: u32 = winuser::MOD_SHIFT as u32;
    pub const SUPER: u32 = winuser::MOD_WIN as u32;
}

pub mod keys {
    use winapi::um::winuser;
    pub const BACKSPACE: u32 = winuser::VK_BACK as u32;
    pub const TAB: u32 = winuser::VK_TAB as u32;
    pub const ENTER: u32 = winuser::VK_RETURN as u32;
    pub const CAPS_LOCK: u32 = winuser::VK_CAPITAL as u32;
    pub const ESCAPE: u32 = winuser::VK_ESCAPE as u32;
    pub const SPACEBAR: u32 = winuser::VK_SPACE as u32;
    pub const PAGE_UP: u32 = winuser::VK_PRIOR as u32;
    pub const PAGE_DOWN: u32 = winuser::VK_NEXT as u32;
    pub const END: u32 = winuser::VK_END as u32;
    pub const HOME: u32 = winuser::VK_HOME as u32;
    pub const ARROW_LEFT: u32 = winuser::VK_LEFT as u32;
    pub const ARROW_RIGHT: u32 = winuser::VK_RIGHT as u32;
    pub const ARROW_UP: u32 = winuser::VK_UP as u32;
    pub const ARROW_DOWN: u32 = winuser::VK_DOWN as u32;
    pub const PRINT_SCREEN: u32 = winuser::VK_SNAPSHOT as u32;
    pub const INSERT: u32 = winuser::VK_INSERT as u32;
    pub const DELETE: u32 = winuser::VK_DELETE as u32;
}

pub type ListenerID = i32;

pub struct Listener {
    last_id: i32,
    handlers: HashMap<ListenerID, Box<dyn Fn()>>,
}

impl Listener {
    pub fn new() -> Listener {
        Listener {
            last_id: 0,
            handlers: HashMap::new(),
        }
    }

    pub fn register_hotkey<CB: 'static + Fn()>(
        &mut self,
        modifiers: u32,
        key: u32,
        handler: CB,
    ) -> Result<ListenerID, String> {
        unsafe {
            self.last_id += 1;
            let id = self.last_id;
            let result = winuser::RegisterHotKey(0 as HWND, id, modifiers, key);
            if result == 0 {
                return Err("Failed to register hotkey".to_string());
            }

            self.handlers.insert(id, Box::new(handler));
            Ok(id)
        }
    }

    pub fn listen(self) {
        unsafe {
            loop {
                let mut msg = mem::MaybeUninit::uninit().assume_init();
                while winuser::GetMessageW(&mut msg, 0 as HWND, 0, 0) > 0 {
                    if msg.wParam != 0 {
                        if let Some(handler) = self.handlers.get(&(msg.wParam as i32)) {
                            handler();
                        }
                    }
                }
            }
        }
    }
}
