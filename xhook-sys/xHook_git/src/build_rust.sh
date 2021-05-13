#!/bin/bash


ndk-build -C ./libxhook/jni
cp libxhook/obj/local/arm64-v8a/libxhook.a ../xHook_rust/dep/
