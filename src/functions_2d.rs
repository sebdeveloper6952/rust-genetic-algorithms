use rand::Rng;

#[derive(Debug)]
pub struct Individual2D {
    fitness: f64,
    x: f64,
    y: f64
}

impl Individual2D {
    pub fn new() -> Individual2D {
        let mut rng = rand::thread_rng();
        let mut i = Individual2D {
            fitness: 0.0,
            x: rng.gen_range(-1.0, 1.0),
            y: rng.gen_range(-1.0, 1.0),
        };

        i.update_fitness();
        
        return i;
    }

    pub fn new_with_chr(x: f64, y: f64) -> Individual2D {
        let mut i = Individual2D { fitness: 0.0, x, y };
        i.update_fitness();
        
        return i;
    }

    pub fn fitness(&self) -> f64 {
        self.fitness
    }

    fn update_fitness(&mut self) {
        self.fitness = 15.0 * self.x + 30.0 * self.y + 4.0 * self.x * self.y - 2.0 * self.x.powf(2.0) - 4.0 * self.y.powf(2.0);
    }

    fn crossover(&self, other: &Individual2D) -> Individual2D {
        Individual2D {
            fitness: 0.0,
            x: self.x + other.y,
            y: self.y + other.x
        }
    }

    fn mutate(&mut self) {
        if rand::random() {
            let mut rng = rand::thread_rng();
            self.x += rng.gen_range(-1.0, 1.0);
            self.y += rng.gen_range(-1.0, 1.0);
        }
    }

    pub fn offspring(&self, other: &Individual2D) -> Individual2D {
        let mut new = self.crossover(other);
        new.mutate();
        new.update_fitness();

        return new;
    }
}