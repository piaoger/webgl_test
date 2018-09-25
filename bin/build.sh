#!/usr/bin/env bash

OS=$(uname)
THIS_FOLDER=$(cd ${0%/*} && echo $PWD)

APP_ROOT=$THIS_FOLDER/..

set -ex


PROJECT_NAME='webgl_test'

cargo +nightly build --target wasm32-unknown-unknown --release
wasm-bindgen                                                     \
    ./target/wasm32-unknown-unknown/release/"$PROJECT_NAME".wasm \
    --out-dir ${APP_ROOT}/out

cd ${APP_ROOT}/out
if [ "$1" == "fresh" ]
then
    npm install
    npm install typescript
    npm install tslint
fi

export PATH=$PATH:./node_modules/.bin/

tslint -p tsconfig.json
tsc
wasm-gc ./"$PROJECT_NAME"_bg.wasm

#disable wasm-opt temporary
#wasm-opt -O4 ./"$PROJECT_NAME"_bg.wasm
webpack
cp index.html ${THIS_FOLDER}/dist
cp style.css ${THIS_FOLDER}/dist
cp -a img ${THIS_FOLDER}/dist
cd ${APP_ROOT}


