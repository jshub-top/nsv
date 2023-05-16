




$proxy_url = "https://ghproxy.com"
$nsv_tgz_file_name = "nsv.tgz"
$nsv_tgz_file_url = "https://github.com/1739616529/nsv/releases/download/v0.0.1/$nsv_tgz_file_name"


$proxy_status = Read-Host "nsv: enable proxy. [Y/n]"
$proxy_status = $proxy_status.toLower()
if ("$proxy_status" -ieq "" -or "$proxy_status" -ieq "yes" -or "$proxy_status" -ieq "y") {
    $nsv_tgz_file_url = "$proxy_url/$nsv_tgz_file_url"
}



function install_dir() {
    $dir = Read-Host "nsv: install path: (default user flower)"
    if (!$dir) {
        $dir = "$Env:USERPROFILE"
    }
    return $dir
}

$dir = install_dir
Write-Output $dir
# Invoke-WebRequest $nsv_tgz_file_url -OutFile "$dir\$nsv_tgz_file_name"

# $tar_command = Get-Command "tar" -ErrorAction SilentlyContinue
if (!$tar_command) {
    Write-Output "nsv: tar is not nonsupport"
    Write-Output "1. nsv: cd $dir"
    Write-Output "2. nsv: manual decompression ==> nsv.tgz <=="
    Write-Output "3. nsv: $dir\package\nsv.ps1 install"
}


# tar
