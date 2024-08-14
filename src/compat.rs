use libloading::{Library, Symbol};
use once_cell::sync::Lazy;

const MAX_BUFFS: usize = 50;

pub static DLL_GET_BUFFS: &str = "GetCurrentPlayerStackedBuffs";

pub static DLL_LIB: Lazy<Option<Library>> = Lazy::new(|| {
    let dll_filename = libloading::library_filename("getbuffs");
    log::info!(target: "file", "dll file: {:?}", dll_filename);

    let lib = unsafe { Library::new(dll_filename) };
    if let Err(e) = lib {
        log::info!(target: "file", "init failed [1]: {}", e);
        return None;
    }

    Some(lib.unwrap())
});

pub static DLL_FUNC: Lazy<Option<Symbol<GetBuffsFnSig>>> = Lazy::new(|| {
    if DLL_LIB.is_some() {
        let f = unsafe {
            DLL_LIB
                .as_ref()
                .unwrap()
                .get::<GetBuffsFnSig>(DLL_GET_BUFFS.as_bytes())
        };

        if let Err(e) = f {
            log::info!(target: "file", "init failed [2]: {}", e);
            return None;
        }

        return Some(f.unwrap());
    }

    None
});

pub type GetBuffsFnSig = fn() -> *const MyBoons;

#[derive(Copy, Clone)]
pub struct MyBoons {
    pub id: u32,
    pub count: i32,
}

pub unsafe fn get_len(items: *const MyBoons) -> usize {
    for len in 0..=MAX_BUFFS {
        let item = *items.add(len);
        if item.id == 0 {
            return len;
        }
    }
    unreachable!();
}
