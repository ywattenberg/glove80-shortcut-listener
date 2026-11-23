use crate::chord::Chord;
use std::{io, mem::size_of};
use windows_sys::Win32::UI::Input::KeyboardAndMouse::*;

const MODIFIERS: [(HOT_KEY_MODIFIERS, VIRTUAL_KEY, VIRTUAL_KEY); 4] = [
    (MOD_CONTROL, VK_LCONTROL, VK_RCONTROL),
    (MOD_SHIFT, VK_LSHIFT, VK_RSHIFT),
    (MOD_ALT, VK_LMENU, VK_RMENU),
    (MOD_WIN, VK_LWIN, VK_RWIN),
];

pub fn send(chord: Chord) -> io::Result<()> {
    let mut changes = Vec::new();
    let mut press = Vec::new();
    for (modifier, left, right) in MODIFIERS {
        let left_down = unsafe { GetAsyncKeyState(left.into()) } < 0;
        let right_down = unsafe { GetAsyncKeyState(right.into()) } < 0;
        if chord.modifiers & modifier == 0 {
            if left_down {
                changes.push((left, true));
            }
            if right_down {
                changes.push((right, true));
            }
        } else if !left_down && !right_down {
            press.push((left, false));
        }
    }
    changes.extend(press);
    changes.push((chord.key, false));
    let events: Vec<_> = changes
        .iter()
        .map(|&(key, up)| event(key, up))
        .chain(changes.iter().rev().map(|&(key, up)| event(key, !up)))
        .collect();

    let inserted = inject(&events);
    if inserted == events.len() {
        return Ok(());
    }
    // Close only the transitions that were inserted, leaving untouched keys alone.
    let remaining = inserted.max(events.len() - inserted);
    if remaining < events.len() {
        inject(&events[remaining..]);
    }
    Err(io::Error::other(format!(
        "Windows inserted {inserted} of {} keyboard events",
        events.len()
    )))
}

fn event(key: VIRTUAL_KEY, up: bool) -> INPUT {
    let scan = unsafe { MapVirtualKeyW(key.into(), MAPVK_VK_TO_VSC_EX) };
    let mut flags = if scan >> 8 == 0xE0 {
        KEYEVENTF_EXTENDEDKEY
    } else {
        0
    };
    if up {
        flags |= KEYEVENTF_KEYUP;
    }
    INPUT {
        r#type: INPUT_KEYBOARD,
        Anonymous: INPUT_0 {
            ki: KEYBDINPUT {
                wVk: key,
                wScan: 0,
                dwFlags: flags,
                time: 0,
                dwExtraInfo: 0,
            },
        },
    }
}

fn inject(events: &[INPUT]) -> usize {
    unsafe {
        SendInput(
            events.len() as u32,
            events.as_ptr(),
            size_of::<INPUT>() as i32,
        ) as usize
    }
}
