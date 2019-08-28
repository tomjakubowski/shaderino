use glium::glutin::{
    self,
    event::{Event, StartCause, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
    ContextBuilder, GlRequest,
};

use glium::{implement_vertex, index, uniform, Display, Program, Surface, VertexBuffer};

fn main() {
    let el = EventLoop::new();
    let wb = WindowBuilder::new().with_title("shaderforge");

    let windowed_context = ContextBuilder::new()
        .with_gl(GlRequest::Specific(glutin::Api::OpenGlEs, (2, 0)))
        .with_vsync(true)
        .build_windowed(wb, &el)
        .unwrap();
    let winsize = windowed_context.window().inner_size();
    let display = Display::from_gl_window(windowed_context).unwrap();

    #[derive(Clone, Copy)]
    struct Vert {
        pos: [f32; 2],
    }

    implement_vertex!(Vert, pos);
    let bl = Vert { pos: [-1.0, -1.0] };
    let tl = Vert { pos: [-1.0, 1.0] };
    let br = Vert { pos: [1.0, -1.0] };
    let tr = Vert { pos: [1.0, 1.0] };
    let quad_strip = vec![bl, tl, br, tr];
    let vertex_buffer = VertexBuffer::new(&display, &quad_strip).unwrap();
    let indices = index::NoIndices(index::PrimitiveType::TriangleStrip);

    let vertex_shader_src = r#"
        in vec2 pos;

        void main() {
          gl_Position = vec4(pos, 0.0, 1.0);
        }
    "#;

    let fragment_shader_src = r#"
        #ifdef GL_ES
        precision mediump float;
        #endif

        uniform vec2 u_resolution;
        uniform float u_time;

        void main() {
            vec2 st = gl_FragCoord.xy/u_resolution.xy;
            st.x *= u_resolution.x/u_resolution.y;

            vec3 color = vec3(0.);
            color = vec3(st.x,st.y,abs(sin(u_time)));

            gl_FragColor = vec4(color,1.0);
        }
    "#;
    let program =
        Program::from_source(&display, vertex_shader_src, fragment_shader_src, None).unwrap();

    #[derive(Debug)]
    struct State {
        resolution: [f32; 2],
    }
    let mut state = State {
        resolution: [winsize.width as f32, winsize.height as f32],
    };

    let epoch = std::time::Instant::now();

    el.run(move |event, _, control_flow| {
        let now = std::time::Instant::now();
        let next_frame_time = now + std::time::Duration::from_nanos(16_666_667);
        *control_flow = ControlFlow::WaitUntil(next_frame_time);

        match event {
            Event::LoopDestroyed => return,
            Event::WindowEvent { ref event, .. } => match event {
                WindowEvent::CloseRequested => {
                    *control_flow = ControlFlow::Exit;
                    return;
                }
                WindowEvent::RedrawRequested => (),
                WindowEvent::Resized(size) => {
                    state.resolution = [size.width as f32, size.height as f32];
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

        let since_epoch = now.duration_since(epoch);
        let u_time = since_epoch.as_secs() as f64 + (since_epoch.subsec_micros() as f64 * 1e-6);

        let uniforms = uniform! {
            u_resolution: state.resolution,
            u_time: u_time as f32,
        };
        target
            .draw(
                &vertex_buffer,
                &indices,
                &program,
                &uniforms,
                &Default::default(),
            )
            .unwrap();
        target.finish().unwrap();
    });
}
