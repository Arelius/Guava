mod platform {
  mod sdl;
}

fn main() {
  let platform = platform::sdl::Platform::new();
  platform::sdl::delay(2000);
}
