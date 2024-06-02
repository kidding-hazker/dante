#!/bin/sh

target/release/minidesk &
exec xterm -geometry 96x24 -bg '#000' -cr '#fff' -fg '#fff'
