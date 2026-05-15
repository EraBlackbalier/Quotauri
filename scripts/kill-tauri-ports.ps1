$ports = @(18999, 19000)

foreach ($port in $ports) {
  $listeners = Get-NetTCPConnection -LocalPort $port -State Listen -ErrorAction SilentlyContinue
  if (-not $listeners) { continue }

  foreach ($listener in $listeners) {
    $ownerPid = $listener.OwningProcess
    if (-not $ownerPid -or $ownerPid -eq $PID) { continue }

    Write-Host "Releasing port $port (PID $ownerPid)"
    Stop-Process -Id $ownerPid -Force -ErrorAction SilentlyContinue
  }
}
