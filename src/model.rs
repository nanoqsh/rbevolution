use rand::{distributions::Open01, Rng};

#[derive(Clone, Copy, Default)]
pub struct Choice {
    pub n_blu: u32,
    pub n_red: u32,
}

impl Choice {
    pub fn push(&mut self, button: Button) {
        match button {
            Button::Blue => self.n_blu += 1,
            Button::Red => self.n_red += 1,
        }
    }
}

pub enum Button {
    Blue,
    Red,
}

pub struct Actor(f64);

impl Actor {
    const VARIATION_COEFFICIENT: f64 = 0.05;

    fn breed(&self) -> Self {
        let v: f64 = rand::thread_rng().sample(Open01); // generates (0, 1);
        let v = (v - 0.5) * 2. * Self::VARIATION_COEFFICIENT;
        let v = self.0 + v;
        Self(v.clamp(0., 1.))
    }

    pub fn select_button(&self) -> Button {
        let r: f64 = rand::thread_rng().gen(); // generates [0, 1)
        let f = self.0;

        // [0, f) - red
        // [f, 1)   - blue
        if r < f {
            Button::Red
        } else {
            Button::Blue
        }
    }
}

pub fn spawn_random_actors() -> impl Iterator<Item = Actor> {
    use std::iter;

    let mut rng = rand::thread_rng();
    iter::repeat_with(move || {
        let v = rng.sample(Open01); // generates (0, 1)
        Actor(v)
    })
}
