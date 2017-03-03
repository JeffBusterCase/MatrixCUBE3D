#![allow(non_upper_case_globals)]
extern crate piston_window;
extern crate find_folder;

use piston_window::types::Color;

use piston_window::line::Shape;
use piston_window::{
    Glyphs,
    text,
    Line,
    CircleArc,
    Button,
    Key,
    PressEvent,
    ReleaseEvent,
    EventLoop,
    Transformed
};

use std::f64::consts::PI;

type Number = f64;

// Constants
const FPS: u64 = 60;
const UPS: u64 = 60;
const SZ: Number = 64.0;
const SPEED: Number = 0.012;
const FREE_SPEED: Number = SPEED / 40.0;
const WIDTH: Number = 620.0;
const HEIGHT: Number = 480.0;
const LRADIUS: Number = 3.5;
const EDGE_COLOR: Color = [1.0, 1.0, 1.0, 0.06];
const EDGE_COLOR_BLUR : Color = [1.0, 1.0, 1.0, 0.02];

fn main() {

    // statics
    static mut xo: Number = WIDTH / 2.0;
    static mut yo: Number = HEIGHT / 2.0;
    static mut x_rot: Number = PI / 4.0;
    static mut z_rot: Number = PI / 4.0;
    static mut left: bool = false;
    static mut right: bool = false;
    static mut up: bool = false;
    static mut down: bool = false;
    static mut lastzr: bool = true;
    static mut lastxr: bool = true;

    // variables
    let mut window: piston_window::PistonWindow;
    let mut pts: Vec<Vec<Number>> = Vec::new();
    
    struct Edge {
        x0: Number,
        y0: Number,
        xf: Number,
        yf: Number,
    }

    struct Vertice {
        x2d: Number,
        y2d: Number,
        color: Color,
    }

    pts.push(vec![SZ, SZ, SZ]);
    pts.push(vec![-SZ, SZ, SZ]);
    pts.push(vec![-SZ, -SZ, SZ]);
    pts.push(vec![SZ, -SZ, SZ]);

    pts.push(vec![SZ, SZ, SZ]);
    pts.push(vec![SZ, SZ, -SZ]);
    pts.push(vec![-SZ, SZ, -SZ]);
    pts.push(vec![-SZ, -SZ, -SZ]);

    pts.push(vec![SZ, -SZ, -SZ]);
    pts.push(vec![SZ, SZ, -SZ]);
    pts.push(vec![SZ, -SZ, -SZ]);
    pts.push(vec![SZ, -SZ, SZ]);

    pts.push(vec![-SZ, -SZ, SZ]);
    pts.push(vec![-SZ, -SZ, -SZ]);
    pts.push(vec![-SZ, SZ, -SZ]);
    pts.push(vec![-SZ, SZ, SZ]);

    unsafe fn to2d(x: Number, y: Number, z: Number) -> Vec<Number> {
        let x_cos = x_rot.cos();
        let z_cos = z_rot.cos();
        let x_sin = x_rot.sin();
        let z_sin = z_rot.sin();

        let xp = x * z_cos - y * z_sin;
        let yy = y * z_cos + x * z_sin;
        let yp = yy * x_cos + z * x_sin;
        vec![xp, yp]
    };

    unsafe fn add_vertices(pts: &Vec<Vec<Number>>) -> (Vec<Vec<Number>>, Vec<Vertice>) {
        let mut pix: Vec<Vec<Number>> = Vec::new();
        let mut vertices: Vec<Vertice> = Vec::new();

        for i in 0..pts.len() {
            let x = pts[i][0];
            let y = pts[i][1];
            let z = pts[i][2];
            let pt2d = to2d(x, y, z);
            let x2d = pt2d[0] + xo;
            let y2d = pt2d[1] + yo;

            let r: f32 = (x as f32) / (SZ as f32) * 55.0 + 200.0;
            let g: f32 = (y as f32) / (SZ as f32) * 55.0 + 200.0;
            let b: f32 = (z as f32) / (SZ as f32) * 55.0 + 200.0;
            let a: f32 = 1.0;

            vertices.push(Vertice {
                x2d: x2d,
                y2d: y2d,
                color: [r / 255.0, g / 255.0, b / 255.0, a] as Color,
            });
            pix.push(pt2d);
        }
        (pix, vertices)
    };

    unsafe fn add_edges(pix: Vec<Vec<Number>>, line_vec: &mut Vec<Edge>) {
        for i in 0..pix.len() - 1 {
            let x0 = pix[i][0] + xo;
            let y0 = pix[i][1] + yo;
            let xf = pix[i + 1][0] + xo;
            let yf = pix[i + 1][1] + yo;

            line_vec.push(Edge {
                x0: x0,
                y0: y0,
                xf: xf,
                yf: yf,
            });
        }
    };

    unsafe fn render(pts: &Vec<Vec<Number>>) -> (Vec<Edge>, Vec<Vertice>) {
        let mut edges: Vec<Edge> = Vec::new();
        let (pix, vertices) = add_vertices(pts);
        add_edges(pix, &mut edges);
        (edges, vertices)
    };

    window = piston_window::WindowSettings::new("Matrix 3D CUBE!", [WIDTH as u32, HEIGHT as u32])
        .exit_on_esc(true)
        .build()
        .unwrap();

    window.set_max_fps(FPS);
    window.set_ups(UPS);

    // Get font
    let assets = find_folder::Search::ParentsThenKids(3, 3)
        .for_folder("assets").unwrap();
    let ref font = assets.join("FiraSans-Regular.ttf");
    let factory = window.factory.clone();

    let mut glyphs  = Glyphs::new(font, factory).unwrap();

    // Render
    while let Some(e) = window.next() {
        unsafe {
            // Movement with Arrow Keys
            if let Some(Button::Keyboard(Key::Right)) = e.press_args() {
                right = true
            } else if let Some(Button::Keyboard(Key::Left)) = e.press_args() {
                left = true
            } else if let Some(Button::Keyboard(Key::Up)) = e.press_args() {
                up = true
            } else if let Some(Button::Keyboard(Key::Down)) = e.press_args() {
                down = true
            }

            if let Some(Button::Keyboard(Key::Right)) = e.release_args() {
                right = false
            } else if let Some(Button::Keyboard(Key::Left)) = e.release_args() {
                left = false
            } else if let Some(Button::Keyboard(Key::Up)) = e.release_args() {
                up = false
            } else if let Some(Button::Keyboard(Key::Down)) = e.release_args() {
                down = false
            }

            if left {
                z_rot += SPEED;
                lastzr = true;
            } else if right {
                z_rot -= SPEED;
                lastzr = false;

            } else if up {
                x_rot += SPEED;
                lastxr = true;

            } else if down {
                x_rot -= SPEED;
                lastxr = false;

            } else {
                // Continue to the last direction
                x_rot += if lastxr { FREE_SPEED } else { -FREE_SPEED };
                z_rot += if lastzr { FREE_SPEED } else { -FREE_SPEED };
            }
        }

        window.draw_2d(&e, |c, g| {
            piston_window::clear([0.0; 4], g);

            text::Text::new_color([1.0,1.0,1.0,1.0], 32).draw(
                "arrow keys to rotate",
                &mut glyphs,
                &c.draw_state,
                c.transform.trans(15.0, 50.0),
                g
            );

            unsafe {
                let (edges, vertices) = render(&pts);

                // Draw Edges
                for edge in edges {

                    // blur
                    Line::new([edge.x0 as f32, edge.y0 as f32, edge.xf as f32, edge.yf as f32],
                        LRADIUS+0.5)
                        .shape(Shape::Round)
                        .color(EDGE_COLOR_BLUR)
                        .draw_tri([edge.x0, edge.y0, edge.xf, edge.yf],
                            &c.draw_state,
                            c.transform,
                            g);

                    // original line
                    Line::new([edge.x0 as f32, edge.y0 as f32, edge.xf as f32, edge.yf as f32],
                              LRADIUS)
                        .shape(Shape::Round)
                        .color(EDGE_COLOR)
                        .draw_tri([edge.x0, edge.y0, edge.xf, edge.yf],
                              &c.draw_state,
                              c.transform,
                              g);

                }

                // Draw Vertices
                for vertice in vertices {
                    CircleArc::new(vertice.color, 5.0, 0.0, (4.0 * PI) - 0.0001)
                        .draw([vertice.x2d - 4.5, vertice.y2d - 4.5, 8.0, 8.0],
                              &c.draw_state,
                              c.transform,
                              g);
                }
            }
        });
    }

}
