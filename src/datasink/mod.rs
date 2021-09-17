use gst::glib;
use gst::prelude::*;

mod imp;

// The public Rust wrapper type for our element
glib::wrapper! {
    pub struct DataSink(ObjectSubclass<imp::DataSink>) @extends gst_base::BaseSink, gst::Element, gst::Object;
}

// GStreamer elements need to be thread-safe. For the private implementation this is automatically
// enforced but for the public wrapper type we need to specify this manually.
unsafe impl Send for DataSink {}
unsafe impl Sync for DataSink {}

pub fn register(plugin: &gst::Plugin) -> Result<(), glib::BoolError> {
    gst::Element::register(
        Some(plugin),
        "datasink",
        gst::Rank::None,
        DataSink::static_type(),
    )
}
