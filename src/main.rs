extern crate konicars;
#[macro_use]
extern crate glium;

use std::{thread, time};
use std::env;

fn main() {
    init_connection(env::args_os().skip(1)).unwrap();

    use glium::{glutin, Surface};

    let mut events_loop = glutin::EventsLoop::new();
    let mut events_loop = glutin
        
    let window = glutin::WindowBuilder::new();
    let context = glutin::ContextBuilder::new().with_depth_buffer(24).with_pixel_format(30, 2);
    let display = glium::Display::new(window, context, &events_loop).unwrap();

    #[derive(Copy, Clone)]
    struct Vertex {
        position: [f32; 2],
    }

    implement_vertex!(Vertex, position);

    let vertex1 = Vertex { position: [-0.5, -0.5] };
    let vertex2 = Vertex { position: [0.0, 0.5] };
    let vertex3 = Vertex { position: [0.5, -0.25] };
    let shape = vec![vertex1, vertex2, vertex3];

    let vertex_buffer = glium::VertexBuffer::new(&display, &shape).unwrap();
    let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

    let vertex_shader_src = r#"
        #version 140
        in vec2 position;
        void main() {
            vec2 pos = position;
            gl_Position = vec4(pos, 0.0, 1.0);
        }
    "#;

    let fragment_shader_src = r#"
        #version 140
        out vec4 color;
        uniform vec4 c;
        void main() {
            color = c;
        }
    "#;

    let program =
        glium::Program::from_source(&display, vertex_shader_src, fragment_shader_src, None)
            .unwrap();

    // let vals = (0f64..1024f64).collect();

    let mut t = 0.5f64;
    // let mut c = 0;
    let mut closed = false;
    while !closed {
        t += 1. / 1024.;
        let mut target = display.draw();
        target.clear_color(0.0, 0.0, 0.0, 1.0);
        let c = [t as f32, t as f32, t as f32, 1.0];
        // let c = [vals[c] as f32, vals[c] as f32, vals[c] as f32, 1.0];
        let uniforms = uniform! { c: c };
        target.draw(&vertex_buffer,
                  &indices,
                  &program,
                  &uniforms,
                  &Default::default())
            .unwrap();
        target.finish().unwrap();

        let cd = measure(&mut port).unwrap();

        events_loop.poll_events(|event| match event {
            glutin::Event::WindowEvent { event, .. } => {
                match event {
                    glutin::WindowEvent::Closed => closed = true,
                    _ => (),
                }
            }
            _ => (),
        });
    }

    set_nd_filter(&mut port, 0).unwrap();
    println!("disconnected");
}
