# Example reproducer script for gvisor issue

## Building

    docker build . -t crash

## Running

    # with gvisor
    $ docker run --rm --runtime=runsc crash
    Starting
    Crashing....
    thread 'main' panicked at /usr/local/cargo/registry/src/index.crates.io-6f17d22bba15001f/neli-0.6.4/src/rtnl.rs:103:32:
    range end index 37 out of range for slice of length 36
    note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

    # without gvisor
    $ docker run --rm crash
    Starting
    Crashing....
    Or not!

    # with gvisor but without setting the max log level
    $ docker run --rm --runtime=runsc crash
    Starting
    Crashing....
    Or not!
