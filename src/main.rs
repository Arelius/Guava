mod platform {
  mod sdl;
}

mod render {
  mod render;
}

fn main() {
  let platform = platform::sdl::Platform::new();
  let ~render = render::render::Render::new();

  while(platform.continue()) {
    render.render();
    platform.finishFrame();
  }
}
