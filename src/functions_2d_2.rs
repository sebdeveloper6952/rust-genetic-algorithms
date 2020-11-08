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
            x: rng.gen(),
            y: rng.gen(),
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
        self.fitness = 3.0 * self.x + 5.0 * self.y;
    }

    fn crossover(&self, other: &Individual2D) -> Individual2D {
        let mut i = Individual2D {
            fitness: 0.0,
            x: self.x + other.y,
            y: self.y + other.x
        };

        if i.x > 4.0 {
            i.x = 4.0;
        }
        
        if i.y > 6.0 {
            i.y = 6.0;
        }

        while 3.0 * i.x + 2.0 * i.y > 18.0 {
            i.x = i.x / 2.0;
            i.y = i.y / 2.0;
        }
        

        return i;
    }

    fn mutate(&mut self) {
        if rand::random() {
            let mut rng = rand::thread_rng();
            self.x += rng.gen_range(-5.0, 5.0);
            self.y += rng.gen_range(-5.0, 5.0);
        }

        if self.x > 4.0 {
            self.x = 4.0;
        }
        
        if self.y > 6.0 {
            self.y = 6.0;
        }

        while 3.0 * self.x + 2.0 * self.y > 18.0 {
            self.x = self.x / 2.0;
            self.y = self.y / 2.0;
        }
    }

    pub fn offspring(&self, other: &Individual2D) -> Individual2D {
        let mut new = self.crossover(other);
        new.mutate();
        new.update_fitness();

        return new;
    }
}
