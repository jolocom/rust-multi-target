#!/bin/bash

set -ex
source ./variables.sh

cd $BASE_DIR/react-native/rust/keriox_wrapper/

# Build android

if [ -z ${NDK_HOME+x} ];
  then
    printf 'Please install android-ndk\n\n'
    printf 'from https://developer.android.com/ndk/downloads or with sdkmanager'
    exit 1
  else
    printf "Building Andriod targets...";
fi

CC_aarch64_linux_android="${ANDROID_PREBUILD_BIN}/aarch64-linux-android${API_LEVEL}-clang" \
CARGO_TARGET_AARCH64_LINUX_ANDROID_LINKER="${ANDROID_PREBUILD_BIN}/aarch64-linux-android${API_LEVEL}-clang" \
AR_aarch64_linux_android="${ANDROID_PREBUILD_BIN}/aarch64-linux-android-ar" \
  cargo build --target aarch64-linux-android --release

CC_armv7_linux_androideabi="${ANDROID_PREBUILD_BIN}/armv7a-linux-androideabi${API_LEVEL}-clang" \
CARGO_TARGET_ARMV7_LINUX_ANDROIDEABI_LINKER="${ANDROID_PREBUILD_BIN}/armv7a-linux-androideabi${API_LEVEL}-clang" \
AR_armv7_linux_androideabi="${ANDROID_PREBUILD_BIN}/arm-linux-androideabi-ar" \
  cargo build --target armv7-linux-androideabi --release


CC_i686_linux_android="${ANDROID_PREBUILD_BIN}/i686-linux-android${API_LEVEL}-clang" \
CARGO_TARGET_I686_LINUX_ANDROID_LINKER="${ANDROID_PREBUILD_BIN}/i686-linux-android${API_LEVEL}-clang" \
AR_i686_linux_android="${ANDROID_PREBUILD_BIN}/i686-linux-android-ar" \
  cargo  build --target i686-linux-android --release


for i in "${!ANDROID_ARCHS[@]}";
  do
    mkdir -p -v "../../android/src/main/jniLibs/${ANDROID_FOLDER[$i]}"
    cp "./target/${ANDROID_ARCHS[$i]}/release/lib${LIB_NAME}.so" "../../android/src/main/jniLibs/${ANDROID_FOLDER[$i]}/lib${LIB_NAME}.so"
done


if [ "$(uname | tr '[:upper:]' '[:lower:]')" == "darwin" ]; then
  printf "Building iOS targets...";

  for i in "${IOS_ARCHS[@]}";
    do
      cargo build --target "$i" --release --no-default-features
  done

  LIPO_LIBS=$(for T in ${IOS_ARCHS[@]}; do echo target/${T}/release/lib${LIB_NAME}.a; done)
  lipo -create -output "$BASE_DIR/react-native/ios/lib${LIB_NAME}.a" $LIPO_LIBS
else
  printf "skipping iOS target, please use a Mac for those"
fi

printf "Build Complete"
