# sway-inactive-window-transparency

Make your unfocused window more transparent in sway (i3 compatible, but not tested).

## Build

With Nix:
```sh
nix build
```

With Cargo:
```sh
cargo build --release
```

## Usage

You need to propagate `I3SOCK` environment variable, which is same to `SWAYSOCK` variable value.

If use this with systemd user services, you may have to add line below at your sway config.
```
exec "systemctl --user import-environment {,WAYLAND_}DISPLAY SWAYSOCK I3SOCK"
```

Then just write systemd user service like below. Replace `sway-session.target` as you want.
```
[Install]
WantedBy=sway-session.target

[Service]
ExecStart=/path/to/your/sway-inactive-window-transparency
Type=simple

[Unit]
BindsTo=sway-session.target
Description=Set inactive window transparency on sway
```

... Or just execute directly from your shell.
```sh
./sway-inactive-window-transparency
```

## Configuration

Set your additional environment variable as:

* `INACTIVE_OPACITY`: opacity of inactive window. Default is `0.8`.
