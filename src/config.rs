use argparse::{ArgumentParser, Store, StoreTrue};

pub struct Config {
    pub train: bool,
    pub predict: bool,
    pub data_path: String,
    pub model_path: String,
    pub output_path: String,
}

impl Config {
    pub fn new() -> Self {
        Self {
            train: false,
            predict: false,
            data_path: String::new(),
            model_path: String::new(),
            output_path: String::new(),
        }
    }

    pub fn parse(&mut self) {
        let mut parser = ArgumentParser::new();
        {
            parser.set_description("Markov model for natural language generating.");
            parser
                .refer(&mut self.train)
                .add_option(&["--train"], StoreTrue, "train mode");
            parser
                .refer(&mut self.predict)
                .add_option(&["--predict"], StoreTrue, "predict mode");
            parser.refer(&mut self.data_path).add_option(
                &["-d", "--data_path"],
                Store,
                "dataset path",
            );
            parser.refer(&mut self.model_path).add_option(
                &["-m", "--model_path"],
                Store,
                "load model path",
            );
            parser.refer(&mut self.output_path).add_option(
                &["-o", "--output_path"],
                Store,
                "result output path",
            );
            parser.parse_args_or_exit();
        }
    }
}
