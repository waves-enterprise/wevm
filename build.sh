#!/bin/bash

mkdir -p /root/.sbt
touch /root/.sbt/.credentials
echo "$SBT_CREDENTIALS" > /root/.sbt/.credentials
unset SBT_CREDENTIALS
sbt -mem 2048 "clean; buildWAT; test; assembly; publish"