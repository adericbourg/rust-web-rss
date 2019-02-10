use std::fs;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Configuration {
    pub subscriptions: Vec<Subscription>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Subscription {
    pub name: String,
    pub url: String,
}

pub fn load(configuration_file: &str) -> Configuration {
    println!("Loading configuration file: '{}'", configuration_file);
    let file_content = read_file(configuration_file);
    let configuration: Configuration = serde_yaml::from_str(&file_content).unwrap();

    configuration
}

fn read_file(file_name: &str) -> String {
    let contents = fs::read_to_string(file_name).expect("Something went wrong reading the file");
    contents
}
