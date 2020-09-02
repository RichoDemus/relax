#[cfg(feature = "gpio")]
use std::fs::File;
#[cfg(feature = "gpio")]
use std::fs::OpenOptions;
#[cfg(feature = "gpio")]
use std::io::Write;

pub(crate) struct ServoController {
    enabled: bool,
    #[cfg(feature = "gpio")]
    file: File,
}

impl Default for ServoController {
    fn default() -> Self {
        ServoController {
            enabled: false,
            #[cfg(feature = "gpio")]
            file: OpenOptions::new()
                .write(true)
                .open("/dev/servoblaster")
                .expect("couldn't open servoblaster file"),
        }
    }
}

impl ServoController {
    pub(crate) fn set_enabled(&mut self) {
        if self.enabled {
            return;
        }
        self.enabled = true;
        #[cfg(feature = "gpio")]
        set_enabled_gpio(&mut self.file);
        #[cfg(not(feature = "gpio"))]
        set_enabled_print();
    }

    pub(crate) fn set_disabled(&mut self) {
        if !self.enabled {
            return;
        }
        self.enabled = false;
        #[cfg(feature = "gpio")]
        set_disabled_gpio(&mut self.file);
        #[cfg(not(feature = "gpio"))]
        set_disabled_print();
    }
}

#[cfg(not(feature = "gpio"))]
fn set_enabled_print() {
    println!("Enabled!");
}

#[cfg(not(feature = "gpio"))]
fn set_disabled_print() {
    println!("Disabled!");
}

#[cfg(feature = "gpio")]
fn set_enabled_gpio(file: &mut File) {
    println!("Raising");
    file.write_all("0=100%\n".as_bytes())
        .expect("failed to set enabled");
    file.flush().expect("couldn't flush");
}

#[cfg(feature = "gpio")]
fn set_disabled_gpio(file: &mut File) {
    println!("Lowering");
    file.write_all("0=0%\n".as_bytes())
        .expect("failed to set enabled");
    file.flush().expect("couldn't flush");
}
