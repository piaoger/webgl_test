#!/bin/bash

function get_os() {
    echo $(uname)
}

function common_bashrc_dir() {
    local dir="$(cd "$(dirname "$BASH_SOURCE")"; pwd)"
    echo $dir
}

function app_bin_dir() {
    local this_dir=$(common_bashrc_dir)
    echo ${this_dir}/../bin
}


## -----------------------------------------
## node.js
## -----------------------------------------

function setup_nodejs_env() {
    local nodejs_version=v8.11.1
    local nodejs_pkg_name="unknown"
    local nodejs_root_name="unknown"

    local os=$(get_os)
    if [ "$os" = "Darwin" ]; then
         nodejs_pkg_name=node-${nodejs_version}-darwin-x64.tar.gz
         nodejs_root_name=node-${nodejs_version}-darwin-x64
    else
         nodejs_pkg_name=node-${nodejs_version}-linux-x64.tar.gz
         nodejs_root_name=node-${nodejs_version}-linux-x64
    fi

    local bin_dir=$(app_bin_dir)

    # copy to destination
    local app_nodejs_dir=${bin_dir}/apps/nodejs/${nodejs_root_name}


    export PATH=${app_nodejs_dir}/bin:$PATH:
    export NODE_PATH=$NODE_PATH:${app_nodejs_dir}/lib/node_modules
}
