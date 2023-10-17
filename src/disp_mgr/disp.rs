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

    // TODO: this will take a gamma string from xrandr and parse it as relative percentages.
    // it won't be exact, but it will be close enough
    // this is for when you update tuib to read the gamma values from xrandr instead of assuming they're all 100%
    // example gamma string: 1.3:1.7:2.5
    // this was taken when r was 80%, green was 60%, and blue was 40%
    // if you input '1.3:1.7:2.5' into this function, it will report 76 58 40, which is not exactly right, but close enough
    pub fn from_gamma_string(&mut self, gamma_str: &str) {
        let values: Vec<&str> = gamma_str.split(':').collect();
        if values.len() != 3 {
            println!("Invalid gamma string format");
        }

        self.r = (100.0 / values[0].parse::<f32>().unwrap_or(1.0)) as u8;
        self.g = (100.0 / values[1].parse::<f32>().unwrap_or(1.0)) as u8;
        self.b = (100.0 / values[2].parse::<f32>().unwrap_or(1.0)) as u8;
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
