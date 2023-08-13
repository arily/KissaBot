use std::{
    collections::HashSet,
    process::Command,
};

fn main() {
    let output = Command::new("cargo")
        .arg("metadata")
        .arg("--format-version=1")
        .output()
        .unwrap();
    let metadata = serde_json::from_slice::<serde_json::Value>(&output.stdout).unwrap();
    let packages = metadata["packages"].as_array().unwrap();
    let mut dependencies: HashSet<&str> = HashSet::new();
    for package in packages {
        let deps = package["dependencies"].as_array().unwrap();
        for dep in deps {
            dependencies.insert(dep["name"].as_str().unwrap());
        }
    }
    println!("{:#?}", dependencies)
}
