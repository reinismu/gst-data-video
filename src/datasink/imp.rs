use bytes::Buf;
use gst::glib;
use gst::prelude::*;
use gst::subclass::prelude::*;
use gst::{gst_debug, gst_info};
use gst_base::subclass::prelude::*;
use gst_video::subclass::prelude::VideoSinkImpl;

use std::i32;

use once_cell::sync::Lazy;

use crate::encoding::convert_back_with_0_and_255;
use crate::encoding::convert_from_sdi_safe_payload;
use crate::encoding::MAGIC_NUMBER;

static CAT: Lazy<gst::DebugCategory> = Lazy::new(|| {
    gst::DebugCategory::new(
        "datasink",
        gst::DebugColorFlags::empty(),
        Some("Read data built by DataSrc"),
    )
});

const SIGNAL_DATA_RECEIVED: &str = "data-received";

#[derive(Default)]
pub struct DataSink {}

impl DataSink {}

#[glib::object_subclass]
impl ObjectSubclass for DataSink {
    const NAME: &'static str = "DataSink";
    type Type = super::DataSink;
    type ParentType = gst_video::VideoSink;
}

impl ObjectImpl for DataSink {
    fn signals() -> &'static [glib::subclass::Signal] {
        static SIGNALS: Lazy<Vec<glib::subclass::Signal>> = Lazy::new(|| {
            vec![glib::subclass::Signal::builder(
                SIGNAL_DATA_RECEIVED,
                &[String::static_type().into()],
                glib::types::Type::UNIT.into(),
            )
            // .action()
            .build()]
        });

        SIGNALS.as_ref()
    }
}

impl ElementImpl for DataSink {
    fn metadata() -> Option<&'static gst::subclass::ElementMetadata> {
        static ELEMENT_METADATA: Lazy<gst::subclass::ElementMetadata> = Lazy::new(|| {
            gst::subclass::ElementMetadata::new(
                "Data Sink",
                "Sink/Data",
                "Read data from a video stream",
                "Reinis Muižnieks <muiznieks.reinis@gmail.com>",
            )
        });

        Some(&*ELEMENT_METADATA)
    }

    fn pad_templates() -> &'static [gst::PadTemplate] {
        static PAD_TEMPLATES: Lazy<Vec<gst::PadTemplate>> = Lazy::new(|| {
            let caps = gst::Caps::new_simple(
                "video/x-raw",
                &[
                    (
                        "format",
                        &gst::List::new(&[
                            &gst_video::VideoFormat::Uyvy.to_str(),
                            &gst_video::VideoFormat::Argb.to_str(),
                            &gst_video::VideoFormat::Bgra.to_str(),
                        ]),
                    ),
                    ("width", &gst::IntRange::<i32>::new(1920, i32::MAX)),
                    ("height", &gst::IntRange::<i32>::new(1080, i32::MAX)),
                    (
                        "framerate",
                        &gst::FractionRange::new(
                            gst::Fraction::new(0, 1),
                            gst::Fraction::new(i32::MAX, 1),
                        ),
                    ),
                ],
            );

            let sink_pad_template = gst::PadTemplate::new(
                "sink",
                gst::PadDirection::Sink,
                gst::PadPresence::Always,
                &caps,
            )
            .unwrap();

            vec![sink_pad_template]
        });
        PAD_TEMPLATES.as_ref()
    }
}

impl BaseSinkImpl for DataSink {
    // Called when starting, so we can initialize all stream-related state to its defaults
    fn start(&self, element: &Self::Type) -> Result<(), gst::ErrorMessage> {
        gst_info!(CAT, obj: element, "Started");
        Ok(())
    }

    // Called when shutting down the element so we can release all stream-related state
    fn stop(&self, element: &Self::Type) -> Result<(), gst::ErrorMessage> {
        gst_info!(CAT, obj: element, "Stopped");

        Ok(())
    }

    fn set_caps(&self, element: &Self::Type, caps: &gst::Caps) -> Result<(), gst::LoggableError> {
        gst_debug!(CAT, obj: element, "Configuring for caps {}", caps);

        let _ = element.post_message(gst::message::Latency::builder().src(element).build());

        Ok(())
    }
}

impl VideoSinkImpl for DataSink {
    fn show_frame(
        &self,
        element: &Self::Type,
        buffer: &gst::Buffer,
    ) -> Result<gst::FlowSuccess, gst::FlowError> {
        let map = buffer.map_readable().unwrap();

        let mut data = map.as_slice();

        let number = data.get_u32();

        if number != MAGIC_NUMBER {
            return Ok(gst::FlowSuccess::Ok);
        }

        let length = convert_back_with_0_and_255(data.get_u32()) as usize;

        if length > buffer.size() || length == 0 {
            return Ok(gst::FlowSuccess::Ok);
        }

        let raw_content = convert_from_sdi_safe_payload(&data[..length]);
        let content = std::str::from_utf8(&raw_content).unwrap().to_string();

        gst_info!(CAT, obj: element, "Got content {:?}", content);

        element
            .emit_by_name(SIGNAL_DATA_RECEIVED, &[&content])
            .unwrap();

        Ok(gst::FlowSuccess::Ok)
    }
}
