# rust-iokit

Higher level wrapper for macOS/iPadOS IOKit framework than iokit_sys crate

## Status

This is an early work in progress. Currently you can only sensibly do this:

```
cargo run --example usb-device-match-notifications
```

This will print the I/O registry paths of all connected USB devices and continues monitoring for newly plugged ones.
I/O registry paths for any hot-plugged devices will printed.

Exit with ctrl-c.
