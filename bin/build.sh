#!/usr/bin/env bash

OS=$(uname)
THIS_FOLDER=$(cd ${0%/*} && echo $PWD)

APP_ROOT=$THIS_FOLDER/..

source ${THIS_FOLDER}/setupenv.bashrc


set -ex

PROJECT_NAME='webgl_test'


mkdir -p ${APP_ROOT}/www/lib

rm -rf ${APP_ROOT}/www/dist
mkdir -p ${APP_ROOT}/www/dist


if [ "VAR$1" == "VARdebug" ]; then
	cargo +nightly build --target wasm32-unknown-unknown
	wasm-bindgen                                                     \
	    ./target/wasm32-unknown-unknown/debug/"$PROJECT_NAME".wasm \
	    --out-dir ${APP_ROOT}/www/src
elif [ "VAR$1" == "VARrelease" ]; then
	cargo +nightly build --target wasm32-unknown-unknown --release
	wasm-bindgen                                                     \
	    ./target/wasm32-unknown-unknown/release/"$PROJECT_NAME".wasm \
	    --out-dir ${APP_ROOT}/www/src
elif [ "VAR$1" == "VARcheck" ]; then
	cargo +nightly check --target wasm32-unknown-unknown
	exit 0
else
	exit 1
fi


cd ${APP_ROOT}/www

if [ "VAR$2" == "VARfresh" ]; then
	setup_nodejs_env

else
	${THIS_FOLDER}/bootstrap.sh
	setup_nodejs_env
    npm install
    npm install --save @types/webassembly-js-api

    npm install --save typescript-formatter
    # npm install typescript
    # npm install tslint
fi


export PATH=$PATH:./node_modules/.bin/

tsfmt -r

#tslint -p tsconfig.json
tsc --allowJs --outDir ${APP_ROOT}/www/lib
wasm-gc ${APP_ROOT}/www/src/"$PROJECT_NAME"_bg.wasm

cp ${APP_ROOT}/www/src/"$PROJECT_NAME"_bg.wasm ${APP_ROOT}/www/lib
cp ${APP_ROOT}/www/src/"$PROJECT_NAME".js ${APP_ROOT}/www/lib


#disable wasm-opt temporary
#wasm-opt -O4 ./"$PROJECT_NAME"_bg.wasm
webpack
cp ${APP_ROOT}/www/static/index.html ${APP_ROOT}/www/dist
cp ${APP_ROOT}/www/static/style.css ${APP_ROOT}/www/dist
cp -a ${APP_ROOT}/www/static/img ${APP_ROOT}/www/dist
cd ${APP_ROOT}