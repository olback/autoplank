# Autoplank

Automatically move plank to the monitor your mouse cursor is currently on.

Build the program or download the latest version from [releases](https://github.com/olback/autoplank/releases).
```
cargo build --release
```

Copy the binary to `~/.local/bin/autoplank`:
```
cp target/release/autoplank ~/.local/bin/autoplank
```

Create a autostart file: `~/.config/autostart/autoplank.desktop`
```
[Desktop Entry]
Type=Application
Name=Autoplank
Description=Move plank to the monitor your mouse is at.
Exec=.local/bin/autoplank
```

When adding/removing a monitor make sure to rescan:
```terminal
autoplank -r
```

Inspired by [abiosoft/autoplank](https://github.com/abiosoft/autoplank).
