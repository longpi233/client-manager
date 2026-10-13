#requires -Version 7.0
[CmdletBinding()]
param(
    [string]$OutputDirectory
)

$ErrorActionPreference = 'Stop'
$PSNativeCommandUseErrorActionPreference = $false
$root = Split-Path -Parent $PSScriptRoot
$startedAt = Get-Date

function Invoke-LoggedCommand {
    param([string]$Executable, [string[]]$Arguments, [string]$LogPath)
    Write-Host "> $Executable $($Arguments -join ' ')"
    & $Executable @Arguments 2>&1 | Tee-Object -FilePath $LogPath | Out-Host
    if ($LASTEXITCODE -ne 0) {
        throw "$Executable failed with exit code $LASTEXITCODE. See $LogPath"
    }
}

Push-Location $root
try {
    if (-not $IsWindows -or [System.Runtime.InteropServices.RuntimeInformation]::OSArchitecture -ne 'X64') {
        throw 'This script builds Windows x64 packages. Run it on a Windows x64 host.'
    }

    $config = Get-Content -LiteralPath 'src-tauri/tauri.conf.json' -Raw | ConvertFrom-Json
    $package = Get-Content -LiteralPath 'package.json' -Raw | ConvertFrom-Json
    $version = $config.version
    $cargo = Get-Content -LiteralPath 'src-tauri/Cargo.toml' -Raw
    $cargoVersion = [regex]::Match($cargo, '(?m)^version\s*=\s*"([^"]+)"').Groups[1].Value
    if ($version -ne $package.version -or $version -ne $cargoVersion) {
        throw 'Versions in package.json, Cargo.toml and tauri.conf.json must match.'
    }
    $rustInfo = & rustc -vV
    if ($LASTEXITCODE -ne 0 -or $rustInfo -notcontains 'host: x86_64-pc-windows-msvc') {
        throw 'The x86_64-pc-windows-msvc Rust toolchain is required.'
    }

    if (-not $OutputDirectory) { $OutputDirectory = "artifacts/release-$version" }
    $output = [System.IO.Path]::GetFullPath($OutputDirectory, $root)
    $logs = Join-Path $output 'logs'
    New-Item -ItemType Directory -Path $logs -Force | Out-Null
    $nodeVersion = & node --version
    $pnpmVersion = (& pnpm --version | Select-Object -Last 1)
    $cargoToolVersion = & cargo --version
    $commit = & git rev-parse HEAD
    $dirty = [bool](& git status --porcelain --untracked-files=normal)

    Invoke-LoggedCommand -Executable 'pnpm' -Arguments @('build') -LogPath (Join-Path $logs 'frontend-build.log')
    # The frontend was already built above. Only skip that duplicated hook, not type checking.
    $override = '{"build":{"beforeBuildCommand":""}}'
    Invoke-LoggedCommand -Executable 'node' -Arguments @(
        'node_modules/@tauri-apps/cli/tauri.js', 'build', '--bundles', 'nsis,msi',
        '--ci', '--verbose', '--config', $override, '--', '--locked'
    ) -LogPath (Join-Path $logs 'tauri-build.log')

    $metadataJson = & cargo metadata --manifest-path src-tauri/Cargo.toml --no-deps --offline --locked --format-version 1
    if ($LASTEXITCODE -ne 0) { throw 'Unable to read the Cargo target directory.' }
    $targetDirectory = ($metadataJson | ConvertFrom-Json).target_directory
    $releaseDirectory = Join-Path $targetDirectory 'release'
    $assets = @()
    foreach ($bundle in @(@{ type='nsis'; extension='.exe'; suffix='_setup.exe' }, @{ type='msi'; extension='.msi'; suffix='.msi' })) {
        $folder = Join-Path $releaseDirectory "bundle/$($bundle.type)"
        $source = Get-ChildItem -LiteralPath $folder -File | Where-Object {
            $_.Extension -eq $bundle.extension -and $_.Name.Contains($version) -and $_.LastWriteTime -ge $startedAt
        } | Sort-Object LastWriteTime -Descending | Select-Object -First 1
        if (-not $source) { throw "No newly built $($bundle.type) package was found in $folder." }
        $destination = Join-Path $output "client-manager_${version}_windows-x64$($bundle.suffix)"
        Copy-Item -LiteralPath $source.FullName -Destination $destination -Force
        $assets += $destination
    }

    $binaryName = [regex]::Match($cargo, '(?m)^name\s*=\s*"([^"]+)"').Groups[1].Value
    $standalone = Join-Path $output 'standalone'
    New-Item -ItemType Directory -Path $standalone -Force | Out-Null
    Copy-Item -LiteralPath (Join-Path $releaseDirectory "$binaryName.exe") -Destination $standalone -Force
    Copy-Item -LiteralPath (Join-Path $root 'LICENSE') -Destination $standalone -Force
    $instructions = @"
Client Manager $version - Windows x64

Extract all files into one directory and run $binaryName.exe.
Microsoft Edge WebView2 Runtime is required.
Data is stored in the system application data directory, not next to this executable.
Keep LICENSE with any redistributed copy. Upstream attribution is recorded in the project README.
This build is unsigned. It has not been installed or tested on a clean Windows machine.
"@
    $instructions | Set-Content -LiteralPath (Join-Path $standalone 'RUNNING.txt') -Encoding utf8NoBOM
    $zip = Join-Path $output "client-manager_${version}_windows-x64.zip"
    $zipFiles = @("$binaryName.exe", 'LICENSE', 'RUNNING.txt') | ForEach-Object { Join-Path $standalone $_ }
    Compress-Archive -LiteralPath $zipFiles -DestinationPath $zip -CompressionLevel Optimal -Force
    $assets += $zip
    Copy-Item -LiteralPath (Join-Path $root 'LICENSE') -Destination $output -Force

    $hashes = foreach ($asset in $assets) {
        $item = Get-Item -LiteralPath $asset
        [ordered]@{ file=$item.Name; bytes=$item.Length; sha256=(Get-FileHash -LiteralPath $asset -Algorithm SHA256).Hash.ToLowerInvariant() }
    }
    $hashes | ForEach-Object { "$($_.sha256)  $($_.file)" } |
        Set-Content -LiteralPath (Join-Path $output 'SHA256SUMS.txt') -Encoding utf8NoBOM
    [ordered]@{
        version=$version
        platform='windows-x64'
        startedAt=$startedAt.ToString('o')
        completedAt=(Get-Date).ToString('o')
        sourceCommit=$commit
        workingTreeDirty=$dirty
        node=$nodeVersion
        pnpm=$pnpmVersion
        cargo=$cargoToolVersion
        rust=$rustInfo
        signed=$false
        cleanMachineTested=$false
        assets=@($hashes)
    } | ConvertTo-Json -Depth 6 | Set-Content -LiteralPath (Join-Path $output 'build-info.json') -Encoding utf8NoBOM

    $logArchive = Join-Path $output "client-manager_${version}_windows-x64_build-logs.zip"
    $logFiles = @('frontend-build.log', 'tauri-build.log') | ForEach-Object { Join-Path $logs $_ }
    $logFiles += Join-Path $output 'build-info.json'
    Compress-Archive -LiteralPath $logFiles -DestinationPath $logArchive -CompressionLevel Optimal -Force

    Write-Host "Release packages, checksums and build logs are ready in $output"
} finally {
    Pop-Location
}
