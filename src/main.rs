mod platform {
  mod sdl;
}

fn main() {
  let platform = platform::sdl::Platform::new();

  while(platform.continue()) {

    platform.finishFrame();
  }
}
