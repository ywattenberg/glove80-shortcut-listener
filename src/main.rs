mod config;
use config::{Action, Bindings};
use std::{
    env, fs, io,
    io::Write,
    os::windows::process::CommandExt,
    path::{Path, PathBuf},
    process::Command,
    ptr::null_mut,
    sync::mpsc,
};
use windows_sys::Win32::{
    Foundation::*,
    System::Threading::*,
    UI::{Input::KeyboardAndMouse::*, WindowsAndMessaging::*},
};

fn launch(exe: &str, args: &[String], cwd: Option<&str>) -> io::Result<()> {
    let mut command = Command::new(exe);
    command.args(args).creation_flags(CREATE_NO_WINDOW);
    if let Some(dir) = cwd {
        command.current_dir(dir);
    }
    command.spawn().map(|_| ())
}

struct Search<'a> {
    process: &'a str,
    found: HWND,
}
unsafe extern "system" fn find_window(hwnd: HWND, data: LPARAM) -> BOOL {
    if IsWindowVisible(hwnd) == 0 || !GetWindow(hwnd, GW_OWNER).is_null() {
        return 1;
    }
    let search = &mut *(data as *mut Search);
    let mut pid = 0;
    GetWindowThreadProcessId(hwnd, &mut pid);
    let handle = OpenProcess(PROCESS_QUERY_LIMITED_INFORMATION, 0, pid);
    if handle.is_null() {
        return 1;
    }
    let mut path = [0u16; 32768];
    let mut size = path.len() as u32;
    let ok = QueryFullProcessImageNameW(handle, 0, path.as_mut_ptr(), &mut size);
    CloseHandle(handle);
    if ok != 0 {
        let path = String::from_utf16_lossy(&path[..size as usize]);
        if Path::new(&path)
            .file_name()
            .and_then(|s| s.to_str())
            .is_some_and(|s| s.eq_ignore_ascii_case(search.process))
        {
            search.found = hwnd;
            return 0;
        }
    }
    1
}

fn focus_or_launch(process: &str, exe: &str, args: &[String]) -> io::Result<()> {
    let mut search = Search {
        process,
        found: null_mut(),
    };
    unsafe {
        EnumWindows(Some(find_window), &mut search as *mut Search as LPARAM);
        if !search.found.is_null() {
            if IsIconic(search.found) != 0 {
                ShowWindowAsync(search.found, SW_RESTORE);
            }
            if SetForegroundWindow(search.found) == 0 {
                return Err(io::Error::other("Windows declined foreground activation"));
            }
            return Ok(());
        }
    }
    launch(exe, args, None)
}

fn perform(action: Action) -> io::Result<()> {
    match action {
        Action::Pipe(path, bytes) => {
            // One local connection per command. Never replay after a partial write.
            let mut pipe = fs::OpenOptions::new().write(true).open(path)?;
            pipe.write_all(&bytes)?;
            Ok(())
        }
        Action::Process(exe, args, cwd) => launch(&exe, &args, cwd.as_deref()),
        Action::Window(process, exe, args) => focus_or_launch(&process, &exe, &args),
    }
}

fn install(old: &Bindings, new: &Bindings) -> Result<(), String> {
    let mut added = Vec::new();
    for &id in new.keys().filter(|id| !old.contains_key(id)) {
        let modifier = match id {
            0..=11 => 0,
            12..=23 => MOD_CONTROL | MOD_SHIFT,
            _ => MOD_CONTROL | MOD_ALT,
        };
        let key = VK_F13 as u32 + id as u32 % 12;
        if unsafe { RegisterHotKey(null_mut(), id, modifier | MOD_NOREPEAT, key) } == 0 {
            let e = io::Error::last_os_error();
            for registered in added {
                unsafe {
                    UnregisterHotKey(null_mut(), registered);
                }
            }
            return Err(format!("cannot register {}: {e}", new[&id].name));
        }
        added.push(id);
    }
    for &id in old.keys().filter(|id| !new.contains_key(id)) {
        unsafe {
            UnregisterHotKey(null_mut(), id);
        }
    }
    Ok(())
}

fn run(path: PathBuf) -> Result<(), String> {
    let mut source = fs::read_to_string(&path).map_err(|e| e.to_string())?;
    let mut bindings = config::compile(&source)?;
    install(&Bindings::new(), &bindings)?;
    let mut workers = std::collections::HashMap::new();
    let timer = unsafe { SetTimer(null_mut(), 0, 1_000, None) };
    if timer == 0 {
        return Err(io::Error::last_os_error().to_string());
    }
    eprintln!("Listening on {} configured slots", bindings.len());
    let mut msg: MSG = unsafe { std::mem::zeroed() };
    loop {
        match unsafe { GetMessageW(&mut msg, null_mut(), 0, 0) } {
            -1 => return Err(io::Error::last_os_error().to_string()),
            0 => break,
            _ => {}
        }
        if msg.message == WM_HOTKEY {
            if let Some(binding) = bindings.get(&(msg.wParam as i32)) {
                let key = match &binding.action {
                    Action::Pipe(path, _) => path.clone(),
                    _ => String::new(),
                };
                let send = workers.entry(key).or_insert_with(|| {
                    let (send, receive) = mpsc::sync_channel::<(String, Action)>(64);
                    std::thread::spawn(move || {
                        for (name, action) in receive {
                            if let Err(e) = perform(action) {
                                eprintln!("{name}: {e}");
                            }
                        }
                    });
                    send
                });
                if let Err(e) = send.try_send((binding.name.clone(), binding.action.clone())) {
                    eprintln!("dispatch failed: {e}");
                }
            }
        } else if msg.message == WM_TIMER && msg.wParam == timer {
            if let Ok(next) = fs::read_to_string(&path) {
                if next != source {
                    source = next;
                    match config::compile(&source).and_then(|new| {
                        install(&bindings, &new)?;
                        Ok(new)
                    }) {
                        Ok(new) => {
                            bindings = new;
                            eprintln!("Reloaded {} bindings", bindings.len());
                        }
                        Err(e) => eprintln!("Keeping previous bindings: {e}"),
                    }
                }
            }
        }
    }
    Ok(())
}
fn main() {
    let args: Vec<_> = env::args_os().skip(1).collect();
    let result = match args.as_slice() {
        [cmd, path] if cmd == "run" => run(path.into()),
        [cmd, path] if cmd == "validate" => fs::read_to_string(path)
            .map_err(|e| e.to_string())
            .and_then(|s| config::compile(&s))
            .map(|b| println!("Valid: {} bound slots", b.len())),
        _ => Err("usage: glove80-listener <run|validate> <config.toml>".into()),
    };
    if let Err(e) = result {
        eprintln!("{e}");
        std::process::exit(1);
    }
}
