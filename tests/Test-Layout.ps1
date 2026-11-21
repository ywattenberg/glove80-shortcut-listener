param([string]$Layout = "$PSScriptRoot/../fixtures/layout.json")
$ErrorActionPreference = 'Stop'
$bindings = Get-Content -LiteralPath $Layout -Raw | ConvertFrom-Json
$slots = @{}
$chords = @{}
foreach ($binding in $bindings) {
    if ($binding.slot -cnotmatch '^[A-Z]$' -or $slots.ContainsKey($binding.slot)) {
        throw "Invalid or duplicate slot '$($binding.slot)'"
    }
    if ($binding.chord -cnotmatch '^(Ctrl\+(Shift|Alt)\+)?F(1[3-9]|2[0-4])$' -or $chords.ContainsKey($binding.chord)) {
        throw "Invalid or duplicate chord '$($binding.chord)'"
    }
    $slots[$binding.slot] = $true
    $chords[$binding.chord] = $true
}
if ($slots.Count -ne 26) { throw "Expected 26 distinct letter slots, found $($slots.Count)" }
"PASS: 26 unique slots and chords"

