$ErrorActionPreference = 'Stop'
$exe = (Resolve-Path "$PSScriptRoot/../target/release/glove80-listener.exe").Path
$dir = Join-Path ([IO.Path]::GetTempPath()) ("glove80-" + [guid]::NewGuid())
[void][IO.Directory]::CreateDirectory($dir)
$config = Join-Path $dir config.toml
$log = Join-Path $dir listener.log
$marker = Join-Path $dir ran.txt
$name = "glove80-" + [guid]::NewGuid()
$pipe = [IO.Pipes.NamedPipeServerStream]::new($name, 'In', 1, 'Byte', 'Asynchronous')
function Wait-Log([string]$pattern) {
    $timer = [Diagnostics.Stopwatch]::StartNew()
    do {
        if ($listener.HasExited) { throw (Get-Content $log -Raw) }
        if ((Get-Content $log -Raw) -match $pattern) { return }
        Start-Sleep -Milliseconds 50
    } while ($timer.Elapsed.TotalSeconds -lt 8)
    throw "Timed out waiting for $pattern"
}
try {
    $source = @"
version = 1
[layers.special]
A = "send"
[apps.test]
pipe = "$name"
[actions.send]
label = "Send test"
type = "app.command"
app = "test"
command = "increment_counter"
args = { amount = 3 }
[actions.mark]
label = "Test process"
type = "process.run"
exe = "powershell.exe"
args = ["-NoProfile", "-Command", 'Set-Content -LiteralPath "$marker" -Value passed']
"@
    Set-Content $config $source
    $listener = Start-Process $exe -ArgumentList @('run', ('"' + $config + '"')) -WindowStyle Hidden -PassThru -RedirectStandardError $log
    Wait-Log 'Listening'
    Set-Content $config 'broken = ['
    Wait-Log 'Keeping previous bindings'
    $connected = $pipe.WaitForConnectionAsync()
    & "$PSScriptRoot/Test-WindowsHotkey.ps1" -SendOnly
    if (-not $connected.Wait(5000)) { throw 'Missing pipe connection' }
    $reader = [IO.StreamReader]::new($pipe)
    $read = $reader.ReadLineAsync()
    if (-not $read.Wait(5000)) { throw 'Missing command' }
    $message = $read.Result | ConvertFrom-Json
    if ($message.command -ne 'increment_counter' -or $message.args.amount -ne 3) { throw 'Incorrect command' }
    Set-Content $config ($source.Replace('A = "send"', 'A = "mark"'))
    Wait-Log 'Reloaded'
    & "$PSScriptRoot/Test-WindowsHotkey.ps1" -SendOnly
    $timer = [Diagnostics.Stopwatch]::StartNew()
    while (!(Test-Path $marker) -and $timer.Elapsed.TotalSeconds -lt 8) { Start-Sleep -Milliseconds 50 }
    if ((Get-Content $marker) -ne 'passed') { throw 'Missing process action' }
    'PASS: Rust listener delivered configured JSON, retained last valid config, reloaded, and launched configured script.'
} finally {
    if ($listener -and !$listener.HasExited) { $listener.Kill(); $listener.WaitForExit() }
    $pipe.Dispose()
    foreach ($file in @($marker,$config,$log)) {
        if (Test-Path -LiteralPath $file) { Remove-Item -LiteralPath $file }
    }
    [IO.Directory]::Delete($dir)
}
