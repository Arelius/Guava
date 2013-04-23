#!/bin/sh

PIDFILE=tmp/guava.pid
if [ -f $PIDFILE ]; then
    PID=$(cat $PIDFILE)
    kill $PID

    rm $PIDFILE
fi
