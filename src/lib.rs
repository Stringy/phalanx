extern crate aya;
extern crate aya_obj;

use std::path::PathBuf;

use aya::{BpfError, BpfLoader, Btf};
use aya_obj::Object;

pub struct Phalanx {
    bpf: aya::Bpf,
}

pub fn load_ext(filename: PathBuf, extensions: Option<&[&str]>) -> Result<Phalanx, BpfError> {
    Phalanx::with_extensions(filename, extensions)
}

pub fn load(filename: PathBuf) -> Result<Phalanx, BpfError> {
    Phalanx::new(filename)
}

impl Phalanx {
    pub fn new(filename: PathBuf) -> Result<Self, BpfError> {
        let bytes = std::fs::read(filename).unwrap();
        let mut object = Object::parse(&bytes).unwrap();

        for function in object.programs.keys() {
            println!("Function: {}", function);
        }

        Ok(Phalanx {
            bpf: aya::Bpf::load(&bytes)?,
        })
    }

    pub fn with_extensions(
        filename: PathBuf,
        extensions: Option<&[&str]>,
    ) -> Result<Self, BpfError> {
        let mut loader = BpfLoader::new();

        let btf = Btf::from_sys_fs().unwrap();
        loader.btf(Some(&btf));

        if let Some(extensions) = extensions {
            for ext in extensions {
                loader.extension(*ext);
            }
        }

        Ok(Phalanx {
            bpf: loader.load_file(filename).unwrap(),
        })
    }

    pub fn map(&self, name: &str) -> Option<aya::maps::MapRef> {
        let map = self.bpf.maps().find(|m| name == m.0);

        match map {
            Some((_name, map_ref)) => match map_ref {
                Ok(m) => Some(m),
                _ => None,
            },
            None => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
