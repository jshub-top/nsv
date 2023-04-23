#!/bin/bash
ShScriptRoot=$(readlink -f $(dirname $0))
OS=$(uname | sed 'y/ABCDEFGHIJKLMNOPQRSTUVWXYZ/abcdefghijklmnopqrstuvwxyz/')
args=$@
if [[ ! $NSV_HOME ]]
then
    export NSV_HOME=$ShScriptRoot
fi
init() {
    if [[ $1 != "--init" ]]; then
        return
    fi

    function set_env() {

        grep ".nsv" ~/.bashrc >/dev/null
        if [ $? -eq 1 ] ; then
            echo "export NSV_HOME=$ShScriptRoot" >> ~/.bashrc
            echo "export PATH=\$NSV_HOME/.nsv:\$PATH" >> ~/.bashrc
            source ~/.bashrc
            state+=1

        fi
    }

    set_env
    exit 0
}



function download_file() {
    if [[ ! $1 ]]; then
        return
    fi
    uri=$1
    file_name=$(basename $1)
    save_dir=$2
    curl $uri -O -#
    mv "./$file_name" $save_dir
}

node_abs_dir=""
function get_node_abs() {
    if [[ $node_abs_dir ]]; then
        echo $node_abs_dir
        return
    fi

    # node_abs_dir=$(which "node")
    # if [[ $node_abs_dir ]]; then
    #     echo $node_abs_dir
    #     return
    # fi

    local cache_dir=$NSV_HOME/cache/node/bin/node
    if [[ -f $cache_dir ]]; then
        node_abs_dir=$cache_dir
        echo $node_abs_dir
        return 
    fi
    echo $node_abs_dir
}

function unzip_file {
    local file=$1
    local dir=$2
    tar -xf $1 -C $2
}

function cache_node() {
    local node_dir=$(get_node_abs)
    if [[ $node_dir ]]; then
        return
    fi
    source ./tools/json/json.sh
    
    local arch="$(uname -m | sed -e 's/x86_64/x64/;s/i86pc/x64/;s/i686/x86/;s/aarch64/arm64/')"
    local base_download_uri=$(params_json "$(cat ./config.json)" "source" "download" )
    local base_node_version=$(params_json "$(cat ./package.json)" "baseNode" )
    local base_node_name="node-v${base_node_version}-${OS}-${arch}"
    local node_file_name="$base_node_name.tar.xz"
    local base_node_download_uri="${base_download_uri}/v${base_node_version}/${node_file_name}"
    local save_file_dir="$ShScriptRoot/cache/$node_file_name"
    # download_file "http://127.0.0.1:3000/$node_file_name" $save_file_dir
    download_file $base_node_download_uri "$ShScriptRoot/cache/$node_file_name"
    unzip_file $save_file_dir "$ShScriptRoot/cache"
    mv "$ShScriptRoot/cache/$base_node_name"  "cache/node"
    
}


function run_js_main() {
    local node_dir=$(get_node_abs)
    $node_dir "./dist/index.js" $args
}



function run_temp_script() {
    local temp_script_dir="$ShScriptRoot/cache/temp_$$.sh"
    if [[ ! -e $temp_script_dir ]]; then 
        return 
    fi
    source $temp_script_dir
}
















function run_js_brfore() {
    cache_node
    export NSV_TEMP_SCRIPT_NAME="temp_$$.sh"
}

function run_js() {
    run_js_main
}

function run_js_after() {
    unset NSV_TEMP_SCRIPT_NAME
    run_temp_script
}

run_js_brfore
run_js
run_js_after
