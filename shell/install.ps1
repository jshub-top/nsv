



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
$_pwd = $pwd
cd "$dir"
Invoke-WebRequest $nsv_tgz_file_url -OutFile "$nsv_tgz_file_name"

$tar_command = Get-Command "tar" -ErrorAction SilentlyContinue
if (!$tar_command) {
    Write-Output "nsv: tar is not nonsupport"
    Write-Output "  1. cd $dir"
    Write-Output "  2. manual decompression ==> nsv.tgz <=="
    Write-Output "  3. $dir\package\nsv.ps1 install"
    cd $_pwd
    exit 0
}


tar xf "$nsv_tgz_file_name"
# Rename-Item "package" "nsv"
mkdir -Force nsv
Remove-Item nsv\* -Recurse -Exclude cache,local,node
Copy-Item -Path package\* -Recurse nsv -Force
Remove-Item package -Recurse
# Remove-Item "$nsv_tgz_file_name"
. "nsv\nsv.ps1" "install"
cd $_pwd
