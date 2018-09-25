#!/usr/bin/env bash

OS=$(uname)
THIS_FOLDER=$(cd ${0%/*} && echo $PWD)

APP_ROOT=$THIS_FOLDER/..

source ${THIS_FOLDER}/setupenv.bashrc


set -ex

PROJECT_NAME='webgl_test'


mkdir -p ${APP_ROOT}/www/lib



cargo +nightly build --target wasm32-unknown-unknown --release
wasm-bindgen                                                     \
    ./target/wasm32-unknown-unknown/release/"$PROJECT_NAME".wasm \
    --out-dir ${APP_ROOT}/www/lib

cd ${APP_ROOT}/www

if [ "VAR$1" == "VARfresh" ]; then
	setup_nodejs_env
else
	${THIS_FOLDER}/bootstrap.sh
    npm install
    # npm install typescript
    # npm install tslint
fi


export PATH=$PATH:./node_modules/.bin/

tslint -p tsconfig.json
tsc --outDir ${APP_ROOT}/www/lib
wasm-gc ${APP_ROOT}/www/lib/"$PROJECT_NAME"_bg.wasm

#disable wasm-opt temporary
#wasm-opt -O4 ./"$PROJECT_NAME"_bg.wasm
webpack
cp ${APP_ROOT}/www/static/index.html ${APP_ROOT}/www/dist
cp ${APP_ROOT}/www/static/style.css ${APP_ROOT}/www/dist
cp -a ${APP_ROOT}/www/static/img ${APP_ROOT}/www/dist
cd ${APP_ROOT}


