# skynet

listen to keyboard events, save them to `/tmp/key.log`.

## Usage:

### docker

`docker build -t skynet .`

```

docker run -d -v /tmp:/tmp/ \
      --userns=host  \
      --privileged -v \
      /dev/input/by-path/platform-i8042-serio-0-event-kbd:/dev/input/by-path/platform-i8042-serio-0-event-kbd \
      skynet

```

`watch -n 1 less /tmp/key.log`
