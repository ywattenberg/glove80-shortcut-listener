$ErrorActionPreference = 'Stop'
$pipeName = "glove80-test-$([Guid]::NewGuid())"
$binding = (Get-Content "$PSScriptRoot/../fixtures/custom-command.json" -Raw | ConvertFrom-Json).F13
# This job stands in for an app that is already running when the key is pressed.
$app = Start-Job -ArgumentList $pipeName -ScriptBlock {
    param($name)
    $ErrorActionPreference = 'Stop'
    $server = [IO.Pipes.NamedPipeServerStream]::new($name)
    try {
        $counter = 0
        $server.WaitForConnection()
        $reader = [IO.StreamReader]::new($server)
        $writer = [IO.StreamWriter]::new($server)
        $writer.AutoFlush = $true
        $message = $reader.ReadLine() | ConvertFrom-Json
        if ($message.command -cne 'increment_counter') { throw 'Unexpected command' }
        $counter += [int]$message.args.amount
        $writer.WriteLine((@{ counter = $counter } | ConvertTo-Json -Compress))
    } finally { $server.Dispose() }
}
$client = [IO.Pipes.NamedPipeClientStream]::new('.', $pipeName)
try {
    $client.Connect(10000)
    & "$PSScriptRoot/Test-WindowsHotkey.ps1" -Emulate
    $writer = [IO.StreamWriter]::new($client)
    $writer.AutoFlush = $true
    $writer.WriteLine(($binding | ConvertTo-Json -Compress))
    $reader = [IO.StreamReader]::new($client)
    $reply = $reader.ReadLineAsync()
    if (-not $reply.Wait(5000)) { throw 'App did not acknowledge the action' }
    if (($reply.Result | ConvertFrom-Json).counter -ne 3) { throw 'Unexpected app state' }
    "PASS: running app handled increment_counter and returned counter=3"
} finally {
    $client.Dispose()
    Remove-Job $app -Force
}
