use glium::glutin::event::{Event, StartCause, WindowEvent};
use glium::glutin::event_loop::{ControlFlow, EventLoop};
use glium::glutin::window::WindowBuilder;
use glium::glutin::ContextBuilder;

use glium::{implement_vertex, index, Display, Program, Surface, VertexBuffer};

fn main() {
    let el = EventLoop::new();
    let wb = WindowBuilder::new().with_title("shaderforge");

    let windowed_context = ContextBuilder::new()
        .with_vsync(true)
        .build_windowed(wb, &el)
        .unwrap();
    let display = Display::from_gl_window(windowed_context).unwrap();
    // let windowed_context = unsafe { windowed_context.make_current().unwrap() };
    println!(
        "Pixel format of window's GL context: {:?}",
        display.gl_window().get_pixel_format()
    );

    let bl = Vert {
        pos: [-0.5, -0.5],
        uv: [0.0, 0.0],
    };
    let tl = Vert {
        pos: [-0.5, 0.5],
        uv: [0.0, 1.0],
    };
    let br = Vert {
        pos: [0.5, -0.5],
        uv: [1.0, 0.0],
    };
    let tr = Vert {
        pos: [0.5, 0.5],
        uv: [1.0, 1.0],
    };
    let quad_strip = vec![bl, tl, br, tr];
    let vertex_buffer = VertexBuffer::new(&display, &quad_strip).unwrap();
    let indices = index::NoIndices(index::PrimitiveType::TriangleStrip);

    let vertex_shader_src = r#"
        #version 140

        in vec2 pos;
        out vec2 v_uv;

        void main() {
          gl_Position = vec4(pos, 0.0, 1.0);
        }
    "#;

    let fragment_shader_src = r#"
        #version 140

        in vec2 v_uv;
        out vec4 color;

        void main() {
          color = vec4(0.0, 1.0, 0.0, 1.0);
        }
    "#;
    let program =
        Program::from_source(&display, vertex_shader_src, fragment_shader_src, None).unwrap();

    el.run(move |event, _, control_flow| {
        let next_frame_time =
            std::time::Instant::now() + std::time::Duration::from_nanos(16_666_667);
        *control_flow = ControlFlow::WaitUntil(next_frame_time);

        match event {
            Event::LoopDestroyed => return,
            Event::WindowEvent { ref event, .. } => match event {
                WindowEvent::CloseRequested => {
                    *control_flow = ControlFlow::Exit;
                    return;
                }
                _ => return,
            },
            Event::NewEvents(cause) => match cause {
                StartCause::ResumeTimeReached { .. } => (),
                StartCause::Init => (),
                _ => return,
            },
            _ => return,
        }

        let mut target = display.draw();
        target.clear_color(0.0, 0.0, 1.0, 1.0);
        target
            .draw(
                &vertex_buffer,
                &indices,
                &program,
                &glium::uniforms::EmptyUniforms,
                &Default::default(),
            )
            .unwrap();
        target.finish().unwrap();
    });
}

#[derive(Clone, Copy)]
struct Vert {
    pos: [f32; 2],
    uv: [f32; 2],
}

implement_vertex!(Vert, pos, uv);
