use std::process::Command;


#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DispProp {
    Brightness,
    R,
    G,
    B,
}

impl DispProp {
    pub fn next(&mut self) {
        *self = match self {
            DispProp::Brightness => DispProp::R,
            DispProp::R          => DispProp::G,
            DispProp::G          => DispProp::B,
            DispProp::B          => DispProp::Brightness,
        };
    }

    pub fn prev(&mut self) {
        *self = match self {
            DispProp::Brightness => DispProp::B,
            DispProp::R          => DispProp::Brightness,
            DispProp::G          => DispProp::R,
            DispProp::B          => DispProp::G,
        };
    }
}
//implement next and prev for this enum to use in ui

#[derive(Debug, Clone, Copy)]
pub struct Gamma {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Gamma {
    pub fn to_gamma_string(self, prop: &DispProp, val: u8) -> String {
        let mut fr = ((self.r as f32) / 100.0).to_string();
        let mut fg = ((self.g as f32) / 100.0).to_string();
        let mut fb = ((self.b as f32) / 100.0).to_string();
        match prop {
            DispProp::R => fr = ((val as f32) / 100.0).to_string(),
            DispProp::G => fg = ((val as f32) / 100.0).to_string(),
            DispProp::B => fb = ((val as f32) / 100.0).to_string(),
            _ => {}
        };
        fr + ":" + &fg + ":" + &fb
    }
}

#[derive(Debug, Clone)]
pub struct Disp {
    pub name: String,
    pub brightness: u8,
    pub gamma: Gamma,
}

impl Disp {
    pub fn new(name: String, brightness: u8, gamma: Gamma) -> Self {
        Disp {
            name,
            brightness,
            gamma,
        }
    }

    fn get_brightness(&self) -> String {
        ((self.brightness as f32) / 100.0).to_string()
    }

    pub fn increment_value(&mut self, prop: DispProp) {
        let val = match prop {
            DispProp::Brightness => self.brightness + 1,
            DispProp::R => self.gamma.r + 1,
            DispProp::G => self.gamma.g + 1,
            DispProp::B => self.gamma.b + 1,
        };
        self.set_value(prop, val);
    }

    pub fn decrement_value(&mut self, prop: DispProp) {
        let val = match prop {
            DispProp::Brightness => self.brightness - 1,
            DispProp::R => self.gamma.r - 1,
            DispProp::G => self.gamma.g - 1,
            DispProp::B => self.gamma.b - 1,
        };
        self.set_value(prop, val);
    }

    pub fn set_value(&mut self, prop: DispProp, mut val: u8) {
        //TODO: error checking
        val = val.clamp(1, 100);
        let mut new_disp = Disp::new(self.name.clone(), self.brightness, self.gamma);
        match prop {
            DispProp::R => new_disp.gamma.r = val,
            DispProp::G => new_disp.gamma.g = val,
            DispProp::B => new_disp.gamma.b = val,
            DispProp::Brightness => new_disp.brightness = val,
        };

        let _out = Command::new("/bin/xrandr")
            .arg("--output")
            .arg(&new_disp.name)
            .arg("--brightness")
            .arg(new_disp.get_brightness())
            .arg("--gamma")
            .arg(new_disp.gamma.to_gamma_string(&prop, val))
            .output()
            .expect("failed to execute process");

        *self = new_disp;
    }
}
