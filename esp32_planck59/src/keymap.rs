use rmk::types::action::KeyAction;
use rmk::{a, k, layer, mo, to, td, lt, lm};

pub(crate) const COL: usize = 13;
pub(crate) const ROW: usize = 5;
pub(crate) const NUM_LAYER: usize = 4;
pub(crate) const NUM_ENCODER: usize = 0;  // 没有编码器

#[rustfmt::skip]
pub const fn get_default_keymap() -> [[[KeyAction; COL]; ROW]; NUM_LAYER] {
    [
        // Layer 0: 基础层
        layer!([
            [k!(Escape), k!(Kc1), k!(Kc2), k!(Kc3), k!(Kc4), k!(Kc5), k!(Kc6), k!(Kc7), k!(Kc8), k!(Kc9), k!(Kc0), k!(Backslash), k!(Backspace)],
            [k!(Tab), k!(Q), k!(W), k!(E), k!(R), k!(T), k!(Y), k!(U), k!(I), k!(O), k!(P), k!(LeftBracket), k!(RightBracket)],
            [td!(0), k!(A), k!(S), k!(D), k!(F), k!(G), k!(H), k!(J), k!(K), k!(L), k!(Semicolon), k!(Quote), td!(1)],
            [k!(LShift), k!(Z), k!(X), k!(C), k!(V), k!(B), k!(N), k!(M), k!(Comma), k!(Dot), k!(Up), k!(Slash), td!(2)],
            [k!(LCtrl), k!(LGui), k!(LAlt), k!(Space), k!(Space), mo!(1), a!(No), mo!(3), k!(Space), k!(Left), k!(Down), k!(Right), k!(RCtrl)]
        ]),
        // Layer 1: Fn层
        layer!([
            [k!(Grave), k!(F1), k!(F2), k!(F3), k!(F4), k!(F5), k!(F6), k!(F7), k!(F8), k!(F9), k!(F10), k!(F11), k!(F12)],
            [a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No)],
            [to!(3), a!(No), a!(No), to!(0), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No)],
            [a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No)],
            [a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No)]
        ]),
        // Layer 2: Sleep/PrtSc层
        layer!([
            [k!(Stop), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), k!(PrintScreen)],
            [a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No)],
            [a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No)],
            [a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No)],
            [k!(Home), k!(End), k!(PageUp), k!(PageDown), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No)]
        ]),
        // Layer 3: 数字小键盘层
        layer!([
            [k!(Escape), k!(KpMinus), k!(KpPlus), k!(KpSlash), k!(Kp1), k!(Kp2), k!(Kp3), k!(Kp4), k!(Kp5), k!(Kp6), k!(Kp7), k!(Kp8), k!(Backspace)],
            [k!(Kp7), k!(Kp8), k!(Kp9), k!(Tab), k!(Q), k!(W), k!(E), k!(R), k!(T), k!(Y), k!(U), k!(I), k!(O)],
            [k!(Kp4), k!(Kp5), k!(Kp6), k!(KpAsterisk), k!(A), k!(S), k!(D), k!(F), k!(G), k!(H), k!(J), k!(K), k!(L)],
            [k!(Kp1), k!(Kp2), k!(Kp3), k!(LShift), k!(Z), k!(X), k!(C), k!(V), k!(B), k!(N), k!(M), k!(P), k!(Backslash)],
            [k!(Backspace), k!(KpDot), k!(Kp0), k!(LCtrl), k!(LGui), mo!(1), a!(No), k!(LAlt), k!(Space), k!(Left), k!(Down), k!(Up), k!(Right)]
        ]),
    ]
}

