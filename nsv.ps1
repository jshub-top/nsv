


$argv = $args
$scriptDir = $PSScriptRoot
function download_file($url, $out_put) {
    $proxySettings = Get-ItemProperty -Path "Registry::HKEY_CURRENT_USER\Software\Microsoft\Windows\CurrentVersion\Internet Settings" -Name ProxyServer -ErrorAction SilentlyContinue
    write-Output "nsv: use proxy with--> $($proxySettings.ProxyServer)"
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
    . "$scriptDir\tools\7-Zip\7zr.exe" x "-o$zip_dir" -y $out_put
}

function use_base_node () {

    if (Test-Path "$scriptDir\cache\node") {
        return
    }

    if (!(Test-Path "$scriptDir\node")) {
        New-Item -Path "$scriptDir\node" -ItemType Directory
    }
    if (!(Test-Path "$scriptDir\cache")) {
        New-Item -Path "$scriptDir\cache" -ItemType Directory
    }


    $system_bit = ""
    if ($env:PROCESSOR_ARCHITECTURE -ieq "AMD64" -or $env:PROCESSOR_ARCHITEW6432 -ieq "AMD64") {
        $system_bit = "x64"
    } elseif ($env:PROCESSOR_ARCHITECTURE -ieq "x86" -or $env:PROCESSOR_ARCHITEW6432 -ieq "x86") {
        $system_bit = "x86"
    } elseif ($env:PROCESSOR_ARCHITECTURE -ieq "ARM64" -or $env:PROCESSOR_ARCHITEW6432 -ieq "ARM64") {
        $system_bit = "arm64"
    }

    $package = Get-Content -Path "$scriptDir\package.json" | ConvertFrom-Json
    $config = Get-Content -Path "$scriptDir\config.json" | ConvertFrom-Json
    $base_node_version = $package.baseNode.win."$system_bit"
    if (! $base_node_version) {$base_node_version  = $package.baseNode.default}
    Write-Output $base_node_version
    $base_node_file_name = "node-v$base_node_version-win-$system_bit"
    $base_node_file_name_suffix = "$base_node_file_name.7z"
    $base_node_download_url = "$($config.source.download)\v$base_node_version\$base_node_file_name_suffix"
    $base_node_file_abs_dir = "$($config.path.cache)\$base_node_file_name_suffix"
    download_file $base_node_download_url $base_node_file_abs_dir
    unzip_file_by_7z cache "$scriptDir\cache\$base_node_file_name_suffix"
    Rename-Item "$scriptDir\cache\$base_node_file_name" "node"

}


function check_node_modules () {
    if (Test-Path "$scriptDir/node_modules") {
        return
    }
    cd "$scriptDir"
    $Env:PATH = ""
    Start-Process "$scriptDir/cache/node/npm.cmd" "install --production" -NoNewWindow -Wait
    Start-Process "$scriptDir/cache/node/npm.cmd" "install --production" -NoNewWindow -Wait
}

function nsv () {
    . "$scriptDir/cache/node/node.exe" "$scriptDir/dist/index.js" $argv

    $temp_ps_file = "$scriptDir/cache/nsv_temp_one_off_file.ps1"
    if (Test-Path $temp_ps_file) {
        & $temp_ps_file
        # Remove-item $temp_ps_file
    }
}

use_base_node
check_node_modules
nsv

exit 0
