

$ErrorActionPreference = 'Stop'


# nsv home
$NSV_HOME= $ENV:NSV_HOME
if($null -eq $NSV_HOME) {
    $NSV_HOME = $pwd
}

# log file dir
$log_file = "$NSV_HOME\install.log"


if(!(Test-Path -Path $NSV_HOME)) {
    New-Item -ItemType Directory -Path $NSV_HOME -Force
}


# nsv profile
$NSV_PROFILE_PS1 = (Split-Path -Path $PROFILE) + "\nsv_profile.ps1"
$NSV_PROFILE_BAT = (Split-Path -Path $PROFILE) + "\nsv_profile.bat"


function Set-Profile-Content {
    # profile short circuit import nsv profile
    if(!(Test-Path -Path $PROFILE)) {
        New-Item -ItemType File -Path $PROFILE -Force | Out-File -FilePath $log_file -Append
    }
    $ps_content = '
    if(Test-Path -Path $Env:NSV_PROFILE_PS1) {
        . $Env:NSV_PROFILE_PS1
    }
    '
    Add-Content -Path $PROFILE -Value  $ps_content
    Add-Content -Path $log_file -Value "add powershell profile content"
    Add-Content -Path $log_file -Value $ps_content

    # nsv_profile set dynamic environment variables
    if(!(Test-Path -Path $NSV_PROFILE_PS1)) {
        New-Item -ItemType File -Path $NSV_PROFILE_PS1 -Force | Out-File -FilePath $log_file -Append
    }
    $nsv_ps1_profile_content = @(
        '$timestamp=Get-Date -UFormat %s'
        '$Env:NSV_MATEFILE="$Env:NSV_HOME\temp\$timestamp"'
        '$Env:Path="$Env:NSV_MATEFILE;$Env:NSV_HOME\temp\default;$Env:NSV_HOME;$Env:Path"'
        "nsv adapt"
    )
    Add-Content -Path $NSV_PROFILE_PS1 -Value  $nsv_ps1_profile_content
    Add-Content -Path $log_file -Value "add nsv profile content"
    Add-Content -Path $log_file -Value $nsv_ps1_profile_content

    # Administrator use set command profile
    if ($is_admin) {
        if(!(Test-Path -Path $NSV_PROFILE_BAT)) {
            New-Item -ItemType File -Path $NSV_PROFILE_BAT -Force | Out-File -FilePath $log_file -Append
        }
        $nsv_bat_profile_content = @(
            'set timestamp=%date:~10,4%%date:~4,2%%date:~7,2%%time:~0,2%%time:~3,2%%time:~6,2%'
            'set NSV_MATEFILE=%NSV_HOME%\temp%timestamp%'
        )
        Add-Content -Path $NSV_PROFILE_BAT -Value $nsv_bat_profile_content
        Add-Content -Path $log_file -Value "add nsv bat profile content"
        Add-Content -Path $log_file -Value $nsv_bat_profile_content
        Set-ItemProperty -Path "HKLM:\SOFTWARE\Microsoft\Command Processor" -Name "nsv_bat_profile" -Value $NSV_PROFILE_BAT
        Add-Content -Path $log_file -Value "set HKLM:\SOFTWARE\Microsoft\Command Processor"
        Add-Content -Path $log_file -Value $NSV_PROFILE_BAT
    }
}

function Set-EnvironmentVariable {
    param(
        [string] $Name,
        [string] $Value,
        [string] $type = "String",
        [bool] $Append = $false
    )

    if ($Append) {
        $existingValue =(Get-Item -Path HKCU:\Environment).GetValue(
            "Path",  # the registry-value name
            $null,   # the default value to return if no such value exists.
            [Microsoft.Win32.RegistryValueOptions]::DoNotExpandEnvironmentNames # the option that suppresses expansion
        )
        if ($existingValue) {
            $Value = "$Value;$existingValue"
        }
    }


    Set-ItemProperty -Path "HKCU:\Environment" -Name $Name -Value $Value -Type $type
    Add-Content -Path $log_file -Value "HKCU:\Environment $Name $Type"
    Add-Content -Path $log_file -Value $Value

}







function Download-File($url, $out_put) {

    Add-Content -Path $log_file -Value "download file $url $out_put"
    $proxyStatus = Get-ItemProperty -Path 'HKCU:\Software\Microsoft\Windows\CurrentVersion\Internet Settings' -Name ProxyEnable -ErrorAction SilentlyContinue | Select-Object -ExpandProperty ProxyEnable
    if ($proxyStatus) {
        $proxySettings = Get-ItemProperty -Path "Registry::HKEY_CURRENT_USER\Software\Microsoft\Windows\CurrentVersion\Internet Settings" -Name ProxyServer -ErrorAction SilentlyContinue
        write-Output "use proxy with--> $($proxySettings.ProxyServer)"
        $proxy_serve = "http://$($proxySettings.ProxyServer)"
        Add-Content -Path $log_file -Value "download file set proxy $proxy_serve"
        Invoke-WebRequest $url -OutFile $out_put -Proxy $proxy_serve
        return
    }
    Invoke-WebRequest $url -OutFile $out_put
}


function Get-Nsv-File-Name {

    $ProcessorArchitecture = switch ($env:PROCESSOR_ARCHITECTURE) {
        "AMD64" { "x64" }
        "x86"   { "x86" }
        "ARM64" { "arm" }
    }
    return "nsv-$ProcessorArchitecture-win.exe"
}

# download nsv binary
function Download-Nsv-Binary {
    $NSV_DOWNLOAD_URL = "https://github.com/1739616529/nsv/releases/download/v0.0.1/" + (Get-Nsv-File-Name)
    $NSV_BINARY_PATH = "$NSV_HOME\nsv.exe"
    Download-File $NSV_DOWNLOAD_URL $NSV_BINARY_PATH
}


Download-Nsv-Binary
# set profile
Set-Profile-Content

# add to user environment variables
# with NSV_PROFILE, NSV_HOME
Set-EnvironmentVariable -Name 'NSV_PROFILE_PS1' -Value $NSV_PROFILE_PS1
Set-EnvironmentVariable -Name 'NSV_HOME' -Value $NSV_HOME



Write-Host "✨✨✨"
Write-Host ""
Write-Host -ForegroundColor Green "    nsv install success."
Write-Host -ForegroundColor Blue "    Please reload the user environment variables..."
Write-Host ""
Write-Host "✨✨✨"


Add-Content -Path $log_file -Value "nsv install success."
