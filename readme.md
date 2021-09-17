# Plugins to allow communicating data over video

## Example usage

-   `cargo build --release`
-   `export GST_PLUGIN_PATH=$(pwd)/target/release`
-   `GST_DEBUG=datasink:4 gst-launch-1.0 datasrc ! video/x-raw,framerate=25/1,width=1920,height=1080 ! datasink`
