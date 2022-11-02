#!/bin/bash

# Set base dir to the rust-multi-target top level
BASE_DIR=$(realpath $(dirname "$0")/..)

# Change this name to the rust library name
LIB_NAME=keriox_wrapper
API_LEVEL=30

ANDROID_HOME=/opt/android-sdk
NDK_HOME=/opt/android-sdk/ndk/21.3.6528147
ANDROID_ARCHS=(aarch64-linux-android armv7-linux-androideabi)
ANDROID_FOLDER=(arm64-v8a armeabi-v7a x86)
ANDROID_BIN_PREFIX=(aarch64-linux-android armv7a-linux-androideabi)
IOS_ARCHS=(aarch64-apple-ios x86_64-apple-ios) # armv7-apple-ios armv7s-apple-ios)
OS_ARCH=$(uname | tr '[:upper:]' '[:lower:]')

ANDROID_PREBUILD_BIN=${NDK_HOME}/toolchains/llvm/prebuilt/${OS_ARCH}-x86_64/bin

#CC_aarch64_linux_android=${ANDROID_PREBUILD_BIN}/aarch64-linux-android29-clang
#CC_armv7_linux_androideabi=${ANDROID_PREBUILD_BIN}/armv7a-linux-androideabi29-clang
#CC_i686_linux_android=${ANDROID_PREBUILD_BIN}/i686-linux-android29-clang
#CC_x86_64_linux_android=${ANDROID_PREBUILD_BIN}/x86_64-linux-android29-clang
