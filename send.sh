#!/bin/sh

# Check if the .bin folder exists and if all the binaries are there
if [ ! -d ".bin" ] || [ ! -f ".bin/fse_trab_1_server" ] || [ ! -f ".bin/fse_trab_1_ground_floor" ] || [ ! -f ".bin/fse_trab_1_first_floor" ] || [ ! -f ".bin/fse_trab_1_second_floor" ]; then
  echo "Please run compile.sh before running this script"
  exit 1
fi

# Check if sshpass is installed
if ! [ -x "$(command -v sshpass)" ]; then
  echo "Error: sshpass is not installed. Please install sshpass using 'sudo apt-get install sshpass' and try again"
  exit 1
fi

# Check if SSHPASS environment variable is set
if [ -z "$SSHPASS" ]; then
  echo "Error: SSHPASS environment variable is not set. Please set SSHPASS to the password of the Raspberry Pi and try again"
  exit 1
fi


echo "Creating the fse folder on the Raspberry Pi..."

# Create or replace the fse folder on the Raspberry Pi
sshpass -e ssh eduardofarias@164.41.98.16 -p 13508 "
  if [ -d ~/fse ]; then
    rm -rf ~/fse
  fi
  mkdir ~/fse
"

echo "Sending the .bin folder to the Raspberry Pi..."

# Send the .bin folder to the Raspberry Pi
sshpass -e scp -P 13508 .bin/* eduardofarias@164.41.98.16:~/fse

echo "All binaries sent to the Raspberry Pi, use run.sh to run them"
