use glium::glutin::{
    self,
    event::{Event, StartCause, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
    ContextBuilder,
};
use glium::{implement_vertex, index, uniform, Display, Program, Surface, VertexBuffer};
use std::{fs, path::PathBuf, process};
use structopt::StructOpt;

mod emu;

fn main() {
    let opts = Opts::from_args();

    let fragment_shader_src = match fs::read_to_string(&opts.file) {
        Ok(src) => src,
        Err(e) => {
            eprintln!("Error reading {}: {}", opts.file.display(), e);
            process::exit(1);
        }
    };

    let el = EventLoop::new();
    let title = if cfg!(debug_assertions) {
        "shaderino-debug".to_string()
    } else {
        "shaderino".to_string()
    };
    let mut wb = WindowBuilder::new();
    #[cfg(unix)]
    {
        use glutin::platform::unix::WindowBuilderExtUnix;
        wb = wb.with_class(title.clone(), title.clone());
    }
    wb = wb.with_title(title);

    let windowed_context = ContextBuilder::new()
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

    let program =
        Program::from_source(&display, vertex_shader_src, &fragment_shader_src, None).unwrap();

    let mut vm = emu::GlslViewer::new();
    vm.set_resolution(winsize.width as f32, winsize.height as f32);

    el.run(move |event, _, control_flow| {
        // Returning from this match block ensures that the scene isn't drawn more than once per
        // frame.  control_flow is sticky, so the timer still finishes at the correct time after
        // the event loop is woken up for a window event other than CloseRequested (which
        // mutates control_flow).
        match event {
            Event::LoopDestroyed => return,
            Event::WindowEvent { ref event, .. } => match event {
                WindowEvent::CloseRequested => {
                    *control_flow = ControlFlow::Exit;
                    return;
                }
                WindowEvent::KeyboardInput { input, .. } => {
                    if input.virtual_keycode == Some(glutin::event::VirtualKeyCode::Q) {
                        *control_flow = ControlFlow::Exit;
                        return;
                    }
                }
                WindowEvent::RedrawRequested => (),
                WindowEvent::Resized(size) => {
                    vm.set_resolution(size.width as f32, size.height as f32);
                    return;
                }
                WindowEvent::CursorMoved { position, .. } => {
                    let x = position.x as f32;
                    let y = vm.resolution()[1] - position.y as f32;
                    vm.set_mouse(x, y);
                    return;
                }
                _ => return,
            },
            Event::NewEvents(cause) => match cause {
                StartCause::ResumeTimeReached { .. } => {}
                StartCause::Init => (),
                _ => return,
            },
            _ => return,
        }

        vm.update();
        let next_frame_time = vm.time() + std::time::Duration::from_nanos(16_666_667);
        *control_flow = ControlFlow::WaitUntil(next_frame_time);

        let mut target = display.draw();
        target.clear_color(0.0, 0.0, 0.0, 1.0);

        target
            .draw(
                &vertex_buffer,
                &indices,
                &program,
                &vm.uniforms(),
                &Default::default(),
            )
            .unwrap();
        target.finish().unwrap();
    });
}

#[derive(StructOpt, Debug)]
#[structopt(name = "shaderino")]
struct Opts {
    #[structopt(name = "FILE", parse(from_os_str))]
    file: PathBuf,
}
