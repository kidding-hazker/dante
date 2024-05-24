#!/bin/sh

target/debug/minidesk || target/release/minidesk &
xeyes &
exec xclock -geometry 200x200-0-0
