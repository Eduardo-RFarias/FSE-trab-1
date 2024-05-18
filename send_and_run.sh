#!/bin/sh

# Check if the .bin folder exists and if all the binaries are there
if [ ! -d ".bin" ] || [ ! -f ".bin/fse_trab_1_server" ] || [ ! -f ".bin/fse_trab_1_ground_floor" ] || [ ! -f ".bin/fse_trab_1_first_floor" ] || [ ! -f ".bin/fse_trab_1_second_floor" ]; then
  echo "Please run compile.sh before running this script"
  exit 1
fi

# Send the .bin folder to the Raspberry Pi
scp -P 13508 -r .bin eduardofarias@164.41.98.28:~/

ssh -p 13508 eduardofarias@164.41.98.28 "nohup ~/.bin/fse_trab_1_server > /dev/null 2>&1 &"
ssh -p 13508 eduardofarias@164.41.98.28 "nohup ~/.bin/fse_trab_1_ground_floor > /dev/null 2>&1 &"
ssh -p 13508 eduardofarias@164.41.98.28 "nohup ~/.bin/fse_trab_1_first_floor > /dev/null 2>&1 &"
ssh -p 13508 eduardofarias@164.41.98.28 "nohup ~/.bin/fse_trab_1_second_floor > /dev/null 2>&1 &"

echo "All binaries sent and running on the Raspberry Pi, use htop to kill them if needed (SIGINT)"
