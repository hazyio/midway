$ErrorActionPreference = "Stop"

$repository = "hazyio/midway"
$processArchitecture = if ($env:PROCESSOR_ARCHITEW6432) {
    $env:PROCESSOR_ARCHITEW6432
} else {
    $env:PROCESSOR_ARCHITECTURE
}

switch ($processArchitecture) {
    "AMD64" { $target = "x86_64-pc-windows-msvc" }
    "ARM64" { throw "No prebuilt Midway release is available for ARM64 Windows. Install Rust and Cargo, then run: cargo install midway-cli" }
    default { throw "Unsupported Windows architecture: $processArchitecture. See https://midway.hazyio.com/installation for supported builds." }
}

$installDir = if ($env:INSTALL_DIR) {
    $env:INSTALL_DIR
} elseif ($env:LOCALAPPDATA) {
    Join-Path $env:LOCALAPPDATA "Programs\Midway"
} else {
    Join-Path $HOME ".local\bin"
}
$installDir = [System.IO.Path]::GetFullPath($installDir)
$asset = "midway-$target.zip"
$downloadUrl = "https://github.com/$repository/releases/latest/download/$asset"
$tempDir = Join-Path ([System.IO.Path]::GetTempPath()) ([guid]::NewGuid().ToString())

try {
    New-Item -ItemType Directory -Path $tempDir | Out-Null
    $archivePath = Join-Path $tempDir $asset
    Invoke-WebRequest -Uri $downloadUrl -OutFile $archivePath -UseBasicParsing
    Expand-Archive -LiteralPath $archivePath -DestinationPath $tempDir

    $binaryPath = Join-Path $tempDir "midway.exe"
    if (-not (Test-Path -LiteralPath $binaryPath -PathType Leaf)) {
        throw "The release archive must contain a top-level midway.exe."
    }

    New-Item -ItemType Directory -Path $installDir -Force | Out-Null
    Copy-Item -LiteralPath $binaryPath -Destination (Join-Path $installDir "midway.exe") -Force

    $userPath = [Environment]::GetEnvironmentVariable("Path", "User")
    $userPathEntries = @($userPath -split ";" | Where-Object { $_ })
    if ($userPathEntries -notcontains $installDir) {
        [Environment]::SetEnvironmentVariable("Path", (@($userPathEntries) + $installDir) -join ";", "User")
    }

    if (($env:Path -split ";") -notcontains $installDir) {
        $env:Path = "$installDir;$env:Path"
    }

    Write-Output "Installed midway to $(Join-Path $installDir 'midway.exe')"
    Write-Output "Run 'midway --version' to verify the installation."
} catch {
    throw "Could not install $asset. Ensure a matching asset exists in the latest release. $($_.Exception.Message)"
} finally {
    Remove-Item -LiteralPath $tempDir -Recurse -Force -ErrorAction SilentlyContinue
}
