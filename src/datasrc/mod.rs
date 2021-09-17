use gst::glib;
use gst::prelude::*;

mod imp;

// The public Rust wrapper type for our element
glib::wrapper! {
    pub struct DataSrc(ObjectSubclass<imp::DataSrc>) @extends gst_base::BaseSrc, gst::Element, gst::Object;
}

// GStreamer elements need to be thread-safe. For the private implementation this is automatically
// enforced but for the public wrapper type we need to specify this manually.
unsafe impl Send for DataSrc {}
unsafe impl Sync for DataSrc {}

pub fn register(plugin: &gst::Plugin) -> Result<(), glib::BoolError> {
    gst::Element::register(
        Some(plugin),
        "datasrc",
        gst::Rank::None,
        DataSrc::static_type(),
    )
}
