#!/bin/bash

if [ ! -f ~/pwnable ]
then
    echo "Pwnable not found!"
    exit 1
fi
if [ ! -d ~/workdir ]
then 
    echo "Workdir not found"
    exit 1
fi
if [ -S ~/sockets/run.sock ]
then
    rm sockets/run.sock
fi
if [ -S ~/sockets/debug.sock ]
then
    rm sockets/debug.sock
fi

socat UNIX-LISTEN:sockets/run.sock,fork,reuseaddr EXEC:./execute.sh,pty,echo=0,raw,iexten=0,stderr &\
socat UNIX-LISTEN:sockets/debug.sock,fork,reuseaddr EXEC:./debug.sh,pty,echo=0,raw,iexten=0,stderr