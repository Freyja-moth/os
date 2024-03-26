#[derive(PartialEq, Eq, Clone, Debug)]
pub enum QemuExitCode {
    Success = 0x10,
    Failed = 0x11,
}
impl QemuExitCode {
    pub fn send(self) {
        use x86_64::instructions::port::Port;

        let mut port = Port::new(0xf4);

        unsafe {
            port.write(self as u32);
        }
    }
}
