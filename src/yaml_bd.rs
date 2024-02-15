use crate::bd_manager::{BDOperation, SerdePersons};;
use std::path::PathBuf;
use clap::Parser;

pub struct YamlBD {
    pub file_path: PathBuf,
}

impl BDOperation for YamlBD {
    fn load(&self) -> Result<crate::bd_manager::SerdePersons, Box<dyn std::error::Error>> {
        let handler = std::fs::File::open(&self.file_path)?;
        let data: SerdePersons = serde_yaml::from_reader(&handler)?;
        Ok(data)
    }
    fn save(
        &self,
        persons: &crate::bd_manager::SerdePersons,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let handler = std::fs::File::open(&self.file_path)?;
        serde_yaml::to_writer(&handler, &persons)?;
        Ok(())
    }
}

//
pub fn get_path()->PathBuf {
    let args = Args::parse();
    let path: String = std::env::var("config.yaml").unwrap();
    match args.file_path {
        Some(path) => path,
        None => PathBuf::from(path)
    }
}


#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    #[arg(short, long)]
    file_path: Option<PathBuf>, 
}
