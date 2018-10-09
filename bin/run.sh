#!/usr/bin/env bash

OS=$(uname)
THIS_FOLDER=$(cd ${0%/*} && echo $PWD)

APP_ROOT=$THIS_FOLDER/..

source ${THIS_FOLDER}/setupenv.bashrc


${APP_ROOT}/bin/build.sh fresh

cd ${APP_ROOT}/server
make
cd ${APP_ROOT}/www/dist

killall -9 webgl_test_server
${APP_ROOT}/server/target/release/webgl_test_server &
cd ${APP_ROOT}

