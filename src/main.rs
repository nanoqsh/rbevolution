mod chart;
mod model;

use crate::model::{Actor, Button, Choice};

struct Population {
    actors: Vec<(Actor, Button)>,
}

impl Population {
    fn start(n: usize) -> Self {
        Self {
            actors: model::spawn_random_actors()
                .take(n)
                .map(|a| (a, Button::default()))
                .collect(),
        }
    }

    fn select(&mut self) -> impl Iterator<Item = Button> + '_ {
        self.actors.iter_mut().map(|(a, c)| {
            let button = a.select_button();
            *c = button;
            button
        })
    }

    fn kill_blue(&mut self) {
        use rand::Rng;

        const CHANCE: f64 = 0.3;

        let f: f64 = rand::thread_rng().gen(); // generates [0, 1)
        self.actors.retain(|&(_, button)| {
            if button == Button::Blue {
                f >= CHANCE
            } else {
                true
            }
        })
    }

    fn life(&mut self) {
        self.actors.retain_mut(|(actor, _)| actor.life())
    }

    fn breed(&mut self) {
        let mut brood = Vec::with_capacity(self.actors.len());
        for (actor, _) in &self.actors {
            brood.extend(actor.breed());
        }

        self.actors
            .extend(brood.into_iter().map(|a| (a, Button::default())));
    }
}

fn main() {
    let mut population = Population::start(200);
    let mut choices = vec![];
    for _ in 0..400 {
        let mut choise = Choice::default();
        for button in population.select() {
            choise.push(button);
        }

        if choise.loss() {
            population.kill_blue();
        }

        population.life();
        population.breed();
        choices.push(choise);
    }

    crate::chart::render(choices);
}
