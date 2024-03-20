use std::{
    io::{self, Write},
    path::Path,
};

use faust_types::{FaustDsp, UI};
include!("src/dsp.rs");

#[derive(Debug)]
#[allow(unused)]
enum Param {
    Normal {
        label: String,
        param: i32,
        init: f64,
        min: f64,
        max: f64,
        step: f64,
    },
}

#[derive(Debug)]
struct CollectParameters {
    collected: Vec<Param>,
}

impl CollectParameters {
    pub fn new() -> Self {
        Self { collected: vec![] }
    }

    pub fn write_nih_params_struct(&self, to: &Path, struct_name: &str) -> io::Result<()> {
        let mut file = std::fs::File::create(to)?;
        let mut content = "/// This file is auto generated by the build.rs file.\n".to_string();
        content += "use faust_types::ParamIndex;\n";
        content += &format!("#[derive(Params)]\nstruct {} {{\n", struct_name);
        content += &format!("// nr of params: {}\n", self.collected.len());
        for (index, parameter) in self.collected.iter().enumerate() {
            // println!("self.collected.len() {}",self.collected.len());
            // println!("*******************************************************************************************************************");
            let is_last = index == self.collected.len() - 1;
            match parameter {
                Param::Normal { label, .. } => {
                    content += &format!("    #[id = \"{}\"]\n", label);
                    content += &format!("    {}: FloatParam", label.to_lowercase());
                    content += &format!("{}\n", if is_last { "" } else { "," });
                }
            }
        }
        content += "}\n\n";
        content += &format!("impl Default for {} {{\n", struct_name);
        content += "    fn default() -> Self {\n";
        content += "        Self {\n";
        for (index, parameter) in self.collected.iter().enumerate() {
            let is_last = index == self.collected.len() - 1;
            match parameter {
                Param::Normal {
                    label,
                    init,
                    min,
                    max,
                    ..
                } => {
                    // TODO: Properly format floats, {:.01} is just a hack
                    content += &format!("            {}: FloatParam::new(\"{}\", {:.01}, FloatRange::Linear {{ min: {:.01}, max: {:.01}}})", label.to_lowercase(), label, init, min, max);
                    content += &format!("{}\n", if is_last { "" } else { "," });
                }
            }
        }

        content += "        }\n";
        content += "    }\n";
        content += "}\n";

        content += "\n";

        for parameter in &self.collected {
            match parameter {
                Param::Normal { label, param, .. } => {
                    content += &format!(
                        "pub const {}_PI: ParamIndex = ParamIndex({});\n",
                        label.to_uppercase(),
                        param
                    );
                }
            }
        }

        file.write_all(content.as_bytes())?;
        Ok(())
    }
}

impl UI<f64> for CollectParameters {
    fn open_tab_box(&mut self, _label: &str) {}

    fn open_horizontal_box(&mut self, _label: &str) {}

    fn open_vertical_box(&mut self, _label: &str) {}

    fn close_box(&mut self) {}

    fn add_button(&mut self, _label: &str, _param: faust_types::ParamIndex) {}

    fn add_check_button(&mut self, _label: &str, _param: faust_types::ParamIndex) {}

    fn add_vertical_slider(
        &mut self,
        label: &str,
        param: faust_types::ParamIndex,
        init: f64,
        min: f64,
        max: f64,
        step: f64,
    ) {
        self.collected.push(Param::Normal {
            label: label.to_string(),
            param: param.0,
            init,
            min,
            max,
            step,
        })
    }

    fn add_horizontal_slider(
        &mut self,
        label: &str,
        param: faust_types::ParamIndex,
        init: f64,
        min: f64,
        max: f64,
        step: f64,
    ) {
        self.collected.push(Param::Normal {
            label: label.to_string(),
            param: param.0,
            init,
            min,
            max,
            step,
        })
    }

    fn add_num_entry(
        &mut self,
        label: &str,
        param: faust_types::ParamIndex,
        init: f64,
        min: f64,
        max: f64,
        step: f64,
    ) {
        self.collected.push(Param::Normal {
            label: label.to_string(),
            param: param.0,
            init,
            min,
            max,
            step,
        })
    }

    fn add_horizontal_bargraph(
        &mut self,
        _label: &str,
        _param: faust_types::ParamIndex,
        _min: f64,
        _max: f64,
    ) {
    }

    fn add_vertical_bargraph(
        &mut self,
        _label: &str,
        _param: faust_types::ParamIndex,
        _min: f64,
        _max: f64,
    ) {
    }

    fn declare(&mut self, _param: Option<faust_types::ParamIndex>, _key: &str, _value: &str) {}
}

fn main() {

    println!("cargo:rerun-if-changed=dsp");

    #[cfg(feature = "faust-rebuild")]

    faust_build::FaustBuilder::new("dsp/lamb.dsp", "src/dsp.rs")
        .set_use_double(true)
        .build();


    // faust_build::build_dsp_to_destination("dsp/lamb.dsp", "src/dsp.rs");

    // println!("cargo:rerun-if-changed=dsp");
    // let mut my_ui = CollectParameters::new();
    // dsp::mydsp::build_user_interface_static(&mut my_ui);
    // my_ui
    // .write_nih_params_struct(Path::new("src/params_auto.rs"), "LambParams")
    // .expect("Failed writing nih params");
}
