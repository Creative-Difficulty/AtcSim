#![allow(dead_code)]

#[derive(Clone, Debug)]
pub struct Aircraft {
    pub callsign: String,
    pub curr_heading: Heading,
    pub ass_heading: Heading,
    pub altitude_ft: u16,
    pub indicated_speed_kts: u16,
    pub ground_speed_kts: u16,
    /// Total aircraft mass in KG rounded to the nearest whole number
    pub mass: u128,
}

impl Aircraft {
    pub fn change_heading(&mut self, new_heading: Heading) {
        self.curr_heading = new_heading;
    }

    //TODO? optimize memory  usage
    pub fn get_energy(&self) -> f64 {
        // E_kin=(m*v^2)/2
        // E_pot=m*g*h
        // E=E_kin+E_pot
        let e_pot: f64 = self.mass as f64 * 9.81 * self.altitude_ft as f64;
        let e_kin: f64 = (self.mass as f64 * self.indicated_speed_kts.pow(2) as f64) / 2_f64;
        e_pot + e_kin
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
