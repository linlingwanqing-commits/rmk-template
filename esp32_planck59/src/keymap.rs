use rmk::types::action::KeyAction;
use rmk::{a, k, layer, mo, to, td};

pub(crate) const COL: usize = 13;
pub(crate) const ROW: usize = 5;
pub(crate) const NUM_LAYER: usize = 4;

// 定义 Tap Dance 键的 ID
// 对应配置中的 TD(0)、TD(1)、TD(2)
const TD_ENTER_MO2_CAPSLOCK: usize = 0;
const TD_ENTER_MO2_NUMLOCK: usize = 1;
const TD_DELETE_RSHIFT: usize = 2;

#[rustfmt::skip]
pub const fn get_default_keymap() -> [[[KeyAction; COL]; ROW]; NUM_LAYER] {
    [
        // ========== Layer 0: 基础层 ==========
        layer!([
            // 行0
            [k!(KcEsc), k!(Kc1), k!(Kc2), k!(Kc3), k!(Kc4), k!(Kc5), k!(Kc6), k!(Kc7), k!(Kc8), k!(Kc9), k!(Kc0), k!(KcBackslash), k!(KcBackspace)],
            // 行1
            [k!(KcTab), k!(KcQ), k!(KcW), k!(KcE), k!(KcR), k!(KcT), k!(KcY), k!(KcU), k!(KcI), k!(KcO), k!(KcP), k!(KcLeftBracket), k!(KcRightBracket)],
            // 行2
            [td!(TD_ENTER_MO2_CAPSLOCK), k!(KcA), k!(KcS), k!(KcD), k!(KcF), k!(KcG), k!(KcH), k!(KcJ), k!(KcK), k!(KcL), k!(KcSemicolon), k!(KcQuote), td!(TD_ENTER_MO2_NUMLOCK)],
            // 行3
            [k!(KcLeftShift), k!(KcZ), k!(KcX), k!(KcC), k!(KcV), k!(KcB), k!(KcN), k!(KcM), k!(KcComma), k!(KcDot), k!(KcUp), k!(KcSlash), td!(TD_DELETE_RSHIFT)],
            // 行4
            [k!(KcLeftCtrl), k!(KcLeftGui), k!(KcLeftAlt), k!(KcSpace), k!(KcSpace), mo!(1), a!(No), mo!(3), k!(KcSpace), k!(KcLeft), k!(KcDown), k!(KcRight), k!(KcRightCtrl)]
        ]),

        // ========== Layer 1: Fn层 (MO1) ==========
        layer!([
            // 行0
            [k!(KcGrave), k!(KcF1), k!(KcF2), k!(KcF3), k!(KcF4), k!(KcF5), k!(KcF6), k!(KcF7), k!(KcF8), k!(KcF9), k!(KcF10), k!(KcF11), k!(KcF12)],
            // 行1
            [a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No)],
            // 行2
            [to!(3), a!(No), a!(No), to!(0), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No)],
            // 行3
            [a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No)],
            // 行4
            [a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No)]
        ]),

        // ========== Layer 2: Stop/PrtSc层 ==========
        layer!([
            // 行0
            [k!(KcStop), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), k!(KcPrintScreen)],
            // 行1
            [a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No)],
            // 行2
            [a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No)],
            // 行3
            [a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No)],
            // 行4
            [k!(KcHome), k!(KcEnd), k!(KcPageUp), k!(KcPageDown), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No)]
        ]),

        // ========== Layer 3: 数字小键盘层 ==========
        layer!([
            // 行0
            [k!(KcEsc), k!(KcKpMinus), k!(KcKpPlus), k!(KcKpSlash), k!(KcKp1), k!(KcKp2), k!(KcKp3), k!(KcKp4), k!(KcKp5), k!(KcKp6), k!(KcKp7), k!(KcKp8), k!(KcBackspace)],
            // 行1
            [k!(KcKp7), k!(KcKp8), k!(KcKp9), k!(KcTab), k!(KcQ), k!(KcW), k!(KcE), k!(KcR), k!(KcT), k!(KcY), k!(KcU), k!(KcI), k!(KcO)],
            // 行2
            [k!(KcKp4), k!(KcKp5), k!(KcKp6), k!(KcKpAsterisk), k!(KcA), k!(KcS), k!(KcD), k!(KcF), k!(KcG), k!(KcH), k!(KcJ), k!(KcK), k!(KcL)],
            // 行3
            [k!(KcKp1), k!(KcKp2), k!(KcKp3), k!(KcLeftShift), k!(KcZ), k!(KcX), k!(KcC), k!(KcV), k!(KcB), k!(KcN), k!(KcM), k!(KcP), k!(KcBackslash)],
            // 行4
            [k!(KcBackspace), k!(KcKpDot), k!(KcKp0), k!(KcLeftCtrl), k!(KcLeftGui), mo!(1), a!(No), k!(KcLeftAlt), k!(KcSpace), k!(KcLeft), k!(KcDown), k!(KcUp), k!(KcRight)]
        ]),
    ]
}

// 如果需要配置 Tap Dance 的行为，需要在主文件中添加：
//
// use rmk::behavior::morse::MorseAction;
// use rmk::config::{morse::Morse, keyboard::KeyboardConfig};
//
// pub fn custom_morse_config() -> Vec<Morse> {
//     vec![
//         // TD(0): tap=Enter, hold=MO(2), hold_after_tap=CapsLock
//         Morse::new(&[
//             MorseAction::Tap(k!(KcEnter)),
//             MorseAction::Hold(mo!(2)),
//             MorseAction::HoldAfterTap(k!(KcCapsLock)),
//         ]),
//         // TD(1): tap=Enter, hold=MO(2), hold_after_tap=NumLock
//         Morse::new(&[
//             MorseAction::Tap(k!(KcEnter)),
//             MorseAction::Hold(mo!(2)),
//             MorseAction::HoldAfterTap(k!(KcNumLock)),
//         ]),
//         // TD(2): tap=Delete, hold=RShift
//         Morse::new(&[
//             MorseAction::Tap(k!(KcDelete)),
//             MorseAction::Hold(k!(KcRightShift)),
//         ]),
//     ]
// }
// 
// 然后在 main.rs 或 lib.rs 中，创建 KeyboardConfig 时添加：
// let config = KeyboardConfig::default().with_morses(custom_morse_config());