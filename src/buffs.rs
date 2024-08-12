use crate::compat::{get_len, GetBuffsFnSig, DLL_GET_BUFFS};
use libloading::Library;
use once_cell::sync::Lazy;
use std::{collections::HashMap, time::Instant};

const TICK: u128 = 100;

static BUFF_MAP: Lazy<HashMap<u32, String>> = Lazy::new(|| {
    HashMap::from([
        (717, "Protection".to_owned()),
        (718, "Regeneration".to_owned()),
        (719, "Swiftness".to_owned()),
        (725, "Fury".to_owned()),
        (726, "Vigor".to_owned()),
        (740, "Might".to_owned()),
        (743, "Aegis".to_owned()),
        (873, "Retaliation".to_owned()),
        (1122, "Stability".to_owned()),
        (1187, "Quickness".to_owned()),
        (26980, "Resistance".to_owned()),
    ])
});

pub struct BuffHandler {
    last_process: Instant,
    last_output: Vec<String>,
    dll_library: Library,
}

impl BuffHandler {
    pub fn new() -> Option<Self> {
        let dll_filename = libloading::library_filename("getbuffs");
        log::info!(target: "file", "dll file: {:?}", dll_filename);

        let lib = unsafe { Library::new(dll_filename) };
        if let Err(e) = lib {
            log::info!(target: "file", "gw2buffbar init failed: {}", e);
            return None;
        }

        let lib = lib.unwrap();

        // Try DLL function
        let getbuffs_fn = unsafe { lib.get::<GetBuffsFnSig>(DLL_GET_BUFFS.as_bytes()) };
        if let Err(e) = getbuffs_fn {
            log::info!(target: "file", "gw2buffbar init failed: {}", e);
            return None;
        }

        Some(BuffHandler {
            last_process: Instant::now(),
            last_output: Vec::new(),
            dll_library: lib,
        })
    }

    pub fn getbuffs(&mut self) -> Result<Vec<String>, String> {
        let elapsed = self.last_process.elapsed().as_millis();
        if elapsed < TICK {
            return Ok(self.last_output.clone());
        }

        self.last_process = Instant::now();

        match unsafe {
            self.dll_library
                .get::<GetBuffsFnSig>(DLL_GET_BUFFS.as_bytes())
        } {
            Err(e) => {
                return Err(e.to_string());
            }
            Ok(func) => {
                let buffs = func();
                let len = unsafe { get_len(buffs) };
                let mut new_vec = Vec::new();
                unsafe {
                    let slice = std::slice::from_raw_parts(buffs, len);
                    for item in slice.iter() {
                        if let Some(name) = BUFF_MAP.get(&(item.id)) {
                            new_vec.push(format!("{}({})", *name, item.count));
                        }
                    }
                }
                self.last_output = new_vec;
                Ok(self.last_output.clone())
            }
        }
    }
}
