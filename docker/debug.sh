#!/bin/bash
export _RR_TRACE_DIR=~/traces
cd ~/workdir && rr record ~/pwnable && rr pack > /dev/null