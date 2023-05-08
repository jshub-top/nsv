#!/bin/sh
OS=$(uname | sed 'y/ABCDEFGHIJKLMNOPQRSTUVWXYZ/abcdefghijklmnopqrstuvwxyz/')
dir=$(dirname $(readlink -f "$0"))



if [[ ! -d "./cache/node" ]]; then
    source "$dir/tools/json/json.sh"
    mkdir -p cache
    mkdir -p node
    arch="$(uname -m | sed -e 's/x86_64/x64/;s/i86pc/x64/;s/i686/x86/;s/aarch64/arm64/')"
    base_download_uri=$(parse_json "$(cat $dir/config.json)" "source" "download" )
    base_node_version=$(parse_json "$(cat $dir/package.json)" "baseNode" $OS $arch )
    if [ -z $base_node_version ]; then
        base_node_version=$(parse_json "$(cat ./package.json)" "baseNode" "default")
    fi
    base_node_name="node-v${base_node_version}-${OS}-${arch}"
    node_file_name="$base_node_name.tar.gz"
    base_node_download_uri="${base_download_uri}/v${base_node_version}/${node_file_name}"
    save_file_dir="$dir/cache/$node_file_name"
    curl "$base_node_download_uri" -# -O
    chmod 755 "$dir/$node_file_name"
    mv "$dir$node_file_name"  "$dir/$save_file_dir"
    tar -xf "$dir/$save_file_dir" -C "$dir/cache"
    mv "$dir/cache/$base_node_name"  "$dir/cache/node"
fi


function nsv() {
    export NSV_STATUS=0

    if [[ ! -d "$dir/node_modules" ]]; then
        export NSV_STATUS=2
        export PATH="$dir/cache/node/bin":$PATH
        npm install
    fi


    export NSV_TEMP_SCRIPT_NAME="temp_$$.sh"
    "$dir/cache/node/bin/node" "$dir/dist/index.js" $@
    local temp_shell_dir="$dir/cache/$NSV_TEMP_SCRIPT_NAME"
    if [[ -f $temp_shell_dir ]]; then
        . $temp_shell_dir
        rm $temp_shell_dir
    fi
    unset NSV_TEMP_SCRIPT_NAME
    unset NSV_STATUS
}
