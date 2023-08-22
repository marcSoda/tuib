pub mod disp;
use disp::{ Disp, Gamma, DispProp };

#[derive(Debug, Clone, Default)]
pub struct DispMgr {
    pub disps: Vec::<Disp>,
}

impl DispMgr {
    //TODO: error checking
    pub fn new() -> Self {
        //run bash command
        let (_ccode, cout, _cerr) = run_script::run_script!(
            r#"
                /bin/xrandr | /bin/grep " connected " | /bin/awk '{ print$1 }'
            "#
        ).unwrap();

        //split xrandr output into vector of display names
        let mut disp_names: Vec<String> = cout.split('\n')
            .map(str::to_string)
            .collect::<Vec<String>>();
        disp_names.retain(|i| !i.is_empty()); //remove empty entries (sometimes one left at the end)

        //create vector of displays
        let gamma = Gamma { r: 100, g: 100, b: 100};
        let brightness = 100;
        let mut disps: Vec<Disp> = Vec::new();
        for name in disp_names {
            disps.push(
                Disp::new(name, brightness, gamma),
            );
        }

        DispMgr {
            disps,
        }
    }

    pub fn get_disp_by_name(&mut self, name: String) -> &mut Disp {
        self.disps.iter_mut().find(|x| x.name == name).unwrap()
    }

    pub fn get_disp_by_index(&self, i: usize) -> Disp {
        self.disps[i].clone()
    }

    pub fn get_name_list(&self) -> Vec<&str> {
        self.disps.iter().map(|n| &n.name as &str).collect()
    }

    pub fn get_num_disps(&self) -> usize {
        self.disps.len()
    }

    pub fn set_value_by_name(&mut self, name: String, prop: DispProp, val: u8) {
        self.get_disp_by_name(name).set_value(prop, val);
    }

    pub fn set_value_by_index(&mut self, index: usize, prop: DispProp, val: u8) {
        self.disps[index].set_value(prop, val);
    }

    pub fn increment_value_by_index(&mut self, index: usize, prop: DispProp) {
        self.disps[index].increment_value(prop);
    }

    pub fn decrement_value_by_index(&mut self, index: usize, prop: DispProp) {
        self.disps[index].decrement_value(prop);
    }

    pub fn scale_value_by_index(&mut self, index: usize, prop: DispProp, scale: u8) {
        if scale > 0 && scale <= 10 {
            self.disps[index].set_value(prop, scale*10);
        } else {
            self.disps[index].set_value(prop, 100);
        }
    }

    pub fn reload(&mut self) {
        *self = DispMgr::new();
    }
}
