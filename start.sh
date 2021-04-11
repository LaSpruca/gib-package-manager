#!/bin/sh
cd gib-server || exit
echo Running diesel
diesel setup
diesel migration run
../target/release/gib-server