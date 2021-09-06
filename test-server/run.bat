@echo off

setlocal

cd /d %~dp0

SET DOCKER_BUILDKIT=1

docker container rm -f ftp-server-container

docker run -ti --name ftp-server-container --volume "%~dp0source":/source --privileged -p 21:21 -p 65000-65010:65000-65010 ftp-server-image /source/script.sh

rem docker run -ti --name ftp-server-container --volume "%~dp0source":/source --privileged -p 20:20 -p 21:21 -p 65000-65010:65000-65010 ftp-server-image /bin/bash

if errorlevel 1 goto :error

echo %~n0 :-) success!
goto :exit

:error
echo %~n0 :-( failure!

:exit
endlocal

cd /d %~dp0
