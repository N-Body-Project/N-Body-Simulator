use crate::types::nbodysystem::NBodySystem;
use clap::Parser;

/// N Body Problem
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// Print the velocities and coordinates
    #[arg(short, long)]
    output: bool,

    /// Limit the output of information about particles to once every N steps
    #[arg(short, long, default_value_t = 1000)]
    limit: u16,
}

pub struct NBodyCli<'a> {
    m_args: Args,
    m_n_body_system: &'a mut NBodySystem,
}

impl<'a> NBodyCli<'a> {
    pub fn new(n_body_system: &'a mut NBodySystem) -> Self {
        Self {
            m_args: Args::parse(),
            m_n_body_system: n_body_system,
        }
    }

    pub fn handle_args(&mut self) {
        self.m_n_body_system.set_output(self.m_args.output);
    }
}
