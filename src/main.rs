mod config;
mod model;
mod slicer;
mod typo;

use typo::Context;

use crate::config::Config;
use crate::model::Model;

use std::fs;

const OUTPUT_BUFFER_SIZE: usize = 1024;

fn main() {
    let mut cfg = Config::new();
    cfg.parse();

    let mut model = Model::new();

    if cfg.train {
        let buf = fs::read(cfg.data_path).unwrap();
        model.train(&buf);

        if cfg.output_path.len() != 0 {
            model.save_model(&cfg.output_path);
        }
    }

    if cfg.model_path.len() != 0 {
        model.load_model(&cfg.model_path);
    }

    if cfg.predict {
        let mut buf = Vec::new();
        let mut context = 0 as Context;
        while let Some(c) = model.predict(&mut context) {
            if buf.len() >= OUTPUT_BUFFER_SIZE {
                break;
            }
            buf.push(c);
        }

        let str_buf = String::from_utf8(buf).unwrap();
        println!("{}", str_buf);
    }
}
