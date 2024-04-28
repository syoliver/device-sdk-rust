use std::env;
use url::Url;
use std::path::Path;
use serde::{Deserialize, Deserializer, de};
use serde_yaml;
use std::ffi::{OsStr, OsString};
use std::error::Error;
use super::os_string_conversion_error::OsStringConversionError;
use std::fmt;
use std::collections::HashMap;

pub enum Configuration {
    Leaf(String),
    Node(HashMap<String, Configuration>),
}

impl<'de> Deserialize<'de> for Configuration {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de> {

            struct ConfigurationVisitor;

            impl<'de> de::Visitor<'de> for ConfigurationVisitor {
                type Value = Configuration;

                fn expecting(&self, _formatter: &mut fmt::Formatter) -> fmt::Result {
                    Ok(())
                }

                fn visit_map<V>(self, mut map: V) -> Result<Configuration, V::Error>
                where
                    V: de::MapAccess<'de>,
                {
                    let mut config: HashMap<String, Configuration> = HashMap::new();
                    while let Some(key) = map.next_key()? {
                        let value = map.next_value()?;
                        config.insert(key, value);
                    }
                    Ok(Configuration::Node(config))
                }

                fn visit_string<E>(self, value: String) -> Result<Configuration, E>
                where
                    E: de::Error,
                {
                    Ok(Configuration::Leaf(value))
                }

                fn visit_str<E>(self, value: &str) -> Result<Configuration, E>
                where
                    E: de::Error,
                {
                    self.visit_string(value.to_string())
                }

                fn visit_bool<E>(self, value: bool) -> Result<Configuration, E>
                where
                    E: de::Error,
                {
                    self.visit_string(value.to_string())
                }

                fn visit_i8<E>(self, value: i8) -> Result<Configuration, E>
                where
                    E: de::Error,
                {
                    self.visit_string(value.to_string())
                }

                fn visit_i16<E>(self, value: i16) -> Result<Configuration, E>
                where
                    E: de::Error,
                {
                    self.visit_string(value.to_string())
                }

                fn visit_i32<E>(self, value: i32) -> Result<Configuration, E>
                where
                    E: de::Error,
                {
                    self.visit_string(value.to_string())
                }

                fn visit_i64<E>(self, value: i64) -> Result<Configuration, E>
                where
                    E: de::Error,
                {
                    self.visit_string(value.to_string())
                }

                fn visit_i128<E>(self, value: i128) -> Result<Configuration, E>
                where
                    E: de::Error,
                {
                    self.visit_string(value.to_string())
                }



                fn visit_u8<E>(self, value: u8) -> Result<Configuration, E>
                where
                    E: de::Error,
                {
                    self.visit_string(value.to_string())
                }

                fn visit_u16<E>(self, value: u16) -> Result<Configuration, E>
                where
                    E: de::Error,
                {
                    self.visit_string(value.to_string())
                }

                fn visit_u32<E>(self, value: u32) -> Result<Configuration, E>
                where
                    E: de::Error,
                {
                    self.visit_string(value.to_string())
                }

                fn visit_u64<E>(self, value: u64) -> Result<Configuration, E>
                where
                    E: de::Error,
                {
                    self.visit_string(value.to_string())
                }

                fn visit_u128<E>(self, value: u128) -> Result<Configuration, E>
                where
                    E: de::Error,
                {
                    self.visit_string(value.to_string())
                }


                fn visit_f32<E>(self, value: f32) -> Result<Configuration, E>
                where
                    E: de::Error,
                {
                    self.visit_string(value.to_string())
                }

                fn visit_f64<E>(self, value: f64) -> Result<Configuration, E>
                where
                    E: de::Error,
                {
                    self.visit_string(value.to_string())
                }
            }

            deserializer.deserialize_any(ConfigurationVisitor)
            
    }
}

fn get_configuration_location() -> Result<String, Box<dyn Error>> {
    let configuration_filename = env::var_os(OsStr::new("EDGEX_CONFIG_FILE"))
        .unwrap_or(OsString::from("configuration.yaml"))
        .into_string()
        .map_err(|_| Box::new(OsStringConversionError) as Box<dyn Error>)?;

    if let Ok(configuration_url) = Url::parse(&configuration_filename) {
        if configuration_url.scheme() == "http" ||  configuration_url.scheme() == "https" {
            return Ok(configuration_filename)
        }
    }
        

    let configuration_dir = env::var_os(OsStr::new("EDGEX_CONFIG_DIR"))
        .unwrap_or(OsString::from("."))
        .into_string()
        .map_err(|_| Box::new(OsStringConversionError) as Box<dyn Error>)?;

    let profile = env::var_os(OsStr::new("EDGEX_PROFILE"))
        .unwrap_or(OsString::from("."))
        .into_string()
        .map_err(|_| Box::new(OsStringConversionError) as Box<dyn Error>)?;

    let path = Path::new(&configuration_dir)
        .join(profile)
        .join(configuration_filename)
        .to_str()
        .unwrap_or("configuration.yaml")
        .to_owned();

    Ok(path)
}

fn format_key_to_env(key: &str) -> String {
    key.replace("-", "_").to_uppercase()
}

fn load_configuration_from_yaml(configuration_file: &str) -> Result<Configuration, Box<dyn Error>> {
    let configuration_reader = std::fs::File::open(configuration_file)?;
    serde_yaml::from_reader(configuration_reader)
        .map_err(|e| Box::new(e) as Box<dyn Error>)
}

impl Configuration {
    pub fn load() -> Result<Self, Box<dyn Error>> {
        let configuration_file = get_configuration_location()?;
        let mut config = load_configuration_from_yaml(&configuration_file)?;
        config.override_configuration_from_env()?;
        Ok(config)
    }

    fn generate_configuration_path<'a>(&'a mut self, current_path: &str, paths: &mut HashMap<String, &'a mut String>) {
        match self {
            Configuration::Leaf(value) => {
                paths.insert(current_path.to_string(), value);
            }
            Configuration::Node(node) => {
                for (key, value) in node {
                    let new_path = if current_path.is_empty() {
                        format_key_to_env(key)
                    } else {
                        let key = format_key_to_env(key);
                        format!("{}_{}", current_path, key)
                    };
                    value.generate_configuration_path(&new_path, paths);
                }
            }
        }
    }

    fn override_configuration_from_env(&mut self) -> Result<u64, Box<dyn Error>> {
        let mut override_configurations: HashMap<String, &mut String> = HashMap::new();

        self.generate_configuration_path("", &mut override_configurations);

        let mut count: u64 = 0;
        for (key, value) in env::vars() {
            if let Some(existing_configuration) = override_configurations.get_mut(&key) {
                **existing_configuration = value;
                count += 1;
            }
        }

        Ok(count)
    }

    pub fn get(&self, keys: &[&str]) -> Option<String> {
        let mut current_node = self;

        for key in keys {
            match current_node {
                Configuration::Node(node_map) => {
                    if let Some(next_node) = node_map.get(*key) {
                        current_node = next_node;
                    } else {
                        return None;
                    }
                }
                _ => return None,
            }
        }

        match current_node {
            Configuration::Leaf(value) => Some(value.clone()),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load() -> Result<(), Box::<dyn Error>> {
        let config_test = Path::new(&env::var("CARGO_MANIFEST_DIR")?).join("src").join("configuration");
        let _ = env::set_current_dir(config_test);

        env::set_var("CLIENTS_CORE_METADATA_PORT", "42");


        let config = Configuration::load()?;
        assert_eq!(config.get(&["Writable", "LogLevel"]).unwrap(), "INFO");
        assert_eq!(config.get(&["Clients", "core-metadata", "Port"]).unwrap(), "42");

        Ok(())
    }
}