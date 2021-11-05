# Plugins to allow communicating data over video (Compatible with SDI)

## How it works?

`datasrc` waits for `send-data` action and sends it down the line.
`datasink` reads each incoming frame, checks Magic number, decodes content and signals it. (Check _examples/data-from-decklink.rs_)

## Example usage

-   `cargo build --release`
-   `export GST_PLUGIN_PATH=$(pwd)/target/release`
-   `GST_DEBUG=datasink:4 gst-launch-1.0 datasrc ! video/x-raw,framerate=25/1,width=1920,height=1080 ! datasink`

-   `cargo run --example src-to-sink`

## Notes

-   0 and 256 bits are used for sdi synchronization so we cant use them. (https://forum.blackmagicdesign.com/viewtopic.php?f=12&t=147543#p791374)
