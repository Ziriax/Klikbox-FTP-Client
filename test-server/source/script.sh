#!/bin/bash

# exit when any command fails
set -e

echo Initializing FTP server...

mkdir -p /var/run/vsftpd/empty
    
useradd -s /bin/bash -d /home/ftp -m -c "Doe ftp user" -g ftp Doe

echo "Doe:mumble"| chpasswd

cp /source/vsftpd.conf /etc/

touch /var/log/vsftpd.log
tail -f /var/log/vsftpd.log | tee $(tty) &

/etc/init.d/vsftpd start

i=0
delay=0
while true
do
  echo "Uploading photo $i..."
  cp /source/photo.jpg /tmp/photo$i.jpg
  ftp-upload -h 127.0.0.1:21 -u Doe --password mumble -d /home/ftp /tmp/photo$i.jpg
  delay=$((5 + $RANDOM % 5))
  echo "Sleeping $delay seconds..."
  sleep $delay
  i=$(( i + 1 ))
done

