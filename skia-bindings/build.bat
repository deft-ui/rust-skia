@echo off
set FORCE_SKIA_BUILD=1
ohrs build --arch x86_64 -- --features gl,egl,svg,textlayout