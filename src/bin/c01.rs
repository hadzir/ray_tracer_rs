use ray_tracer::tuple::*;

fn main() {
    let dt = 0.01;
    let environment = Environment::new(
        VTuple::vector(0.0, -9.81, 0.0),
        VTuple::vector(0.0, 0.0, 0.02),
    );
    println!("Environment => {:?}", environment);
    let mut projectile =
        Projectile::new(VTuple::point(0.0, 1.0, 0.0), VTuple::vector(0.02, 0.0, 0.0));

    let mut t = 0.0;
    while projectile.position.y > 0.0 {
        t += dt;
        println!("{:.2} | {:?}", t, projectile.position);
        projectile = tick(projectile, &environment, dt);
    }
    println!("Finito => {:.2}", t);
}
#[derive(Debug)]
struct Environment {
    gravity: VTuple,
    wind: VTuple,
}
impl Environment {
    pub fn new(gravity: VTuple, wind: VTuple) -> Self {
        Environment { gravity, wind }
    }
}
#[derive(Debug)]
struct Projectile {
    position: VTuple,
    velocity: VTuple,
}
impl Projectile {
    pub fn new(position: VTuple, velocity: VTuple) -> Self {
        Projectile { position, velocity }
    }
}

fn tick(projectile: Projectile, environment: &Environment, dt: f64) -> Projectile {
    Projectile::new(
        projectile.position + projectile.velocity * dt,
        projectile.velocity + environment.gravity * dt + environment.wind * dt,
    )
}
