# computer

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
