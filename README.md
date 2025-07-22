# computer-statistics

Sends statistics of my computer to my [API](https://github.com/trafficlunar/api) that can be seen on my [website](https://github.com/trafficlunar/website)

## features

- CPU usage
- Memory usage
- Key press counter
- Left and right mouse clicks counter

## running without sudo

```bash
$ sudo nano /etc/udev/rules.d/99-inputbot.rules
```

Copy and paste this below and save the file.

```
KERNEL=="event*", SUBSYSTEM=="input", TAG+="uaccess"
KERNEL=="mouse*", SUBSYSTEM=="input", TAG+="uaccess"
KERNEL=="kbd*", SUBSYSTEM=="input", TAG+="uaccess"
```

Restart your computer.

## systemd service

```bash
$ nano /etc/systemd/system/computer-statistics.service
```

```
[Unit]
Description=Computer statistics client
After=network.target

[Service]
Type=simple
Restart=always
WorkingDirectory=%h/Projects/computer-statistics
ExecStart=%h/Projects/computer-statistics/target/release/computer
Environment="RUST_LOG=info"

[Install]
WantedBy=multi-user.target
```

```bash
$ cargo build --release
$ systemctl --user daemon-reload
$ systemctl --user enable --now computer-statistics.service
$ systemctl --user status computer-statistics.service
```
