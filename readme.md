# MiniDesk
An **overminimalistic** desktop environment made in _rust_.
Test run with Xephyr: `cargo build && xinit ./xinitrc -- /usr/bin/Xephyr :100 -screen 800x600`.
Necessary `xinitrc` config:
``` sh
#!/bin/sh

usr/bin/minidesk &
xclock &
# ... running your custom apps
exec xterm
```
