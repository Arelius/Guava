mod platform {
  mod sdl;
}

fn main() {
    platform::sdl::sdlMain();
}