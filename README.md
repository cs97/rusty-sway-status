# rusty-sway-status

### Installing
* cargo build --release
* cp target/release/status /usr/bin/status

~/.config/sway/config
```
# Read `man 5 sway-bar` for more information about this section.
bar {
    position top

    # When the status_command prints a new line to stdout, swaybar updates.
    # The default just shows the current date and time.
    #status_command while date +'%Y-%m-%d %I:%M:%S %p'; do sleep 1; done
    #status_command while date +'%Y-%m-%d %H:%M:%S %Z'; do sleep 1; done
    status_command while /usr/bin/status || echo "loading..."; do sleep 1; done

    colors {
        statusline #ffffff
        background #323232
        inactive_workspace #32323200 #32323200 #5c5c5c
    }
}

```¸
