// SDL.h
static SDL_INIT_VIDEO:u32 = 0x00000020;

// SDL_video.h
static SDL_WINDOWPOS_CENTERED:i32 = 0x2FFF0000;

static SDL_WINDOW_FULLSCREEN:u32 = 0x00000001;
static SDL_WINDOW_OPENGL:u32 = 0x00000002;
static SDL_WINDOW_SHOWN:u32 = 0x00000004;
static SDL_WINDOW_HIDDEN:u32 = 0x00000008;
static SDL_WINDOW_BORDERLESS:u32 = 0x00000010;
static SDL_WINDOW_RESIZABLE:u32 = 0x00000020;
static SDL_WINDOW_MINIMIZED:u32 = 0x00000040;
static SDL_WINDOW_MAXIMIZED:u32 = 0x00000080;
static SDL_WINDOW_INPUT_GRABBED:u32 = 0x00000100;
static SDL_WINDOW_INPUT_FOCUS:u32 = 0x00000200;
static SDL_WINDOW_MOUSE_FOCUS:u32 = 0x00000400;
static SDL_WINDOW_FULLSCREEN_DESKTOP:u32 = 0x00001001;
static SDL_WINDOW_FOREIGN:u32 = 0x0000080;

type SDL_Window = libc::c_void;
type SDL_GLContext = libc::c_void;

// SDL_GLattr
static SDL_GL_RED_SIZE:u32 = 0;
static SDL_GL_GREEN_SIZE:u32 = 1;
static SDL_GL_BLUE_SIZE:u32 = 2;
static SDL_GL_ALPHA_SIZE:u32 = 3;
static SDL_GL_BUFFER_SIZE:u32 = 4;
static SDL_GL_DOUBLEBUFFER:u32 = 5;
static SDL_GL_DEPTH_SIZE:u32 = 6;
static SDL_GL_STENCIL_SIZE:u32 = 7;
static SDL_GL_ACCUM_RED_SIZE:u32 = 8;
static SDL_GL_ACCUM_GREEN_SIZE:u32 = 9;
static SDL_GL_ACCUM_BLUE_SIZE:u32 = 10;
static SDL_GL_ACCUM_ALPHA_SIZE:u32 = 11;
static SDL_GL_STEREO:u32 = 12;
static SDL_GL_MULTISAMPLEBUFFERS:u32 = 13;
static SDL_GL_MULTISAMPLESAMPLES:u32 = 14;
static SDL_GL_ACCELERATED_VISUAL:u32 = 15;
static SDL_GL_RETAINED_BACKING:u32 = 16;
static SDL_GL_CONTEXT_MAJOR_VERSION:u32 = 17;
static SDL_GL_CONTEXT_MINOR_VERSION:u32 = 18;
static SDL_GL_CONTEXT_EGL:u32 = 19;
static SDL_GL_CONTEXT_FLAGS:u32 = 20;
static SDL_GL_CONTEXT_PROFILE_MASK:u32 = 21;
static SDL_GL_SHARE_WITH_CURRENT_CONTEX:u32 = 22;

#[link_name = "SDL2"]
extern mod SDL {
  // SDL.h
  fn SDL_Init(flags: libc::c_uint) -> libc::c_int;
  fn SDL_Quit();

  // SDL_video.h
  fn SDL_CreateWindow(title: *libc::c_char,
                      x: i32, y: i32,
                      w: i32, h: i32,
                      flags: u32)
    -> *SDL_Window;
  fn SDL_DestroyWindow(window: *SDL_Window);

  fn SDL_GL_CreateContext(window: *SDL_Window) -> *SDL_GLContext;
  fn SDL_GL_DeleteContext(context: *SDL_GLContext);

  fn SDL_GL_SetAttribute(attr: libc::c_uint, value: libc::c_int) -> libc::c_int;

  // SDL_timer.h

  fn SDL_Delay(ms: u32);
}

pub struct Platform {
  window: *SDL_Window,
  context: *SDL_GLContext
}

impl Platform {
  pub fn new() -> Platform {
    unsafe {
      if(SDL::SDL_Init(SDL_INIT_VIDEO) != 0) {
        io::println("Error Initializing SDL.");
      }

      SDL::SDL_GL_SetAttribute(SDL_GL_CONTEXT_MAJOR_VERSION, 3);
      SDL::SDL_GL_SetAttribute(SDL_GL_CONTEXT_MINOR_VERSION, 2);

      SDL::SDL_GL_SetAttribute(SDL_GL_DOUBLEBUFFER, 1);
      SDL::SDL_GL_SetAttribute(SDL_GL_DEPTH_SIZE, 32);


      let window = do str::as_c_str("Project Guava") |cstr| {
        SDL::SDL_CreateWindow(cstr,
                              SDL_WINDOWPOS_CENTERED,
                              SDL_WINDOWPOS_CENTERED,
                              1024, 768,
                              (SDL_WINDOW_OPENGL | SDL_WINDOW_SHOWN))
      };

      let context = SDL::SDL_GL_CreateContext(window);

      Platform{window: window, context: context}
    }
  }
}

impl Drop for Platform {
  fn finalize(&self) {
    unsafe {
      SDL::SDL_GL_DeleteContext(self.context);
      SDL::SDL_DestroyWindow(self.window);
      SDL::SDL_Quit();
    }
  }
}

pub fn delay(ms: u32) {
  unsafe {
    SDL::SDL_Delay(ms);
  }
}
