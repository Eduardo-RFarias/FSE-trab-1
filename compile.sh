#!/bin/sh

# Create folder to store the compiled binaries, if it doesn't exist
if [ ! -d ".bin" ]; then
  mkdir .bin
fi

# Clean the .bin folder
rm -rf .bin/*

# Cross-Compile all four projects using cross and the armv7-unknown-linux-musleabihf target
cd server
cross build --release --target armv7-unknown-linux-musleabihf
cp target/armv7-unknown-linux-musleabihf/release/fse_trab_1_server ../.bin/fse_trab_1_server

cd ../app
cross build --release --target armv7-unknown-linux-musleabihf
cp target/armv7-unknown-linux-musleabihf/release/fse_trab_1_app ../.bin/fse_trab_1_app

cd ../ground_floor
cross build --release --target armv7-unknown-linux-musleabihf
cp target/armv7-unknown-linux-musleabihf/release/fse_trab_1_ground_floor ../.bin/fse_trab_1_ground_floor

cd ../first_floor
cross build --release --target armv7-unknown-linux-musleabihf
cp target/armv7-unknown-linux-musleabihf/release/fse_trab_1_first_floor ../.bin/fse_trab_1_first_floor

cd ../second_floor
cross build --release --target armv7-unknown-linux-musleabihf
cp target/armv7-unknown-linux-musleabihf/release/fse_trab_1_second_floor ../.bin/fse_trab_1_second_floor

echo "All projects compiled successfully, binaries are in the .bin folder"