use glib::Error;
use gst::prelude::*;

fn create_pipeline() -> Result<gst::Pipeline, Error> {
    gst::init().unwrap();
    gst::debug_set_default_threshold(gst::DebugLevel::Error);

    let pipeline = gst::Pipeline::new(None);

    let datasrc = gst::ElementFactory::make("datasrc", None).unwrap();

    let video_filter = gst::ElementFactory::make("capsfilter", None).unwrap();
    video_filter
        .set_property(
            "caps",
            &gst::Caps::builder("video/x-raw")
                .field("width", &1920)
                .field("height", &1080)
                .field("framerate", &gst::Fraction::new(25, 1))
                .build(),
        )
        .unwrap();

    let sink = gst::ElementFactory::make("decklinkvideosink", None).unwrap();
    sink.set_property_from_str("device-number", "1");
    sink.set_property_from_str("mode", "1080p25");

    pipeline
        .add_many(&[&datasrc, &video_filter, &sink])
        .unwrap();

    gst::Element::link_many(&[&datasrc, &video_filter, &sink]).unwrap();

    Ok(pipeline)
}

fn main() -> Result<(), Error> {
    let pipeline = create_pipeline().unwrap();

    pipeline.set_state(gst::State::Playing).unwrap();

    let bus = pipeline.bus().unwrap();

    for msg in bus.iter_timed(gst::ClockTime::NONE) {
        match msg.view() {
            gst::MessageView::Error(err) => println!(
                "Error received {:?}: {}",
                err.src().map(|s| String::from(s.path_string())),
                err.error(),
            ),
            gst::MessageView::StateChanged(state) => println!(
                "State changed from {:?} to {:?}, {:?}",
                state.old(),
                state.current(),
                state.src()
            ),
            gst::MessageView::Latency(latency) => println!("Latency {:?}", latency.src()),
            gst::MessageView::Element(element) => {
                println!("Element: {:?}", element);
            }
            _ => {}
        }
    }

    Ok(())
}
