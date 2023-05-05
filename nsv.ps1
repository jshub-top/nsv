


$argv = $args
$scriptDir = $PSScriptRoot
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

function unzip_file_by_7z($zip_dir, $out_put) {
    . "$scriptDir/tools/7-Zip/7zr.exe" x "-o$zip_dir" -y $out_put
}

function use_base_node () {

    if (Test-Path "$scriptDir/cache/node") {
        return
    }

    $system_bit = "x86"
    if ($env:PROCESSOR_ARCHITECTURE -ieq "AMD64" -or $env:PROCESSOR_ARCHITEW6432 -ieq "AMD64") {
        $system_bit = "x64"
    }

    $package = Get-Content -Path "$scriptDir/package.json" | ConvertFrom-Json
    $config = Get-Content -Path "$scriptDir/config.json" | ConvertFrom-Json
    $base_node_file_name = "node-v$($package.baseNode)-win-$system_bit"
    $base_node_file_name_suffix = "$base_node_file_name.7z"
    $base_node_download_url = "$($config.source.download)/v$($package.baseNode)/$base_node_file_name_suffix"
    $base_node_file_abs_dir = "$($config.path.cache)/$base_node_file_name_suffix"
    download_file $base_node_download_url $base_node_file_abs_dir
    unzip_file_by_7z cache "$scriptDir/cache/$base_node_file_name_suffix"
    Rename-Item "$scriptDir/cache/$base_node_file_name" "node"
}

function nsv () {
    $NSV_TEMP_SCRIPT_NAME = "temp_$Pid.ps1"
    $Env:NSV_TEMP_SCRIPT_NAME = $NSV_TEMP_SCRIPT_NAME
    . "$scriptDir/cache/node/node.exe" "$scriptDir/dist/index.js" $argv
    $Env:NSV_TEMP_SCRIPT_NAME = ""

    $temp_ps_file = "$scriptDir/cache/$NSV_TEMP_SCRIPT_NAME"
    if (Test-Path $temp_ps_file) {
        & $temp_ps_file
        Remove-item $temp_ps_file
    }

}

use_base_node
nsv
