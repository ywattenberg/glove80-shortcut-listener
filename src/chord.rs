use windows_sys::Win32::UI::Input::KeyboardAndMouse::*;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Chord {
    pub modifiers: HOT_KEY_MODIFIERS,
    pub key: VIRTUAL_KEY,
}

pub fn trigger(id: i32) -> Chord {
    let modifiers = match id {
        0..=11 => 0,
        12..=23 => MOD_CONTROL | MOD_SHIFT,
        _ => MOD_CONTROL | MOD_ALT,
    };
    Chord {
        modifiers,
        key: VK_F13 + id as u16 % 12,
    }
}

impl Chord {
    pub fn parse(input: &str) -> Result<Self, String> {
        let mut parts = input.split('+').map(str::trim);
        let key = parts.next_back().unwrap_or_default();
        let mut modifiers = 0;
        for part in parts {
            let modifier = match part.to_ascii_uppercase().as_str() {
                "CTRL" | "CONTROL" => MOD_CONTROL,
                "SHIFT" => MOD_SHIFT,
                "ALT" => MOD_ALT,
                "WIN" | "WINDOWS" => MOD_WIN,
                _ => {
                    return Err(format!(
                        "unknown modifier {part:?}; use Ctrl, Shift, Alt or Win"
                    ))
                }
            };
            if modifiers & modifier != 0 {
                return Err(format!("duplicate modifier {part:?}"));
            }
            modifiers |= modifier;
        }
        let key = virtual_key(&key.to_ascii_uppercase()).ok_or_else(|| {
            format!("unknown key {key:?}; enter a key name such as A, Tab, F5 or Plus")
        })?;
        Ok(Self { modifiers, key })
    }
}

fn virtual_key(name: &str) -> Option<VIRTUAL_KEY> {
    if let [key] = name.as_bytes() {
        if key.is_ascii_alphanumeric() {
            return Some(u16::from(*key));
        }
    }
    if let Some(number) = name.strip_prefix('F').and_then(|n| n.parse::<u16>().ok()) {
        return (1..=24).contains(&number).then(|| VK_F1 + number - 1);
    }
    if let Some([digit]) = name.strip_prefix("NUMPAD").map(str::as_bytes) {
        if digit.is_ascii_digit() {
            return Some(VK_NUMPAD0 + u16::from(digit - b'0'));
        }
    }
    if let Some(hex) = name.strip_prefix("VK_") {
        let key = u16::from_str_radix(hex, 16).ok()?;
        return match key {
            // Exclude mouse buttons, modifier-only keys, and non-keyboard ranges.
            0x08..=0x0F | 0x13..=0x5A | 0x5D..=0x87 | 0x90..=0x9F | 0xA6..=0xFE => Some(key),
            _ => None,
        };
    }
    Some(match name {
        "BACKSPACE" => VK_BACK,
        "TAB" => VK_TAB,
        "ENTER" | "RETURN" => VK_RETURN,
        "ESC" | "ESCAPE" => VK_ESCAPE,
        "SPACE" => VK_SPACE,
        "PAGEUP" | "PGUP" => VK_PRIOR,
        "PAGEDOWN" | "PGDN" => VK_NEXT,
        "HOME" => VK_HOME,
        "END" => VK_END,
        "LEFT" => VK_LEFT,
        "UP" => VK_UP,
        "RIGHT" => VK_RIGHT,
        "DOWN" => VK_DOWN,
        "INSERT" | "INS" => VK_INSERT,
        "DELETE" | "DEL" => VK_DELETE,
        "PRINTSCREEN" => VK_SNAPSHOT,
        "PAUSE" => VK_PAUSE,
        "CAPSLOCK" => VK_CAPITAL,
        "NUMLOCK" => VK_NUMLOCK,
        "SCROLLLOCK" => VK_SCROLL,
        "APPS" | "CONTEXTMENU" => VK_APPS,
        "PLUS" | "=" => VK_OEM_PLUS,
        "MINUS" | "-" => VK_OEM_MINUS,
        "COMMA" | "," => VK_OEM_COMMA,
        "PERIOD" | "." => VK_OEM_PERIOD,
        "SEMICOLON" | ";" => VK_OEM_1,
        "SLASH" | "/" => VK_OEM_2,
        "BACKTICK" | "`" => VK_OEM_3,
        "LEFTBRACKET" | "[" => VK_OEM_4,
        "BACKSLASH" | "\\" => VK_OEM_5,
        "RIGHTBRACKET" | "]" => VK_OEM_6,
        "QUOTE" | "'" => VK_OEM_7,
        "NUMPADADD" => VK_ADD,
        "NUMPADSUBTRACT" => VK_SUBTRACT,
        "NUMPADMULTIPLY" => VK_MULTIPLY,
        "NUMPADDIVIDE" => VK_DIVIDE,
        "NUMPADDECIMAL" => VK_DECIMAL,
        _ => return None,
    })
}
