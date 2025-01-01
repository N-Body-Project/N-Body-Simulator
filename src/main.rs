use crate::types::particle::Particle;
use crate::types::nbodysystem::NBodySystem;

pub mod physics;
pub mod types;

fn main() {
    let mut system = NBodySystem::default();
    system.add_particle(Particle::new(0, [-50.0, 0.0, 0.0], [0.0, 0.0, 0.0], [0.0, 0.0, 0.0], 1.5e14));
    system.add_particle(Particle::new(1, [50.0, 0.0, 0.0], [0.0, 0.0, 0.0], [0.0, 0.0, 0.0], 1.5e14));
    system.add_particle(Particle::new(2, [100.0, 0.0, 0.0], [0.0, 0.0, 0.0], [0.0, 0.0, 0.0], 1.5e14));

    for i in 0..system.len() {
        println!("{:?}", system.get_particle_by_index(i).unwrap());
    }

    let forces = system.compute_all_forces();
    println!("{:?}", forces);
}
