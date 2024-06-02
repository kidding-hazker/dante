# MiniDesk
An **overminimalistic** desktop environment made in _rust_.

The `minidesk` binary file contains the implementation of:
- Window manager
- Keyboard daemon
- Desktop and panels

Test run with Xephyr:
`cargo build --release && xinit ./xinitrc -- /usr/bin/Xephyr :100 -screen 800x600`.
Necessary `xinitrc` config:
``` sh
#!/bin/sh

target/release/minidesk &
exec xterm -geometry 96x24 -bg '#000' -cr '#fff' -fg '#fff'
```
