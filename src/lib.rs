use std::error::Error;
use std::path::{Path};
use std::process::{Command, Output};

pub fn set_dpi(scale: u64, monitor_id: u64) -> Result<bool, Box<dyn Error>>{
    unsafe{
        let lib = libloading::Library::new("ext/HA-C-Lib.dll")?;
        let func: libloading::Symbol<unsafe extern fn(u64, u64) -> bool> = lib.get(b"SetDPI")?;
        Ok(func(scale, monitor_id))
    }
}

pub fn set_display_mode(config_path: &Path) -> std::io::Result<Output> {
    Command::new("ext/MultiMonitorTool.exe")
        .args(&["/LoadConfig", config_path.to_str().unwrap()])
        .output()
}


#[cfg(test)]
mod tests {
    use crate::{set_dpi, set_display_mode};
    use std::thread;
    use std::time::Duration;
    use std::path::Path;

    #[test]
    fn it_works() {
        let res = [100,125,150,175];
        for re in &res {
            assert!(set_dpi(*re, 1).unwrap());
            thread::sleep(Duration::from_millis(1000));
        }
        assert!(set_dpi(100, 1).unwrap());
    }

    #[test]
    fn set_monitor_config(){
        set_display_mode(Path::new("ext/disable_right.cfg")).unwrap();
        set_display_mode(Path::new("ext/activate_all.cfg")).unwrap();
    }
}
