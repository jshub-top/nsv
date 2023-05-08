#!/bin/bash
ShScriptRoot=$(readlink -f $(dirname $0))
OS=$(uname | sed 'y/ABCDEFGHIJKLMNOPQRSTUVWXYZ/abcdefghijklmnopqrstuvwxyz/')
echo $OS
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
            echo "export PATH=\$HOME/.nsv:\$PATH" >> ~/.bashrc
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
    # curl $uri -o
}

node_abs_dir=""
function get_node_abs() {
    if [[ $node_abs_dir ]]; then
        echo $node_abs_dir
        return
    fi

    node_abs_dir=$(which node)
    if [[ $node_abs_dir ]]; then
        echo $node_abs_dir
        return
    fi


    local cache_dir=$NSV_HOME/cache/node/node
    if [[ -f $cache_dir ]]; then
        node_abs_dir=$cache_dir
        echo $node_abs_dir
    fi
    echo $node_abs_dir
}


function cache_node() {
    local node_dir=$(get_node_abs)
    if [[ $node_dir ]]; then
        return
    fi
    


}

function run_js_brfore() {
    cache_node
}

# function run_js() {

# }

# function run_js_after() {

# }

run_js_brfore
# run_js
# run_js_after
