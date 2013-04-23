#!/bin/sh

source stop.sh

./guava&

echo $! > $PIDFILE
