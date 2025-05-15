#![allow(dead_code)]

#[derive(Clone, Debug)]
pub struct Aircraft {
    pub callsign: String,
    pub curr_heading: Heading,
    pub ass_heading: Heading,
    pub altitude: u16,
}

impl Aircraft {
    pub fn change_heading(&mut self, new_heading: Heading) {
        self.curr_heading = new_heading;
    }
}

#[derive(Clone, Debug)]
pub struct Heading(f32);

impl Heading {
    pub fn validate(&self) -> bool {
        (&0.0..=&360.0).contains(&&self.0)
    }

    pub fn new<T>(value: T) -> Option<Heading>
    where
        T: Into<f32>,
    {
        let value = value.into();
        if (0.0..=360.0).contains(&value) {
            Some(Heading(value))
        } else {
            None
        }
    }
}
