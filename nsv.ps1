


$argv = $args
function download_file($url, $out_put) {
    $proxyStatus = Get-ItemProperty -Path 'HKCU:\Software\Microsoft\Windows\CurrentVersion\Internet Settings' -Name ProxyEnable -ErrorAction SilentlyContinue | Select-Object -ExpandProperty ProxyEnable
    echo $proxyStatus
    if ($proxyStatus) {
        $proxySettings = Get-ItemProperty -Path "Registry::HKEY_CURRENT_USER\Software\Microsoft\Windows\CurrentVersion\Internet Settings" -Name ProxyServer -ErrorAction SilentlyContinue
        write-Output "nsv: use proxy with--> $($proxySettings.ProxyServer)"
        $proxy_serve = "http://$($proxySettings.ProxyServer)"
        Invoke-WebRequest $url -OutFile $out_put -Proxy $proxy_serve
        return
    }
    Invoke-WebRequest $url -OutFile $out_put
}

function nsv_root_dir($dir) {
    return Join-Path $env:NSV_HOME $dir
}

function unzip_file_by_7z($zip_dir, $out_put) {
    . "tools\7-Zip\7zr.exe" x "-o$out_put" -y $zip_dir
}

function use_base_node () {
    if (Test-Path "cache\node") {
        return
    }

    if (!(Test-Path "node")) {
        New-Item -Path "node" -ItemType Directory
    }
    if (!(Test-Path "cache")) {
        New-Item -Path "cache" -ItemType Directory
    }


    $system_bit = ""
    if ($env:PROCESSOR_ARCHITECTURE -ieq "AMD64" -or $env:PROCESSOR_ARCHITEW6432 -ieq "AMD64") {
        $system_bit = "x64"
    } elseif ($env:PROCESSOR_ARCHITECTURE -ieq "x86" -or $env:PROCESSOR_ARCHITEW6432 -ieq "x86") {
        $system_bit = "x86"
    } elseif ($env:PROCESSOR_ARCHITECTURE -ieq "ARM64" -or $env:PROCESSOR_ARCHITEW6432 -ieq "ARM64") {
        $system_bit = "arm64"
    }

    $package = Get-Content -Path "package.json" | ConvertFrom-Json
    $config = Get-Content -Path "config.json" | ConvertFrom-Json
    $base_node_version = $package.baseNode.win."$system_bit"
    if (! $base_node_version) {$base_node_version  = $package.baseNode.default}
    $base_node_file_name = "node-v$base_node_version-win-$system_bit"
    $base_node_file_name_suffix = "$base_node_file_name.7z"
    $base_node_download_url = "$($config.source.download)\v$base_node_version\$base_node_file_name_suffix"
    $base_node_file_abs_dir = "$($config.path.cache)\$base_node_file_name_suffix"
    download_file $base_node_download_url $base_node_file_abs_dir
    unzip_file_by_7z "cache\$base_node_file_name_suffix" "cache"
    Rename-Item "cache\$base_node_file_name" "node"

}


function check_node_modules () {
    if (Test-Path "node_modules") {
        return
    }
    $Env:PATH = "$scriptDir\cache\node;$Env:PATH"
    npm install --production
    npm run init
}

function nsv () {
    try {
        Remove-item $temp_ps_file
    } catch {

    }
    . "$scriptDir\cache\node\node.exe" "$scriptDir\dist\index.js" $argv

    $temp_ps_file = "$scriptDir\cache\nsv_temp_one_off_file.ps1"
    if (Test-Path $temp_ps_file) {
        & $temp_ps_file
    }
}


# 运行 这些脚本需要进入 安装目录 运行完成之后返回用户当前目录
$_pwd = $PWD
$scriptDir = $PSScriptRoot
cd $scriptDir
use_base_node
check_node_modules
cd $_pwd




nsv
