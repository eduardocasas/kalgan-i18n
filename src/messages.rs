use log::{debug, error, info, trace, warn};
use serde::Deserialize;
use serde_yaml::Value;
use std::{collections::HashMap, fs, path::Path};

pub(crate) fn generate(source: &str) -> crate::Messages {
    info!("");
    info!("***********************************************************************************");
    info!("Crate: i18n");
    info!("Start Translation Messages Generation");
    let mut messages = crate::Messages {
        collection: HashMap::new(),
    };
    let source_folder = Path::new(source);
    if source_folder.exists() {
        if source_folder.is_dir() {
            match fs::read_dir(source_folder) {
                Ok(read_dir) => {
                    for entry in read_dir {
                        let path = entry.unwrap().path();
                        if path.is_dir() {
                            let folder_name =
                                path.file_name().unwrap().to_str().unwrap().to_string();
                            messages
                                .collection
                                .insert(folder_name.clone(), HashMap::new());
                            info!("Parsing messages for language \"{}\"...", &folder_name);
                            walk_folder_files(
                                &path,
                                &mut messages.collection.get_mut(&folder_name).unwrap(),
                            );
                        }
                    }
                }
                Err(e) => error!("{}", e),
            }
        } else {
            error!("Source path {} is not a folder.", source);
        }
    } else {
        error!("Source path {} not found.", source);
    }
    let mut number_of_messages = 0;
    for (_language, collection) in &messages.collection {
        number_of_messages += collection.len();
    }
    match number_of_messages {
        0 => info!("Result: No messages have been parsed"),
        1 => info!("Result: 1 message has been parsed"),
        _ => info!("Result: {} messages have been parsed", number_of_messages),
    }
    info!("End Translation Messages Generation");
    info!("***********************************************************************************");
    messages
}
fn walk_folder_files(dir: &Path, parameters: &mut HashMap<String, String>) {
    debug!("Reading folder {}...", dir.display());
    if dir.is_dir() {
        match fs::read_dir(dir) {
            Ok(read_dir) => {
                for entry in read_dir {
                    let path = entry.unwrap().path();
                    if path.is_dir() {
                        debug!("{} is dir", path.display());
                        walk_folder_files(&path, parameters);
                    } else {
                        debug!("{} is file", path.display());
                        get_file_content(&path, parameters);
                    }
                }
            }
            Err(e) => warn!("{}", e),
        }
    }
}
fn get_file_content(path: &Path, parameters: &mut HashMap<String, String>) {
    debug!("Reading file {}...", path.display());
    match fs::read_to_string(path) {
        Ok(config_file) => {
            let document = serde_yaml::Deserializer::from_str(&config_file);
            match Value::deserialize(document) {
                Ok(value) => iterate(value, parameters, &mut "".to_string(), ""),
                Err(e) => warn!("{}", e),
            };
        }
        Err(e) => warn!("{}", e),
    };
}
fn iterate(map: Value, parameters: &mut HashMap<String, String>, subpath: &mut String, prev: &str) {
    for element in map.as_mapping() {
        for (index, value) in element.iter() {
            trace!("Parsing: {:?}", index);
            let key = match index.as_str() {
                Some(p) => p.to_string(),
                None => index.as_i64().unwrap().to_string(),
            };
            if key.contains(".") {
                warn!("Dots are not allowed in key names.");
                warn!("Message \"{}\" is skipped.", key);
                break;
            }
            let mut pos = match subpath.rfind(".") {
                Some(num) => num + 1,
                None => 0,
            };
            while !subpath.is_empty() && subpath[pos..].to_string() != prev {
                *subpath = if pos > 0 {
                    subpath[..pos - 1].to_string()
                } else {
                    "".to_string()
                };
                pos = match subpath.rfind(".") {
                    Some(num) => num + 1,
                    None => 0,
                };
            }
            if value.is_mapping() {
                if subpath.is_empty() {
                    subpath.push_str(format!("{}", key.clone()).as_str());
                } else {
                    subpath.push_str(format!(".{}", key.clone()).as_str());
                }
                iterate(value.clone(), parameters, subpath, &key);
            } else {
                let message_name = if subpath.is_empty() {
                    key.to_string()
                } else {
                    format!("{}.{}", subpath.clone(), key)
                };
                let mut val = "".to_string();
                if value.is_string() {
                    val = match value.as_str() {
                        Some(x) => x.to_string(),
                        None => "".to_string(),
                    };
                } else if value.is_bool() {
                    val = match value.as_bool() {
                        Some(x) => x.to_string(),
                        None => "".to_string(),
                    };
                } else if value.is_number() {
                    val = match value.as_i64() {
                        Some(x) => x.to_string(),
                        None => "".to_string(),
                    };
                };
                parameters.insert(message_name, val);
            }
        }
    }
}
