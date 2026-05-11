use rmk::types::action::KeyAction;
use rmk::{a, k, layer, mo, to, td};

pub const ROW: usize = 5;
pub const COL: usize = 13;
pub const NUM_LAYER: usize = 4;

#[rustfmt::skip]
pub const fn get_default_keymap() -> [[[KeyAction; COL]; ROW]; NUM_LAYER] {
    [
        // ========== Layer 0: 基础层 ==========
        layer!([
            [k!(Escape), k!(Key1), k!(Key2), k!(Key3), k!(Key4), k!(Key5), k!(Key6), k!(Key7), k!(Key8), k!(Key9), k!(Key0), k!(Backslash), k!(Backspace)],
            [k!(Tab), k!(KeyQ), k!(KeyW), k!(KeyE), k!(KeyR), k!(KeyT), k!(KeyY), k!(KeyU), k!(KeyI), k!(KeyO), k!(KeyP), k!(LeftBracket), k!(RightBracket)],
            [td!(0), k!(KeyA), k!(KeyS), k!(KeyD), k!(KeyF), k!(KeyG), k!(KeyH), k!(KeyJ), k!(KeyK), k!(KeyL), k!(Semicolon), k!(Quote), td!(1)],
            [k!(LeftShift), k!(KeyZ), k!(KeyX), k!(KeyC), k!(KeyV), k!(KeyB), k!(KeyN), k!(KeyM), k!(Comma), k!(Period), k!(UpArrow), k!(Slash), td!(2)],
            [k!(LeftControl), k!(LeftGui), k!(LeftAlt), k!(Space), k!(Space), mo!(1), a!(No), mo!(3), k!(Space), k!(LeftArrow), k!(DownArrow), k!(RightArrow), k!(RightControl)]
        ]),

        // ========== Layer 1: Fn层 (MO1) ==========
        layer!([
            [k!(Grave), k!(F1), k!(F2), k!(F3), k!(F4), k!(F5), k!(F6), k!(F7), k!(F8), k!(F9), k!(F10), k!(F11), k!(F12)],
            [a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No)],
            [to!(3), a!(No), a!(No), to!(0), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No)],
            [a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No)],
            [a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No)]
        ]),

        // ========== Layer 2: Stop/PrtSc层 ==========
        layer!([
            [k!(Stop), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), k!(PrintScreen)],
            [a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No)],
            [a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No)],
            [a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No)],
            [k!(Home), k!(End), k!(PageUp), k!(PageDown), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No)]
        ]),

        // ========== Layer 3: 数字小键盘层 ==========
        layer!([
            [k!(Escape), k!(KpMinus), k!(KpPlus), k!(KpSlash), k!(Kp1), k!(Kp2), k!(Kp3), k!(Kp4), k!(Kp5), k!(Kp6), k!(Kp7), k!(Kp8), k!(Backspace)],
            [k!(Kp7), k!(Kp8), k!(Kp9), k!(Tab), k!(KeyQ), k!(KeyW), k!(KeyE), k!(KeyR), k!(KeyT), k!(KeyY), k!(KeyU), k!(KeyI), k!(KeyO)],
            [k!(Kp4), k!(Kp5), k!(Kp6), k!(KpAsterisk), k!(KeyA), k!(KeyS), k!(KeyD), k!(KeyF), k!(KeyG), k!(KeyH), k!(KeyJ), k!(KeyK), k!(KeyL)],
            [k!(Kp1), k!(Kp2), k!(Kp3), k!(LeftShift), k!(KeyZ), k!(KeyX), k!(KeyC), k!(KeyV), k!(KeyB), k!(KeyN), k!(KeyM), k!(KeyP), k!(Backslash)],
            [k!(Backspace), k!(KpDot), k!(Kp0), k!(LeftControl), k!(LeftGui), mo!(1), a!(No), k!(LeftAlt), k!(Space), k!(LeftArrow), k!(DownArrow), k!(UpArrow), k!(RightArrow)]
        ]),
    ]
}