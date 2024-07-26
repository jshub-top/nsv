
# nsv home
$NSV_HOME= $ENV:NSV_HOME
if($null -eq $NSV_HOME) {
    $NSV_HOME = $PSScriptRoot
}

# nsv defualt node dir matefile
# $NSV_DEFAULT_MATEFILE = "$NSV_HOME\temp\default"

# log file dir
$log_file = "$NSV_HOME\install.log"

# nsv profile
$NSV_PROFILE_PS1 = (Split-Path -Path $PROFILE) + "\nsv_profile.ps1"
$NSV_PROFILE_BAT = (Split-Path -Path $PROFILE) + "\nsv_profile.bat"


function Set-Profile-Content {
    # powershell profile
    if(!(Test-Path -Path $PROFILE)) {
        New-Item -ItemType File -Path $PROFILE -Force > $log_file
    }

    if(!(Test-Path -Path $NSV_PROFILE_PS1)) {
        New-Item -ItemType File -Path $NSV_PROFILE_PS1 -Force > $log_file
    }

    if(!(Test-Path -Path $NSV_PROFILE_BAT)) {
        New-Item -ItemType File -Path $NSV_PROFILE_BAT -Force > $log_file
    }
    # profile short circuit import nsv profile
    $ps_content = '
    if(Test-Path -Path $Env:NSV_PROFILE_PS1) {
        . $Env:NSV_PROFILE_PS1
    }
    '
    Add-Content -Path $PROFILE -Value  $ps_content

    # nsv_profile set dynamic environment variables
    $nsv_ps1_profile_content = @(
        '$timestamp=Get-Date -UFormat %s'
        '$Env:NSV_MATEFILE='+'"'+'$NSV_HOME\temp\'+'$timestamp'+'"'
        "nsv adapt"
    )
    Add-Content -Path $NSV_PROFILE_PS1 -Value  $nsv_ps1_profile_content

    $nsv_bat_profile_content = @(
        'set timestamp=%date:~10,4%%date:~4,2%%date:~7,2%%time:~0,2%%time:~3,2%%time:~6,2%'
        'set NSV_MATEFILE=%NSV_HOME%\temp%timestamp%'
    )
    Add-Content -Path $NSV_PROFILE_BAT -Value  $nsv_bat_profile_content

    Set-ItemProperty -Path "HKLM:\SOFTWARE\Microsoft\Command Processor" -Name "nsv_bat_profile" -Value $NSV_PROFILE_BAT
}

function Set-EnvironmentVariable {
    param(
        [string] $Name,
        [string] $Value,
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


    Set-ItemProperty -Path "HKCU:\Environment" -Name $Name -Value $Value -Type "ExpandString"

}


Set-Profile-Content


# add to user environment variables
# # NSV_PROFILE NSV_HOME NSV_DEFAULT_MATEFILE PATH

# Set-EnvironmentVariable -Name 'NSV_PROFILE_PS1' -Value $NSV_PROFILE_PS1
# Set-EnvironmentVariable -Name 'NSV_HOME' -Value $NSV_HOME
# Set-EnvironmentVariable -Name 'NSV_DEFAULT_MATEFILE' -Value '%NSV_HOME%\temp\default'
# Set-EnvironmentVariable -Name 'Path' -Value '%NSV_MATEFILE%;%NSV_DEFAULT_MATEFILE%;%NSV_HOME%' -Append $true


function Download-File($url, $out_put) {
    $proxyStatus = Get-ItemProperty -Path 'HKCU:\Software\Microsoft\Windows\CurrentVersion\Internet Settings' -Name ProxyEnable -ErrorAction SilentlyContinue | Select-Object -ExpandProperty ProxyEnable
    if ($proxyStatus) {
        $proxySettings = Get-ItemProperty -Path "Registry::HKEY_CURRENT_USER\Software\Microsoft\Windows\CurrentVersion\Internet Settings" -Name ProxyServer -ErrorAction SilentlyContinue
        write-Output "nsv: use proxy with--> $($proxySettings.ProxyServer)"
        $proxy_serve = "http://$($proxySettings.ProxyServer)"
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

# Download-Nsv-Binary
