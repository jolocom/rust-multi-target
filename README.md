# Jolocom Native Core Bindings

This package contains Node and React Native bindings for
[jolocom/wallet-rs](https://github.com/jolocom/wallet-rs) and
[jolocom/keriox](https://github.com/jolocom/keriox)

This is intended to be used through
[jolocom/vaulted-key-provider](https://github.com/jolocom/vaulted-key-provider)

It was originally based on the amazing[React Native Substrate Sign](https://github.com/paritytech/react-native-substrate-sign), from Parity.

## Build and Develop

### Requirements

- `node.js` ( `>=10`)
- `yarn` (tested on `1.6.0`)
- `rustup` (tested on `rustup 1.21.0`)
- `rustc` (tested on `rustc 1.41.1`,  from 1.42.0 rust [dropped 32-bit apple target support](https://blog.rust-lang.org/2020/01/03/reducing-support-for-32-bit-apple-targets.html))
- `cargo` (tested on `cargo 1.41.0`)
- `android_ndk` (tested on `r21`, can be downloaded [here](https://developer.android.com/ndk/downloads))
- `$NDK_HOME` envarionment variable set to ndk home directory (eg. `/usr/local/opt/android-ndk`)

\* It's recommended to install **Android Studio** and use that to install the necessary build tools and SDKs for the Android version you want to test on. It's also the best way to test in the emulator. 

### Setup

- Use the following script to install the required rust toolchains.

```shell script
./scripts/init.sh
```


### Develop
After update the rust code, you need to change the following files for updating the interface to native android and ios code.

- ios/core.h
- ios/JolocomCore.m
- ios/JolocomCore.swift
- android/src/main/java/io/jolocom/JolocomCoreModule.java
- index.js
- index.d.ts

### Test

- To run the rust test

```shell script
yarn test
```

### Build

- Use the following script to build the dynamic library for Android and static library for iOS.

```shell script
./scripts/build.sh
```
