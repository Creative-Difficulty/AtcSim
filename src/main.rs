use atcsim::structs::{self, *};
fn main() {
    let heading = Heading::new(180.0).expect("Invalid heading");

    let mut airfcra = Aircraft {
        altitude: 20000,
        callsign: "Delta 123".into(),
        curr_heading: heading.clone(),
        ass_heading: heading.clone(),
    };

    airfcra.change_heading(heading);
}
