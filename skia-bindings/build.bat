@echo off
set FORCE_SKIA_BUILD=1

mkdir out\x86_64
set BUILD_ARTIFACTSTAGINGDIRECTORY=out\x86_64
ohrs build --release --arch x86_64 -- --features gl,egl,svg,textlayout

mkdir out\aarch
set BUILD_ARTIFACTSTAGINGDIRECTORY=out\aarch
ohrs build --release --arch aarch -- --features gl,egl,svg,textlayout