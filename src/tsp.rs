use rand::Rng;
use std::collections::HashMap;

#[derive(Debug)]
struct Individual {
    fitness: u32,
    chromosome: Vec<u32>
}

impl Individual {
    fn new() -> Individual {
        let mut v = Vec::new();
        let mut rng = rand::thread_rng();

        while v.len() < 6 {
            let r = rng.gen_range(0, 6);
            if !v.contains(&r) {
                v.push(r);
            }
        }
        
        let mut i = Individual {
            fitness: 0,
            chromosome: v,
        };

        i.fitness();

        return i;
    }

    fn new_with_chromosome(chr: Vec<u32>) -> Individual {
        let mut i = Individual::new();
        i.chromosome = chr;
        i.fitness();

        return i;
    }

    fn fitness(&mut self) {
        let mut dist = HashMap::new();
        dist.insert(0, vec![0, 20, 15, 30, 52, 47]);
        dist.insert(1, vec![20, 0, 19, 27, 21, 25]);
        dist.insert(2, vec![15, 19, 0, 26, 43, 33]);
        dist.insert(3, vec![30, 27, 26, 0, 29, 10]);
        dist.insert(4, vec![52, 21, 43, 29, 0, 19]);
        dist.insert(5, vec![47, 25, 33, 10, 19, 0]);
        
        let mut fitness = 0;
        for i in 0..self.chromosome.len() - 1 {
            let src = self.chromosome[i];
            let dst = self.chromosome[i + 1];
            fitness += dist[&(src as usize)][dst as usize];
        }
        self.fitness = fitness;
    }

    fn crossover(&self, other: &Individual) -> Individual {
        // take first 3 of self, then missing from other
        let mut new_chr = Vec::new();
        new_chr.push(self.chromosome[0]);
        new_chr.push(self.chromosome[1]);
        new_chr.push(self.chromosome[2]);

        for phenotype in &other.chromosome {
            if !new_chr.contains(&phenotype) {
                new_chr.push(*phenotype);
            }
        }

        return Individual::new_with_chromosome(new_chr);
    }
}