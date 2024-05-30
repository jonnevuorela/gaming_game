#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#![allow(unused_imports)]
#![allow(clippy::single_match)]
#![allow(clippy::zero_ptr)]

const WINDOW_TITLE: &str = "gaming_game";
use beryllium::{
    events::Event,
    init::InitFlags,
    video::{CreateWinArgs, GlContextFlags, GlProfile, GlSwapInterval},
    *,
};
use core::{
    convert::{TryFrom, TryInto},
    mem::{size_of, size_of_val},
};
use gaming_game as lib;
use lib::{Buffer, BufferType, PolygonMode, Shader, ShaderProgram, ShaderType, VertexArray};
use ogl33::*;

type Vertex = [f32; 3];
type TriIndexes = [u32; 3];

const VERTICES: [Vertex; 4] = [
    [0.5, 0.5, 0.0],
    [0.5, -0.5, 0.0],
    [-0.5, -0.5, 0.0],
    [-0.5, 0.5, 0.0],
];
const INDICES: [TriIndexes; 2] = [[0, 1, 3], [1, 2, 3]];

const VERT_SHADER: &str = r#"#version 330 core
    layout (location = 0) in vec3 pos;
    
    void main() {
        gl_Position = vec4(pos.x, pos.y, pos.z, 1.0);
    }
"#;
const FRAG_SHADER: &str = r#"#version 330 core
    out vec4 final_color;

    void main(){
        final_color = vec4(0.2,0.5,0.5,1.0);
    }
"#;
fn main() {
    let sdl = Sdl::init(init::InitFlags::EVERYTHING);

    sdl.set_gl_context_major_version(3).unwrap();
    sdl.set_gl_context_major_version(3).unwrap();
    sdl.set_gl_profile(video::GlProfile::Core).unwrap();
    let mut flags = video::GlContextFlags::default();
    if cfg!(target_os = "macos") {
        flags |= video::GlContextFlags::FORWARD_COMPATIBLE;
    }
    if cfg!(debug_asserts) {
        flags |= video::GlContextFlags::DEBUG;
    }
    sdl.set_gl_context_flags(flags).unwrap();

    let win = sdl
        .create_gl_window(video::CreateWinArgs {
            title: WINDOW_TITLE,
            width: 800,
            height: 600,
            ..Default::default()
        })
        .expect("couldn't make a window and context");
    //win.set_swap_interval(video::GlSwapInterval::Vsync).unwrap();

    unsafe {
        load_gl_with(|f_name| win.get_proc_address(f_name.cast()));
    }

    let vao = VertexArray::new().expect("Couldn't make a VAO");
    vao.bind();

    let vbo = Buffer::new().expect("Couldn't make a VBO");
    vbo.bind(BufferType::Array);
    lib::buffer_data(
        BufferType::Array,
        bytemuck::cast_slice(&VERTICES),
        GL_STATIC_DRAW,
    );

    unsafe {
        glVertexAttribPointer(
            0,
            3,
            GL_FLOAT,
            GL_FALSE,
            size_of::<Vertex>().try_into().unwrap(),
            0 as *const _,
        );
        glEnableVertexAttribArray(0);
    }

    let shader_program = ShaderProgram::from_vert_frag(VERT_SHADER, FRAG_SHADER).unwrap();
    shader_program.use_program();

    let ebo = Buffer::new().expect("Couldn't make the element buffer.");
    ebo.bind(BufferType::ElementArray);
    lib::buffer_data(
        BufferType::ElementArray,
        bytemuck::cast_slice(&INDICES),
        GL_STATIC_DRAW,
    );

    lib::clear_color(0.1, 0.1, 0.2, 1.0);

    'main_loop: loop {
        while let Some(event) = sdl.poll_events() {
            match event {
                (events::Event::Quit, _) => break 'main_loop,
                _ => (),
            }
        }
        unsafe {
            glClear(GL_COLOR_BUFFER_BIT);
            glDrawElements(GL_TRIANGLES, 6, GL_UNSIGNED_INT, 0 as *const _);
        }
        win.swap_window();
    } //'main_loop
} //main
