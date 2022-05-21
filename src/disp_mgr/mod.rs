pub mod disp;
use disp::{ Disp, Gamma, DispProp };

#[derive(Debug)]
pub struct DispMgr {
    pub disps: Vec::<Disp>,
}

impl DispMgr {
    //TODO: error checking
    pub fn new() -> Self {
        //run bash command
        let (ccode, mut cout, cerr) = run_script::run_script!(
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
        let brightness = 1;
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

    pub fn set_value(&mut self, name: String, prop: DispProp, val: u8) {
        let disp = self.get_disp_by_name(name);
        disp.set_value(prop, val);
    }

    pub fn get_disp_by_name<'a>(&'a mut self, name: String) -> &'a mut Disp {
        //TODO: error checking
        self.disps.iter_mut().find(|x| x.name == name).unwrap()
    }

    pub fn reload(&mut self) {
        *self = DispMgr::new();
    }
}
