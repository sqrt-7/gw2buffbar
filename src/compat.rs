const MAX_BUFFS: usize = 50;

pub static DLL_GET_BUFFS: &str = "GetCurrentPlayerStackedBuffs";

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
