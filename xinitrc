#!/bin/sh

target/release/minidesk-wm || target/debug/minidesk-wm &
xeyes &
exec xclock -geometry 200x200-0-0
