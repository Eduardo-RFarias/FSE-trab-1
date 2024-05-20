#!/bin/sh

# Check if sshpass is installed
if ! [ -x "$(command -v sshpass)" ]; then
    echo "Error: sshpass is not installed. Please install sshpass using 'sudo apt-get install sshpass' and try again"
    exit 1
fi

# Check if the SSHPASS environment variable is set
if [ -z "$SSHPASS" ]; then
    echo "Error: SSHPASS environment variable is not set. Please set SSHPASS to the password of the Raspberry Pi and try again"
    exit 1
fi


# Check if the fse folder exists on the Raspberry Pi and if all the binaries are there
if ! sshpass -e ssh eduardofarias@164.41.98.16 -p 13508 "[ -d ~/fse ] && [ -f ~/fse/fse_trab_1_server ] && [ -f ~/fse/fse_trab_1_ground_floor ] && [ -f ~/fse/fse_trab_1_first_floor ] && [ -f ~/fse/fse_trab_1_second_floor ]"; then
    echo "Please run send.sh before running this script"
    exit 1
fi

# Create or replace the logs folder and the db folder
sshpass -e ssh eduardofarias@164.41.98.16 -p 13508 "
    if [ -d ~/fse/logs ]; then
        rm -rf ~/fse/logs
    fi
    mkdir ~/fse/logs
    if [ -d ~/fse/db ]; then
        rm -rf ~/fse/db
    fi
    mkdir ~/fse/db
"

# Run the binaries on the Raspberry Pi
echo "Running binaries on the Raspberry Pi..."

sshpass -e ssh eduardofarias@164.41.98.16:~/fse -p 13508 "nohup ./fse_trab_1_server > server.log 2>&1 &"
sshpass -e ssh eduardofarias@164.41.98.16:~/fse -p 13508 "nohup ./fse_trab_1_ground_floor > ground_floor.log 2>&1 &"
sshpass -e ssh eduardofarias@164.41.98.16:~/fse -p 13508 "nohup ./fse_trab_1_first_floor > first_floor.log 2>&1 &"
sshpass -e ssh eduardofarias@164.41.98.16:~/fse -p 13508 "nohup ./fse_trab_1_second_floor > second_floor.log 2>&1 &"

echo "All binaries are running on the Raspberry Pi, use htop to monitor them or send SIGINT to stop them"
