$ErrorActionPreference = "Stop"

$scriptArguments = @($args)
if ($scriptArguments.Count -gt 1 -or ($scriptArguments.Count -eq 1 -and $scriptArguments[0] -cne "update")) {
    throw "Usage: .\install.ps1 [update]"
}
$mode = if ($scriptArguments.Count -eq 1) { "update" } else { "install" }

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

$installedBinaryPath = $null
if ($env:INSTALL_DIR) {
    $installDir = [System.IO.Path]::GetFullPath($env:INSTALL_DIR)
    if ($mode -eq "update") {
        $installedBinaryPath = Join-Path $installDir "midway.exe"
    }
} elseif ($mode -eq "update") {
    $installedCommand = Get-Command midway -CommandType Application -ErrorAction SilentlyContinue
    if (-not $installedCommand) {
        throw "Midway is not installed. Run the installer without 'update' to install it first."
    }
    $installedBinaryPath = $installedCommand.Source
    $installDir = Split-Path -Parent $installedBinaryPath
} elseif ($env:LOCALAPPDATA) {
    $installDir = Join-Path $env:LOCALAPPDATA "Programs\Midway"
} else {
    $installDir = Join-Path $HOME ".local\bin"
}
$installDir = [System.IO.Path]::GetFullPath($installDir)
$asset = "midway-$target.zip"
$downloadUrl = "https://github.com/$repository/releases/latest/download/$asset"
$tempDir = Join-Path ([System.IO.Path]::GetTempPath()) ([guid]::NewGuid().ToString())

try {
    if ($mode -eq "update") {
        if (-not (Test-Path -LiteralPath $installedBinaryPath -PathType Leaf)) {
            throw "Midway was not found at $installedBinaryPath."
        }
        Write-Output "Checking for Midway updates..."
        $currentVersion = (& $installedBinaryPath --version | Out-String).Trim()
        if ($LASTEXITCODE -ne 0 -or -not $currentVersion) {
            throw "Could not read the installed Midway version from $installedBinaryPath."
        }
    }

    New-Item -ItemType Directory -Path $tempDir | Out-Null
    $archivePath = Join-Path $tempDir $asset
    Invoke-WebRequest -Uri $downloadUrl -OutFile $archivePath -UseBasicParsing
    Expand-Archive -LiteralPath $archivePath -DestinationPath $tempDir

    $binaryPath = Join-Path $tempDir "midway.exe"
    if (-not (Test-Path -LiteralPath $binaryPath -PathType Leaf)) {
        throw "The release archive must contain a top-level midway.exe."
    }

    $releaseVersion = (& $binaryPath --version | Out-String).Trim()
    if ($LASTEXITCODE -ne 0 -or -not $releaseVersion) {
        throw "Could not read the Midway version from the downloaded release."
    }

    if ($mode -eq "update" -and $currentVersion -ceq $releaseVersion) {
        Write-Output "Midway is up to date ($currentVersion). Exiting."
        return
    }

    if ($mode -eq "update") {
        Write-Output "Updating Midway from $currentVersion to $releaseVersion..."
    } else {
        Write-Output "Installing Midway $releaseVersion..."
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

    if ($mode -eq "update") {
        Write-Output "Updated midway to $(Join-Path $installDir 'midway.exe')"
    } else {
        Write-Output "Installed midway to $(Join-Path $installDir 'midway.exe')"
        Write-Output "Run 'midway --version' to verify the installation."
    }
} catch {
    throw "Could not install $asset. Ensure a matching asset exists in the latest release. $($_.Exception.Message)"
} finally {
    Remove-Item -LiteralPath $tempDir -Recurse -Force -ErrorAction SilentlyContinue
}
