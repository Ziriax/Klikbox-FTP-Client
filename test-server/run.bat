docker build -t ftp-server-image .

REM run the ftp server instance in detached mode (in the background)
REM but also with TTY and interactive mode, so we can attach to it if we want to
docker rm -f ftp-server-container

docker run -dti --name ftp-server-container --privileged -p 21:21 -p 65000-65010:65000-65010 ftp-server
