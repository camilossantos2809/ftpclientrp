extern crate confy;

use std::default::Default;

#[derive(Serialize, Deserialize, Debug)]
pub struct Data {
    pub dir: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub erp: Data,
    pub erp_alpha: Data,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            erp: Data {
                dir: "/tmp/".to_string(),
            },
            erp_alpha: Data {
                dir: "/tmp/".to_string(),
            },
        }
    }
}

pub fn get() -> Result<Config, confy::ConfyError> {
    let cfg: Config = confy::load_path("config.toml")?;
    // println!("{:#?}", cfg);
    Ok(cfg)
}

pub fn store(conf: Config) {
    confy::store_path("config.toml", conf).unwrap();
}
