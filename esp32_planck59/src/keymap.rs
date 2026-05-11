use rmk::types::action::KeyAction;
use rmk::{a, k, layer, mo, to, td};

pub const ROW: usize = 5;
pub const COL: usize =13;
pub const NUM_LAYER: usize = 4;

#[rustfmt::skip]
pub const fn get_default_keymap() -> [[[KeyAction; COL]; ROW]; NUM_LAYER] {
    [
        // ========== Layer 0: 基础层 ==========
        layer!([
            [k!(KcEsc), k!(Kc1), k!(Kc2), k!(Kc3), k!(Kc4), k!(Kc5), k!(Kc6), k!(Kc7), k!(Kc8), k!(Kc9), k!(Kc0), k!(KcBackslash), k!(KcBackspace)],
            [k!(KcTab), k!(KcQ), k!(KcW), k!(KcE), k!(KcR), k!(KcT), k!(KcY), k!(KcU), k!(KcI), k!(KcO), k!(KcP), k!(KcLeftBracket), k!(KcRightBracket)],
            [td!(0), k!(KcA), k!(KcS), k!(KcD), k!(KcF), k!(KcG), k!(KcH), k!(KcJ), k!(KcK), k!(KcL), k!(KcSemicolon), k!(KcQuote), td!(1)],
            [k!(KcLeftShift), k!(KcZ), k!(KcX), k!(KcC), k!(KcV), k!(KcB), k!(KcN), k!(KcM), k!(KcComma), k!(KcDot), k!(KcUp), k!(KcSlash), td!(2)],
            [k!(KcLeftCtrl), k!(KcLeftGui), k!(KcLeftAlt), k!(KcSpace), k!(KcSpace), mo!(1), a!(No), mo!(3), k!(KcSpace), k!(KcLeft), k!(KcDown), k!(KcRight), k!(KcRightCtrl)]
        ]),

        // ========== Layer 1: Fn层 (MO1) ==========
        layer!([
            [k!(KcGrave), k!(KcF1), k!(KcF2), k!(KcF3), k!(KcF4), k!(KcF5), k!(KcF6), k!(KcF7), k!(KcF8), k!(KcF9), k!(KcF10), k!(KcF11), k!(KcF12)],
            [a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No)],
            [to!(3), a!(No), a!(No), to!(0), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No)],
            [a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No)],
            [a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No)]
        ]),

        // ========== Layer 2: Stop/PrtSc层 ==========
        layer!([
            [k!(KcStop), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), k!(KcPrintScreen)],
            [a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No)],
            [a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No)],
            [a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No)],
            [k!(KcHome), k!(KcEnd), k!(KcPageUp), k!(KcPageDown), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No)]
        ]),

        // ========== Layer 3: 数字小键盘层 ==========
        layer!([
            [k!(KcEsc), k!(KcKpMinus), k!(KcKpPlus), k!(KcKpSlash), k!(KcKp1), k!(KcKp2), k!(KcKp3), k!(KcKp4), k!(KcKp5), k!(KcKp6), k!(KcKp7), k!(KcKp8), k!(KcBackspace)],
            [k!(KcKp7), k!(KcKp8), k!(KcKp9), k!(KcTab), k!(KcQ), k!(KcW), k!(KcE), k!(KcR), k!(KcT), k!(KcY), k!(KcU), k!(KcI), k!(KcO)],
            [k!(KcKp4), k!(KcKp5), k!(KcKp6), k!(KcKpAsterisk), k!(KcA), k!(KcS), k!(KcD), k!(KcF), k!(KcG), k!(KcH), k!(KcJ), k!(KcK), k!(KcL)],
            [k!(KcKp1), k!(KcKp2), k!(KcKp3), k!(KcLeftShift), k!(KcZ), k!(KcX), k!(KcC), k!(KcV), k!(KcB), k!(KcN), k!(KcM), k!(KcP), k!(KcBackslash)],
            [k!(KcBackspace), k!(KcKpDot), k!(KcKp0), k!(KcLeftCtrl), k!(KcLeftGui), mo!(1), a!(No), k!(KcLeftAlt), k!(KcSpace), k!(KcLeft), k!(KcDown), k!(KcUp), k!(KcRight)]
        ]),
    ]
}