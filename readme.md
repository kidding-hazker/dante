# MiniDesk
An **overminimalistic** desktop environment made in _rust_.
Test run with Xephyr: `cargo build && xinit ./xinitrc -- /usr/bin/Xephyr :100 -screen 800x600`.
Necessary `xinitrc` config:
``` sh
#!/bin/sh

target/release/minidesk-wm || target/debug/minidesk-wm &
xeyes &
xterm &
exec xclock -geometry 200x200-0-0
```
