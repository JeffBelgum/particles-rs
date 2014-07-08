extern crate particles;
extern crate glfw;
extern crate native;
extern crate hgl;
extern crate gl;
extern crate libc;

use std::mem::size_of;

use glfw::Context;
use hgl::{Shader, Program, Points, Vbo, Vao};

static VERTEX_SHADER: &'static str = r"
#version 150

in vec3 position;
in vec3 color;
out vec3 Color;

void main() {
  gl_Position = vec4(position, 1.0);
  Color = color;
}";


static FRAGMENT_SHADER: &'static str = r"
#version 150

out vec4 out_color;
in vec3 Color;

void main() {
  out_color = vec4(Color, 1.0);
}";

#[start]
fn start(argc: int, argv: *const *const u8) -> int {
  native::start(argc, argv, main)
}

fn main() {
  let glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();

  glfw.window_hint(glfw::ContextVersion(4, 1));
  glfw.window_hint(glfw::OpenglProfile(glfw::OpenGlCoreProfile));
  glfw.window_hint(glfw::OpenglForwardCompat(true));

  let (window, events) = glfw.create_window(800, 600, "HGL", glfw::Windowed)
      .expect("Failed to create GLFW window.");

  window.set_key_polling(true);
  window.make_current();
  gl::load_with(|p| glfw.get_proc_address(p));

  let (w, h) = window.get_framebuffer_size();
  gl::Viewport(0, 0, w, h);

  let vao = Vao::new();
  vao.bind();
  let program = Program::link([Shader::compile(VERTEX_SHADER, hgl::VertexShader),
                               Shader::compile(FRAGMENT_SHADER, hgl::FragmentShader)]).unwrap();
  program.bind_frag(0, "out_color");
  program.bind();

  let vbo = Vbo::from_data(
    [ 0.0f32,  0.0, 0.0, 0.0, 1.0, 0.0],
  hgl::StaticDraw);

  vao.enable_attrib(&program, "position", gl::FLOAT, 3, 6*size_of::<f32>() as i32, 0);
  vao.enable_attrib(&program, "color", gl::FLOAT, 3, 6*size_of::<f32>() as i32, 3*size_of::<f32>());
  vbo.bind();

  gl::ClearColor(0.0, 0.0, 0.0, 1.0);
  gl::PointSize(20.0);

  let mut frame = 0u;

  while !window.should_close() {
    frame += 1;
    glfw.poll_events();
    for (_, event) in glfw::flush_messages(&events) {
      handle_window_event(&window, event);
    }
    gl::Clear(gl::COLOR_BUFFER_BIT);

    vbo.load_data(
      [ 0.0f32, (frame as f32)/200.0-1.0, 0.0, 1.0, 0.0, 0.0],
    hgl::StaticDraw);

    vao.draw_array(Points, 0, 1);
    window.swap_buffers();
  }
}

fn handle_window_event(window: &glfw::Window, event: glfw::WindowEvent) {
  match event {
    glfw::KeyEvent(glfw::KeyEscape, _, glfw::Press, _) => {
      window.set_should_close(true)
    }
    _ => {}
  }
}
