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

    pub fn loss(&self) -> bool {
        self.n_red > self.n_blu
    }
}

#[derive(Clone, Copy, Default, PartialEq, Eq)]
pub enum Button {
    #[default]
    Blue, // default *uninitialized* value
    Red,
}

pub struct Actor {
    dna: f64,
    lifetime: u32,
}

impl Actor {
    const LIFETIME: u32 = 50;
    const VARIATION_COEFFICIENT: f64 = 0.07;
    const FERTILITY: f64 = 0.03;

    pub fn new(dna: f64) -> Self {
        Self {
            dna,
            lifetime: Self::LIFETIME,
        }
    }

    pub fn breed(&self) -> Option<Self> {
        let f: f64 = rand::thread_rng().gen(); // generates [0, 1)
        if f > Self::FERTILITY {
            return None;
        }

        let v: f64 = rand::thread_rng().sample(Open01); // generates (0, 1);
        let v = (v - 0.5) * 2. * Self::VARIATION_COEFFICIENT;
        let v = self.dna + v;
        Some(Self::new(v.clamp(0., 1.)))
    }

    pub fn select_button(&self) -> Button {
        let r: f64 = rand::thread_rng().gen(); // generates [0, 1)

        // [0, f) - red
        // [f, 1) - blue
        if r < self.dna {
            Button::Red
        } else {
            Button::Blue
        }
    }

    pub fn life(&mut self) -> bool {
        self.lifetime -= 1;
        self.lifetime != 0
    }
}

pub fn spawn_random_actors() -> impl Iterator<Item = Actor> {
    use std::iter;

    let mut rng = rand::thread_rng();
    iter::repeat_with(move || {
        let f = rng.sample(Open01); // generates (0, 1)
        Actor::new(f)
    })
}
