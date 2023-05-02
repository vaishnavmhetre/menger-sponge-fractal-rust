use nannou::prelude::*;

use crate::utils::create_wireframe;

pub struct Space {
    window_id: window::Id,
    cubes: Vec<Cube>,
    rotation_angle: f32,
}

impl Space {
    pub fn new(window_id: window::Id, cubes: Vec<Cube>) -> Self {
        Self {
            window_id,
            cubes,
            rotation_angle: 0.,
        }
    }

    pub fn window_id(&self) -> WindowId {
        self.window_id
    }

    pub fn cubes(&self) -> &[Cube] {
        self.cubes.as_ref()
    }

    pub fn rotation_angle(&self) -> f32 {
        self.rotation_angle
    }

    pub fn window_id_mut(&mut self) -> &mut window::Id {
        &mut self.window_id
    }

    pub fn cubes_mut(&mut self) -> &mut Vec<Cube> {
        &mut self.cubes
    }

    pub fn set_window_id(&mut self, window_id: window::Id) {
        self.window_id = window_id;
    }

    pub fn set_cubes(&mut self, cubes: Vec<Cube>) {
        self.cubes = cubes;
    }

    pub fn set_rotation_angle(&mut self, rotation_angle: f32) {
        self.rotation_angle = rotation_angle;
    }

    pub fn update(&mut self, _app: &App, _update: Update) {
        self.set_rotation_angle(self.rotation_angle() + 0.005);
    }

    pub fn event(&mut self, _app: &App, event: Event) {
        if let Event::WindowEvent {
            id: _,
            simple: Some(window_event),
        } = event
        {
            if let WindowEvent::MousePressed(mouse_button) = window_event {
                if let MouseButton::Left = mouse_button {
                    let cubes = self
                        .cubes()
                        .iter()
                        .map(Cube::generate)
                        .flatten()
                        .collect::<Vec<Cube>>();
                    self.set_cubes(cubes);
                }
            }
        }
    }

    pub fn draw(&self, app: &App, frame: Frame) {
        frame.clear(BLACK);
        let draw = app.draw().radians(vec3(
            // global rotation
            (app.time * 0.1).sin() * 2.0,
            (app.time * 0.2).sin() * 2.0,
            (app.time * 0.3).sin() * 2.0,
        ));
        draw.background().color(BLACK);

        // let draw = draw.transform(Mat4::from_rotation_x(self.rotation_angle()));
        // let draw = draw.transform(Mat4::from_rotation_y(self.rotation_angle() + 0.002));
        // let draw = draw.transform(Mat4::from_rotation_z(self.rotation_angle() + 0.003));
        self.cubes().iter().for_each(|c| c.draw(app, &draw, &frame));

        draw.to_frame(app, &frame).unwrap();
    }
}

#[derive(Clone)]
pub struct Cube {
    x: f32,
    y: f32,
    z: f32,
    len: f32,
    cuboid: Cuboid,
    wireframe: Vec<Cuboid>,
}

impl Cube {
    pub fn new(x: f32, y: f32, z: f32, len: f32) -> Self {
        let cuboid = Cuboid::from_xyz_whd(vec3(x, y, z), vec3(len, len, len));

        let wireframe = create_wireframe(&cuboid);

        Self {
            x,
            y,
            z,
            len,
            cuboid,
            wireframe,
        }
    }

    pub fn x(&self) -> f32 {
        self.x
    }

    pub fn y(&self) -> f32 {
        self.y
    }

    pub fn z(&self) -> f32 {
        self.z
    }

    pub fn len(&self) -> f32 {
        self.len
    }

    pub fn cuboid(&self) -> Cuboid<f32> {
        self.cuboid
    }

    pub fn wireframe(&self) -> &[Cuboid<f32>] {
        self.wireframe.as_ref()
    }

    pub fn x_mut(&mut self) -> &mut f32 {
        &mut self.x
    }

    pub fn y_mut(&mut self) -> &mut f32 {
        &mut self.y
    }

    pub fn z_mut(&mut self) -> &mut f32 {
        &mut self.z
    }

    pub fn len_mut(&mut self) -> &mut f32 {
        &mut self.len
    }

    pub fn set_x(&mut self, x: f32) {
        self.x = x;
    }

    pub fn set_y(&mut self, y: f32) {
        self.y = y;
    }

    pub fn set_z(&mut self, z: f32) {
        self.z = z;
    }

    pub fn set_len(&mut self, len: f32) {
        self.len = len;
    }

    pub fn generate(&self) -> Vec<Cube> {
        let mut new_generation = Vec::<Cube>::new();
        for x in -1..2 {
            for y in -1..2 {
                for z in -1..2 {
                    let x = x as f32;
                    let y = y as f32;
                    let z = z as f32;
                    let sum = abs(x) + abs(y) + abs(z);
                    let new_len = self.len() / 3.;

                    if sum > 1. {
                        let cube = Cube::new(
                            (self.x() + x * new_len) + 10.,
                            (self.y() + y * new_len) + 10.,
                            (self.z() + z * new_len) + 10.,
                            new_len,
                        );
                        new_generation.push(cube)
                    }
                }
            }
        }

        new_generation
    }

    pub fn update(&self, _app: &App, _update: Update) {}

    pub fn draw(&self, _app: &App, draw: &Draw, _frame: &Frame) {
        let cuboid = self.cuboid();

        // draw cuboid 
        let cpoints = cuboid.triangles_iter().flat_map(geom::Tri::vertices);
        draw.mesh().points(cpoints).color(WHITESMOKE);

        // draw the wireframe
        self.wireframe().into_iter().for_each(|w| {
            let wpoints = w.triangles_iter().flat_map(geom::Tri::vertices);
            draw.mesh().points(wpoints).color(RED);
        });
    }
}
