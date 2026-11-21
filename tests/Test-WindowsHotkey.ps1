param([switch]$WaitForPress, [switch]$Emulate, [switch]$SendOnly)
$ErrorActionPreference = 'Stop'
if (-not ('ShortcutProbe' -as [type])) {
    Add-Type @"
using System;
using System.ComponentModel;
using System.Runtime.InteropServices;
using System.Threading;
public static class ShortcutProbe {
    [StructLayout(LayoutKind.Sequential)]
    struct Message {
        public IntPtr window;
        public uint message;
        public UIntPtr wParam;
        public IntPtr lParam;
        public uint time;
        public int x, y;
        public uint privateData;
    }
    [DllImport("user32.dll", SetLastError=true)]
    static extern bool RegisterHotKey(IntPtr window, int id, uint modifiers, uint key);
    [DllImport("user32.dll", SetLastError=true)]
    static extern bool UnregisterHotKey(IntPtr window, int id);
    [DllImport("user32.dll", EntryPoint="PeekMessageW")]
    static extern bool PeekMessage(out Message msg, IntPtr window, uint min, uint max, uint remove);
    [StructLayout(LayoutKind.Sequential)]
    struct KeyboardInput {
        public ushort key, scan;
        public uint flags, time;
        public UIntPtr extra;
    }
    [StructLayout(LayoutKind.Sequential)]
    struct MouseInput {
        public int x, y;
        public uint data, flags, time;
        public UIntPtr extra;
    }
    [StructLayout(LayoutKind.Explicit)]
    struct InputData {
        [FieldOffset(0)] public KeyboardInput keyboard;
        [FieldOffset(0)] public MouseInput mouse;
    }
    [StructLayout(LayoutKind.Sequential)]
    struct Input {
        public uint type;
        public InputData data;
    }
    [DllImport("user32.dll", SetLastError=true)]
    static extern uint SendInput(uint count, Input[] inputs, int size);
    public static void SendF13() {
        var down = new Input { type = 1 };
        down.data.keyboard.key = 0x7C;
        var up = down;
        up.data.keyboard.flags = 2;
        int size = Marshal.SizeOf(typeof(Input));
        uint sent = SendInput(2, new[] { down, up }, size);
        if (sent != 2) {
            int error = Marshal.GetLastWin32Error();
            if (sent == 1) SendInput(1, new[] { up }, size);
            throw new Win32Exception(error, "Incomplete F13 injection");
        }
    }
    public static void Run(bool wait, bool emulate) {
        const int id = 314;
        const uint hotkeyMessage = 0x0312;
        if (!RegisterHotKey(IntPtr.Zero, id, 0x4000, 0x7C))
            throw new Win32Exception(Marshal.GetLastWin32Error(), "Could not reserve F13");
        try {
            if (!wait && !emulate) return;
            if (emulate) SendF13();
            else
            Console.WriteLine("Press F13 within 20 seconds. You can focus another app.");
            var timer = System.Diagnostics.Stopwatch.StartNew();
            while (timer.ElapsedMilliseconds < 20000) {
                Message msg;
                if (PeekMessage(out msg, IntPtr.Zero, hotkeyMessage, hotkeyMessage, 1)
                    && msg.wParam.ToUInt64() == id) return;
                Thread.Sleep(10);
            }
            throw new TimeoutException("No F13 press received");
        } finally {
            if (!UnregisterHotKey(IntPtr.Zero, id))
                throw new Win32Exception(Marshal.GetLastWin32Error(), "Could not release F13");
        }
    }
}
"@
}
if ($SendOnly) { [ShortcutProbe]::SendF13(); return }
[ShortcutProbe]::Run($WaitForPress.IsPresent, $Emulate.IsPresent)
if ($Emulate) { "PASS: received synthetic F13 and released registration" } elseif ($WaitForPress) { "PASS: received F13 and released it" } else { "PASS: reserved and released F13" }


