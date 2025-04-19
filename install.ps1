$ErrorActionPreference = "Stop"

$repo = "sebastianstupak/biolens"
$installDir = "$env:LOCALAPPDATA\biolens"
$version = if ($env:VERSION) { $env:VERSION } else { "latest" }
$visualization = if ($env:VISUALIZATION) { $env:VISUALIZATION.ToLower() } else { $null }

function Write-ColorOutput {
    param(
        [string]$Text,
        [string]$Color
    )
    $originalColor = $host.UI.RawUI.ForegroundColor
    $host.UI.RawUI.ForegroundColor = $Color
    Write-Output $Text
    $host.UI.RawUI.ForegroundColor = $originalColor
}

Write-ColorOutput "┌───────────────────────────────────────┐" "Cyan"
Write-ColorOutput "│            BioLens Installer          │" "Cyan"
Write-ColorOutput "└───────────────────────────────────────┘" "Cyan"
Write-Output ""

Write-ColorOutput "System detection:" "Cyan"
Write-ColorOutput "- Operating System: Windows" "Yellow"
Write-ColorOutput "- Architecture: x86_64" "Yellow"
Write-Output ""

if ($null -eq $visualization) {
    Write-ColorOutput "BioLens is available in two versions:" "Cyan"
    Write-Output "1. Standard version (smaller, basic functionality)"
    Write-Output "2. Visualization version (larger, includes visualization features)"
    Write-Output ""
    $vizConfirm = Read-Host "Would you like to install the version with visualization features? [y/N]"

    if ($vizConfirm -eq "y" -or $vizConfirm -eq "Y") {
        $visualization = "true"
        Write-ColorOutput "Installing BioLens with visualization features." "Green"
    } else {
        $visualization = "false"
        Write-ColorOutput "Installing standard BioLens version." "Green"
    }
    Write-Output ""
}

if ($visualization -eq "true") {
    $packageSuffix = "-viz"
} else {
    $packageSuffix = ""
}

if ($version -eq "latest") {
    Write-ColorOutput "Fetching latest release information..." "Cyan"
    try {
        $releaseData = Invoke-RestMethod -Uri "https://api.github.com/repos/$repo/releases/latest"
        $packageName = "biolens$packageSuffix-windows-x86_64.zip"
        $asset = $releaseData.assets | Where-Object { $_.name -eq $packageName }
        $downloadUrl = $asset.browser_download_url
        $versionDisplay = $releaseData.tag_name

        if (-not $downloadUrl) {
            Write-ColorOutput "Could not find release '$packageName' for Windows." "Yellow"

            if ($visualization -eq "true") {
                $altPackageName = "biolens-windows-x86_64.zip"
                Write-ColorOutput "Falling back to standard version..." "Yellow"
            } else {
                $altPackageName = "biolens-viz-windows-x86_64.zip"
                Write-ColorOutput "Falling back to visualization version..." "Yellow"
            }

            $asset = $releaseData.assets | Where-Object { $_.name -eq $altPackageName }
            $downloadUrl = $asset.browser_download_url

            if (-not $downloadUrl) {
                Write-ColorOutput "Could not find any suitable release for Windows. Please check the repository." "Red"
                exit 1
            }
        }

        Write-ColorOutput "Latest version: $versionDisplay" "Yellow"
    }
    catch {
        Write-ColorOutput "Error fetching release information: $_" "Red"
        exit 1
    }
}
else {
    $packageName = "biolens$packageSuffix-windows-x86_64.zip"
    $downloadUrl = "https://github.com/$repo/releases/download/$version/$packageName"
    Write-ColorOutput "Installing version: $version" "Yellow"
}

Write-Output ""
Write-ColorOutput "BioLens will be installed to: $installDir" "Yellow"
Write-ColorOutput "It will be added to your PATH." "Yellow"
Write-Output ""
$confirmation = Read-Host "Do you want to continue? [y/N]"

if ($confirmation -ne "y" -and $confirmation -ne "Y") {
    Write-ColorOutput "Installation cancelled." "Red"
    exit 1
}

Write-Output ""
Write-ColorOutput "Installing BioLens..." "Cyan"

if (-not (Test-Path $installDir)) {
    Write-Output "Creating installation directory: $installDir"
    New-Item -ItemType Directory -Path $installDir | Out-Null
}

Write-Output "Downloading from: $downloadUrl"
$zipPath = Join-Path $env:TEMP "biolens.zip"
try {
    Invoke-WebRequest -Uri $downloadUrl -OutFile $zipPath
}
catch {
    Write-ColorOutput "Error downloading file: $_" "Red"
    exit 1
}

Write-Output "Extracting..."
try {
    Expand-Archive -Path $zipPath -DestinationPath $installDir -Force
}
catch {
    Write-ColorOutput "Error extracting zip file: $_" "Red"
    exit 1
}

$currentPath = [Environment]::GetEnvironmentVariable("Path", "User")
if ($currentPath -notlike "*$installDir*") {
    try {
        [Environment]::SetEnvironmentVariable("Path", "$currentPath;$installDir", "User")
        $env:Path = "$env:Path;$installDir"
        Write-Output "Added $installDir to your PATH"
    }
    catch {
        Write-ColorOutput "Error updating PATH: $_" "Red"
        Write-ColorOutput "You may need to manually add $installDir to your PATH" "Yellow"
    }
}

Write-Output ""
Write-ColorOutput "BioLens has been successfully installed!" "Green"
Write-ColorOutput "Run 'biolens --help' to get started" "Cyan"
Write-ColorOutput "Note: You may need to restart your terminal for the PATH changes to take effect." "Yellow"

Remove-Item $zipPath -Force
