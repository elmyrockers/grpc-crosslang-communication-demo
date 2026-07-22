@echo off
setlocal

REM Ensure build is configured first
if not exist build\build.ninja (
    cmake -B build -G Ninja -DCMAKE_BUILD_TYPE=Debug -DCMAKE_CXX_COMPILER=C:/Tools/llvm-mingw/bin/clang++.exe -DCMAKE_C_COMPILER=C:/Tools/llvm-mingw/bin/clang.exe -DCMAKE_TOOLCHAIN_FILE=C:/Tools/vcpkg/scripts/buildsystems/vcpkg.cmake -DVCPKG_TARGET_TRIPLET=x64-mingw-static
    if errorlevel 1 (
        echo Configure failed.
        exit /b 1
    )
)

watchexec --shell=cmd -e cpp,h,hpp,cc,cppm -w src -i build -i vcpkg_installed -r --clear -- "cmake --build build && build\app.exe"