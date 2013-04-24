extern mod opengles;
//use gl = opengles::gl2;

pub trait Device {
  pub fn init(&self);
}

pub struct GLDevice;

impl GLDevice {
  pub fn new() -> ~GLDevice {
    ~GLDevice
  }
}

impl Device for GLDevice {
  pub fn init(&self) {
    1;
  }
}

pub fn test() {
  let device = ~GLDevice::new();
  device.init();
}