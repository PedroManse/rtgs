use std::ffi::{CStr, CString};
use std::ptr::NonNull;

use libloading::{Library, Symbol};

#[repr(C)]
pub struct GameId(i32);

pub struct GameLibSymb<'l> {
    pub init: Symbol<'l, unsafe extern "C" fn()>,
    pub work: Symbol<'l, unsafe extern "C" fn()>,
    pub new_game: Symbol<'l, unsafe extern "C" fn() -> GameId>,
    pub inspect: Symbol<'l, unsafe extern "C" fn(GameId)>,
    pub resource: Symbol<'l, unsafe extern "C" fn(*const i8) -> *const std::ffi::c_void>,
}

impl<'l> GameLibSymb<'l> {
    pub unsafe fn new(lib: &'l Library) -> Result<Self, libloading::Error> {
        unsafe {
            Ok(Self {
                init: lib.get("init")?,
                work: lib.get("work")?,
                new_game: lib.get("new_game")?,
                inspect: lib.get("inspect")?,
                resource: lib.get("resource")?,
            })
        }
    }

    pub fn resource<T>(&self, resource_id: &str) -> Option<NonNull<T>> {
        let c_res_id = CString::new(resource_id).unwrap();
        let res_ptr = unsafe { (self.resource)(c_res_id.as_ptr()) };
        NonNull::new(res_ptr as *mut T)
    }

    pub fn cstr_resource(&self, resource_id: &str) -> Option<&'static CStr> {
        Some(unsafe { std::ffi::CStr::from_ptr(self.resource(resource_id)?.as_ptr()) })
    }
}

pub struct GameLib<'l> {
    pub id: &'static std::ffi::CStr,
    pub version: &'static std::ffi::CStr,
    pub symbs: GameLibSymb<'l>,
}

impl<'l> GameLib<'l> {
    pub unsafe fn new(lib: &'l Library) -> Result<Self, libloading::Error> {
        let symbs = unsafe { GameLibSymb::new(lib)? };
        unsafe { (symbs.init)() };
        let id = symbs.cstr_resource("id").unwrap();
        let version = symbs.cstr_resource("version").unwrap();
        Ok(Self { id, version, symbs })
    }

    pub fn work(&self) {
        unsafe { (self.symbs.work)() }
    }
    pub fn init(&self) {
        unsafe { (self.symbs.init)() }
    }
    pub fn new_game(&self) -> GameId {
        unsafe { (self.symbs.new_game)() }
    }
    pub fn inspect(&self, gid: GameId) {
        unsafe { (self.symbs.inspect)(gid) }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let lib = unsafe { Library::new("./test.so")? };
    let llib = unsafe { GameLib::new(&lib)? };
    println!("[Loaded {:?} @ {:?}]", llib.id, llib.version);
    println!("[{:?}]: executing \"init\"", llib.id);
    llib.init();
    let gid = llib.new_game();
    println!("[{:?}]: New Game(0x{:X})", llib.id, gid.0);
    llib.inspect(gid);
    Ok(())
}
