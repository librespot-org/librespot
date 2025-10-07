use symphonia::core::meta::Metadata;
use symphonia::core::probe::ProbeResult;

pub fn get_latest_metadata(probe_result: &mut ProbeResult) -> Option<Metadata> {
    let mut metadata = probe_result.format.metadata();

    // If we can't get metadata from the container, fall back to other tags found by probing.
    // Note that this is only relevant for local files.
    if metadata.current().is_none() {
        if let Some(inner_probe_metadata) = probe_result.metadata.get() {
            metadata = inner_probe_metadata;
        }
    }

    // Advance to the latest metadata revision.
    // None means we hit the latest.
    loop {
        if metadata.pop().is_none() {
            break;
        }
    }

    Some(metadata)
}
