# Autoplank

Automatically move plank to the monitor your mouse cursor is currently on.

Build the program or download the latest version from [releases](https://github.com/olback/autoplank/releases). If you download a pre-built binary, don't forget to make it executable!
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
Description=Automatically move plank across monitors
Exec=.local/bin/autoplank
```

optionally set your own polling rate like this (default: 500):
```
...
Exec=.local/bin/autoplank -p 200
```

When adding/removing a monitor make sure to rescan:
```terminal
autoplank -r
```

#### Elementary OS  
Elementary has a service called `cerbere` which handles wingpanel and plank. To make autoplank work properly, you might have to run `autoplank --elementary-fix`. You can revert this change with `autoplank --elementary-restore`. For these commands to take affect, you have to restart/log out and in.

Inspired by [abiosoft/autoplank](https://github.com/abiosoft/autoplank).
