#!/bin/bash
OS=$(uname)
THIS_FOLDER=$(cd ${0%/*} && echo $PWD)

$THIS_FOLDER/provision/nodejs.sh