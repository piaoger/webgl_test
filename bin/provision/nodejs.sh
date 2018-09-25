#!/bin/bash
OS=$(uname)
THIS_DIR=$(cd ${0%/*} && echo $PWD)
APP_BIN_DIR=$THIS_DIR/..

APP_NAME=uyun

APP_BIN_APPS_DIR=$THIS_DIR/../apps
APP_CACHE_DIR=~/.$APP_NAME/cache

## -----------------------------------------
## node.js
## -----------------------------------------

mkdir -p $APP_CACHE_DIR
cd $APP_CACHE_DIR

export NODEJS_VERSION=v8.11.1

if [ "$OS" = "Darwin" ]; then
   export NODEJS_PKG_NAME=node-$NODEJS_VERSION-darwin-x64.tar.gz
   export NODEJS_ROOT_NAME=node-$NODEJS_VERSION-darwin-x64
else
   export NODEJS_PKG_NAME=node-$NODEJS_VERSION-linux-x64.tar.gz
   export NODEJS_ROOT_NAME=node-$NODEJS_VERSION-linux-x64
fi

if [ ! -f $NODEJS_PKG_NAME ]; then
    wget --no-check-certificate https://nodejs.org/dist/$NODEJS_VERSION/$NODEJS_PKG_NAME
    #wget http://olxkpcnfn.bkt.clouddn.com/$NODEJS_PKG_NAME
fi
tar -xf $NODEJS_PKG_NAME

# copy to destination
APP_BIN_NODEJS_DIR=$APP_BIN_APPS_DIR/nodejs/$NODEJS_ROOT_NAME
rm -rf $APP_BIN_APPS_DIR/nodejs/

mkdir -p $APP_BIN_NODEJS_DIR
mv -f $APP_CACHE_DIR/$NODEJS_ROOT_NAME/* $APP_BIN_NODEJS_DIR
rm -rf $APP_CACHE_DIR/$NODEJS_ROOT_NAME

export PATH=$APP_BIN_NODEJS_DIR/bin:$PATH:
export NODE_PATH=$NODE_PATH:$APP_BIN_NODEJS_DIR/lib/node_modules

echo $APP_BIN_NODEJS_DIR/bin/node
echo "node version:"
node --version