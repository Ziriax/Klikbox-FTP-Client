@echo off

setlocal

cd /d %~dp0

SET DOCKER_BUILDKIT=1

docker container rm -f ftp-server-container

docker build -t ftp-server-image .
if errorlevel 1 goto :error

echo %~n0 :-) success!
goto :exit

:error
echo %~n0 :-( failure!

:exit
endlocal

cd /d %~dp0
