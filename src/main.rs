use gl::{types::*, COLOR_BUFFER_BIT, FRAGMENT_SHADER, VERTEX_SHADER};
use sdl2::video::GLProfile;
mod shaders;

fn main() {
    let shader_program;

    let sdl = sdl2::init().unwrap();
    let video_sys = sdl.video().unwrap();

    let gl_attr = video_sys.gl_attr();
    gl_attr.set_context_profile(GLProfile::Core);
    gl_attr.set_context_version(4, 1);

    let window = video_sys
        .window("SDL2 Proyect", 1280, 720)
        .opengl()
        .resizable()
        .build()
        .unwrap();

    let _gl_context = window.gl_create_context().unwrap();
    window.gl_make_current(&_gl_context).unwrap();

    gl::load_with(|s| video_sys.gl_get_proc_address(s) as *const _);

    let la_matriz: [f32; 9] = [
        -0.5, -0.5, 1.0, // Vértice inferior izquierdo
        0.5, -0.5, 0.0, // Vértice inferior derecho
        0.0, 0.5, 0.0, // Vértice superior
    ];
    let vertex_shader_src = r#"
        #version 410 core
        layout (location = 0) in vec3 aPos;
        void main() {
            gl_Position = vec4(aPos, 1.0);
        }
        "#;
    let fragment_shader_src = r#"
    #version 410 core
    out vec4 FragColor;
    void main() {
        FragColor = vec4(0.0, 0.0, 0.0, 1.0);
    }
    "#;

    let vertex_shader = shaders::compile_shader(vertex_shader_src, VERTEX_SHADER);
    let fragment_shader = shaders::compile_shader(fragment_shader_src, FRAGMENT_SHADER);

    shader_program = shaders::link_program(vertex_shader, fragment_shader);

    unsafe {
        gl::UseProgram(shader_program);
    }
    let mut vao: GLuint = 0;
    let mut vbo: GLuint = 0;
    unsafe {
        // Crear VAO y VBO
        gl::GenVertexArrays(1, &mut vao);
        gl::GenBuffers(1, &mut vbo);

        // Configurar VAO
        gl::BindVertexArray(vao);

        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        gl::BufferData(
            gl::ARRAY_BUFFER,
            (la_matriz.len() * std::mem::size_of::<GLfloat>()) as GLsizeiptr,
            la_matriz.as_ptr() as *const _,
            gl::STATIC_DRAW,
        );

        // Configurar atributos de vértices
        gl::VertexAttribPointer(
            0,
            3,
            gl::FLOAT,
            gl::FALSE,
            (3 * std::mem::size_of::<GLfloat>()) as GLsizei,
            std::ptr::null(),
        );
        gl::EnableVertexAttribArray(0);

        gl::BindBuffer(gl::ARRAY_BUFFER, 0);
        gl::BindVertexArray(0);
    }
    let mut event_pump = sdl.event_pump().unwrap();

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                sdl2::event::Event::Quit { .. } => break 'running,
                _ => {}
            }
        }

        unsafe {
            gl::ClearColor(1.0, 1.0, 1.0, 1.0);
            gl::Clear(COLOR_BUFFER_BIT);

            gl::UseProgram(shader_program);
            gl::BindVertexArray(vao);
            gl::DrawArrays(gl::TRIANGLES, 0, 3)
        }

        window.gl_swap_window();
    }
}
