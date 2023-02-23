#!/bin/bash

# get the current datetime and create a directory with that name
NOW=$(date +"%Y-%m-%d_%H-%M-%S")
mkdir -p /opt/backup/$NOW

# copy all files in directory to the backup directory
cp -r /root/Portfolio/data-backend /opt/backup/$NOW

# run pg_dump to backup the database
pg_dump postgresql://DB_USERNAME:DB_PASSWORD@localhost:5432/DATABASE > /opt/backup/$NOW/db.sql

# compress the backup directory
tar -zcvf /opt/backup/$NOW.tar.gz /opt/backup/$NOW

# remove the uncompressed backup directory
rm -rf /opt/backup/$NOW

# scp the backup to the remote server
scp /opt/backup/$NOW.tar.gz USERNAME@SERVER_IP:/home/zalohy/backups/$NOW.tar.gz

# remove the local backup
rm /opt/backup/$NOW.tar.gz