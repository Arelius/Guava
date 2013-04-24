mod gldevice;

pub struct Render {
  device: ~gldevice::GLDevice
}

impl Render {
  pub fn new() -> ~Render {
    let render = ~Render { device: gldevice::GLDevice::new() };
    render.init();
    render
  }

  fn init(&self) {
    //self.device.init()
    gldevice::GLDevice::test();
  }

  pub fn render(&self) {
    
  }
}