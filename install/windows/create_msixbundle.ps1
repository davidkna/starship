#!/usr/bin/env pwsh

Set-StrictMode -Version 3.0

$env:Path = "C:\Program Files (x86)\Windows Kits\10\bin\10.0.22621.0\x64;$env:Path"

New-Item -ItemType "directory" -Force .\target
New-Item -ItemType "directory" -Force .\target\msix_files

$version = $null

foreach ($starship_zip in (Get-ChildItem starship-*/starship-*-pc-windows-msvc.zip)) {
    # Determine the architecture
    $starship_zip_name = $starship_zip.Name
    if ($starship_zip_name.Contains("aarch64")) {
        $arch = "arm64"
    } elseif ($starship_zip_name.Contains("x86_64")) {
        $arch = "x64"
    } elseif ($starship_zip_name.Contains("i686")) {
        $arch = "x86"
    } else {
        $arch = "neutral"
    }

    # Extract the starship zip into target/release for easier testing
    New-Item -ItemType "directory" -Force .\target\release
    Expand-Archive -Force -Path $starship_zip -DestinationPath .\target\release

    if ($null -eq $version) {
        $version = (Get-Item ".\target\release\starship.exe").VersionInfo.FileVersion
        $dot_count = ($version | Select-String -pattern '\.' -AllMatches).Matches.Count

        for ($i = $dot_count; $i -lt 3; $i++) {
            $version = $version + ".0"
        }
    }

    # Create arch-specific appx manifest & insert version
    (Get-Content -Raw .\install\windows\Package.appxmanifest) -replace "neutral", $arch -replace "0.0.0.1", $version |
    Set-Content .\Package.$arch.appxmanifest
    
    # Bundle into an msix file
    makeappx pack /o /v /m .\Package.$arch.appxmanifest /f .\install\windows\mapping.ini /p ".\target\msix_files\starship-$arch.msix"

    Remove-Item -Force .\Package.$arch.appxmanifest
}

# Bundle msix files into a single msixbundle
makeappx bundle /o /v /bv $version /d .\target\msix_files /p .\target\starship.msixbundle

# Clean up
Remove-Item -Recurse -Force .\target\msix_files