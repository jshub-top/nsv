#!/bin/bash
OS=$(uname | sed 'y/ABCDEFGHIJKLMNOPQRSTUVWXYZ/abcdefghijklmnopqrstuvwxyz/')



if [[ ! -d "./cache/node" ]]; then
    source ./tools/json/json.sh
    mkdir -p cache
    mkdir -p node
    arch="$(uname -m | sed -e 's/x86_64/x64/;s/i86pc/x64/;s/i686/x86/;s/aarch64/arm64/')"
    base_download_uri=$(params_json "$(cat ./config.json)" "source" "download" )
    base_node_version=$(params_json "$(cat ./package.json)" "baseNode" )
    base_node_name="node-v${base_node_version}-${OS}-${arch}"
    node_file_name="$base_node_name.tar.gz"
    base_node_download_uri="${base_download_uri}/v${base_node_version}/${node_file_name}"
    save_file_dir="./cache/$node_file_name"
    curl "$base_node_download_uri" -# -O
    chmod 755 "$node_file_name"
    mv "$node_file_name"  "$save_file_dir"
    tar -xf $save_file_dir -C "./cache"
    mv "cache/$base_node_name"  "cache/node"
fi


function nsv() {
    export NSV_TEMP_SCRIPT_NAME="temp_$$.sh"
    "./cache/node/bin/node" "./dist/index.js" $@
    local temp_shell_dir="./cache/$NSV_TEMP_SCRIPT_NAME"
    if [[ -f $temp_shell_dir ]]; then
        . $temp_shell_dir
        rm $temp_shell_dir
    fi
    unset NSV_TEMP_SCRIPT_NAME
}
