@echo off
REM Build Docker image using WSL

set IMAGE_NAME=http-server:latest
set DOCKERFILE_PATH=/mnt/c/Projects/All/grpc-crosslang-communication-demo/http-server/Dockerfile
set CONTEXT_PATH=/mnt/c/Projects/All/grpc-crosslang-communication-demo/http-server

wsl -d Ubuntu -e bash -c "docker build -t %IMAGE_NAME% -f %DOCKERFILE_PATH% %CONTEXT_PATH%"

pause
