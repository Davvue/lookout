use crate::config::{Config, OutputList, Service};
use crate::store::{Store, StoreError};
use serde::Serialize;
use serde::de::DeserializeOwned;
use std::io::Write;
use std::path::PathBuf;
use std::{fs, io};

const CONFIG_FILE: &str = "config.json";
const SERVICE_FILE: &str = "services.json";
const OUTPUT_FILE: &str = "output.json";

pub struct FileStore {
    dir: PathBuf,
}

impl FileStore {
    pub fn open(dir: &str) -> Result<Self, StoreError> {
        let store = Self {
            dir: expanduser::expanduser(dir)?,
        };
        store.ensure_structure()?;
        Ok(store)
    }

    fn ensure_structure(&self) -> io::Result<()> {
        fs::create_dir_all(&self.dir)?;

        let config_json = serde_json::to_string(&Config::default())?;
        let services_json = serde_json::to_string::<[Service]>(&[])?;
        let output_json = serde_json::to_string(&OutputList::default())?;

        self.write_if_missing(self.dir.join(CONFIG_FILE), &config_json)?;
        self.write_if_missing(self.dir.join(SERVICE_FILE), &services_json)?;
        self.write_if_missing(self.dir.join(OUTPUT_FILE), &output_json)?;

        Ok(())
    }

    fn write_if_missing(&self, path: PathBuf, contents: &str) -> io::Result<()> {
        if !fs::exists(&path)? {
            fs::write(path, contents)?;
        }
        Ok(())
    }

    fn read_json<T: DeserializeOwned>(&self, name: &str) -> Result<T, StoreError> {
        let file = fs::File::open(self.dir.join(name))?;
        Ok(serde_json::from_reader(io::BufReader::new(file))?)
    }

    fn write_json<T: Serialize + ?Sized>(&self, name: &str, data: &T) -> Result<(), StoreError> {
        let tmp = self.dir.join(format!("{name}.tmp"));

        let mut writer = io::BufWriter::new(fs::File::create(&tmp)?);
        serde_json::to_writer(&mut writer, data)?;
        writer.flush()?;

        fs::rename(tmp, self.dir.join(name))?;
        Ok(())
    }
}

impl Store for FileStore {
    fn config(&self) -> Result<Config, StoreError> {
        self.read_json(CONFIG_FILE)
    }
    fn services(&self) -> Result<Vec<Service>, StoreError> {
        self.read_json(SERVICE_FILE)
    }
    fn output(&self) -> Result<OutputList, StoreError> {
        self.read_json(OUTPUT_FILE)
    }

    fn save_output(&self, output: &OutputList) -> Result<(), StoreError> {
        self.write_json(OUTPUT_FILE, output)
    }

    fn save_services(&self, services: &[Service]) -> Result<(), StoreError> {
        self.write_json(SERVICE_FILE, services)
    }
}
