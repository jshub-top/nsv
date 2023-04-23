


$argv = $args
$scriptDir = $PSScriptRoot
$isAdmin = ([Security.Principal.WindowsPrincipal] [Security.Principal.WindowsIdentity]::GetCurrent()).IsInRole([Security.Principal.WindowsBuiltInRole] "Administrator")
if (-not $env:NSV_HOME) {
    $env:NSV_HOME = $scriptDir
}

$EnvRegistryDir = [PSCustomObject]@{
    User = "HKCU:\Environment"
    System = "HKLM:\SYSTEM\ControlSet001\Control\Session Manager\Environment\"
}

function Get-Path-Crs ($path) {

    $local_env =  Get-Item -Path $path
    $local_env.GetValue(
        "Path",
        "",
        [Microsoft.Win32.RegistryValueOptions]::DoNotExpandEnvironmentNames
    )
}

function init() {
    if ($argv -notcontains "--init") {
        return
    }

    if (-not $isAdmin) {
        Start-Process powershell.exe "-File $PSCommandPath $argv" -Verb RunAs
        Exit 0
    }


    function Set-Path_Crs() {
        $state_num = 0

        $user_path_dir = $EnvRegistryDir.User
        $user_path_value = Get-Path-Crs $user_path_dir
        if ( $user_path_value -notmatch "%NSV_HOME%" ) {
            Set-ItemProperty -Path $user_path_dir -Name 'NSV_HOME' -Value $env:NSV_HOME -Type String
            Set-ItemProperty -Path $user_path_dir -Name 'Path' -Value "%NSV_HOME%;$user_path_value" -Type ExpandString
            $state_num += 1
        }

        $system_path_dir = $EnvRegistryDir.System
        $system_path_value = Get-Path-Crs $system_path_dir

        if ($system_path_value -notmatch "%NSV_LOCAL_NODE%") {
            Set-ItemProperty -Path $system_path_dir -Name 'NSV_LOCAL_NODE' -Value "$env:NSV_HOME\local\node" -Type String
            Set-ItemProperty -Path $system_path_dir -Name 'Path' -Value "%NSV_LOCAL_NODE%;$system_path_value" -Type ExpandString
            $state_num += 2
        }

        return $state_num
    }


    function reload_env($state) {
        if ($state -eq 0) {
            return
        }

        $Env:Path = "$($env:NSV_HOME);$($Env:Path)"

#        if ($state -eq 1) {
            $answer = Read-Host "Restart the system for the environment variables to take effect. Do you want logout user now? (Y/n)"
            $answer = $answer.ToLower()
            if (!($answer -eq "y") -and !($answer -eq "yes") -and !($answer.Length -eq 0)) {
                exit 0
            }
            Logoff
#        }

#        if ($state -gt 1) {
#            $answer = Read-Host "Restart the system for the environment variables to take effect. Do you want reboot system now? (Y/n)"
#            $answer = $answer.ToLower()
#            if (!($answer -eq "y") -and !($answer -eq "yes") -and !($answer.Length -eq 0)) {
#
#                exit 0
#            }
#            Restart-Computer -Force
#        }
    }

#    function set_open_pws_before_hooks() {
#
#        $file_dir = "C:\Users\ail\Desktop\t.js"
##        $file_dir = $PROFILE
#        if (-not (Test-Path $PROFILE)) {
#            New-Item -ItemType File -Path $PROFILE -Force
#        }
#
#        $run_script = ". $Env:NSV_HOME fast --on"
#
#        if ((Select-String -Path $file_dir -Pattern ) -eq $null) {
#            # 文件不包含 "hello"，执行相应操作
#            Write-Host "File does not contain 'hello'."
#        } else {
#            # 文件包含 "hello"，不执行任何操作
#            Write-Host "File contains 'hello'."
#        }
#
#
#    }

    $state = Set-Path_Crs
    reload_env $state
    Exit 0
}




init
#




















function download_file($url, $out_put) {
    $proxySettings = Get-ItemProperty -Path "Registry::HKEY_CURRENT_USER\Software\Microsoft\Windows\CurrentVersion\Internet Settings" -Name ProxyServer -ErrorAction SilentlyContinue
    if($proxySettings.ProxyServer) {
        $proxy_serve = "http://$($proxySettings.ProxyServer)"
        Invoke-WebRequest $url -OutFile $out_put -Proxy $proxy_serve
        return
    }
    Invoke-WebRequest $url -OutFile $out_put
}





function nsv_root_dir($dir) {
    return Join-Path $env:NSV_HOME $dir
}

$config = Get-Content -Path $(nsv_root_dir "config.json") | ConvertFrom-Json
$package = Get-Content -Path $(nsv_root_dir "package.json") | ConvertFrom-Json
$cache_path = Join-Path $env:NSV_HOME $config.path.cache
$node_path = Join-Path $env:NSV_HOME $config.path.node
$js_dir = Join-Path $env:NSV_HOME "dist/index.js"
$node_abs_dir = ""
if (-not (Test-Path $cache_path)) {
    New-Item -ItemType Directory -Force -Path $cache_path -ErrorAction:Stop > $null
}
if (-not (Test-Path $node_path)) {
    New-Item -ItemType Directory -Force -Path $node_path -ErrorAction:Stop > $null
}


function unzip_file_by_7z($zip_dir, $out_put) {
    . "$($env:NSV_HOME)/tools/7-Zip/7zr.exe" x "-o$zip_dir" -y $out_put
}
function get_node_dir() {
    if ($node_abs_dir) {
        return $node_abs_dir
    }

    $cache_node = Join-Path $cache_path "node\node.exe"
    if (Test-Path $cache_node) {
        $node_abs_dir = $cache_node
        return $node_abs_dir
    }

    $node_command = Get-Command node -ErrorAction SilentlyContinue
    if ($node_command) {
        $min_version = $package.engines.node
        $min_version = $min_version.Substring(2)
        if ($node_command.Version -lt $min_version) {
            return 1
        }
        $node_abs_dir = $node_command.Source
        return $node_abs_dir
    }

    $node_cache_abs_dir = Join-Path $cache_path "node\node.exe"
    if (Test-Path $node_cache_abs_dir) {
        $node_abs_dir = $node_cache_abs_dir
        return $node_abs_dir
    }
    return $node_abs_dir
}

function use_base_node_version {
    $node_state = $(get_node_dir)
    $show_msg = "Command node is not found. Do you want download now? (Y/n)"
    if ($node_state -and ($node_state -ne 1)) {
        return
    }
    if ($node_state -eq 1) {
        $show_msg = "The lowest running version is not met. Do you want download minimum version now? (Y/n)"
    }
    $answer = Read-Host $show_msg
    $answer = $answer.ToLower()
    if (!($answer -eq "y") -and !($answer -eq "yes") -and !($answer.Length -eq 0)) {
        exit 0
    }

    $system_bit = "x86"
    if ($env:PROCESSOR_ARCHITECTURE -ieq "AMD64" -or $env:PROCESSOR_ARCHITEW6432 -ieq "AMD64") {
        $system_bit = "x64"
    }

    $base_node_file_name = "node-v$($package.baseNode)-win-$system_bit"
    $base_node_file_name_suffix = "$base_node_file_name.7z"
    $base_node_download_url = "$($config.source.download)/v$($package.baseNode)/$base_node_file_name_suffix"
    $base_node_file_abs_dir = "$($config.path.cache)/$base_node_file_name_suffix"
    download_file $base_node_download_url $base_node_file_abs_dir
    unzip_file_by_7z $cache_path "$cache_path\$base_node_file_name_suffix"
    Rename-Item "$cache_path\$base_node_file_name" "node"
}

function check_node_version ($ignore_error) {
    $node_file_info = Get-Item $(get_node_dir) -ErrorAction SilentlyContinue
    $node_version = $node_file_info.VersionInfo.FileVersion
    $version_regex = "\d+\.\d+\.\d+"
    $version_match = $node_version -match $version_regex
    if ($ignore_error) {
        return $version_match
    }
    if ($version_match) {
        return
    }
    Write-Error "node version is not match"
    exit 1
}


function run_main_js {
    $path = $(get_node_dir)
    . $path $js_dir $argv
}

function get_temp_script_name {
    return "temp_$Pid.ps1"
}


function run_temp_ps1() {
    $temp_ps_file = Join-Path $cache_path $(get_temp_script_name)
    if (!(Test-Path $temp_ps_file)) {
        return
    }
    & $temp_ps_file

    Remove-item $temp_ps_file
}








function running_js_before {
    use_base_node_version
    check_node_version | Out-Null
    $Env:NSV_TEMP_SCRIPT_NAME = $(get_temp_script_name)
    $Env:NSV_USER_PATH = Get-Path-Crs $($EnvRegistryDir.User)
    $Env:NSV_SYSTEM_PATH = Get-Path-Crs $($EnvRegistryDir.System)
    $Env:isAdmin = $isAdmin
}


function running_js {
    run_main_js
}


function running_js_after {
    $Env:NSV_TEMP_SCRIPT_NAME = ""
    $Env:NSV_USER_PATH = ""
    $Env:NSV_SYSTEM_PATH = ""
    $Env:isAdmin = ""

    run_temp_ps1
}




running_js_before
#
running_js
#
running_js_after
