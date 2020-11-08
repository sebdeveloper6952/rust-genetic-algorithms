mod tsp;
mod functions_2d;
mod functions_2d_2;
mod functions_2d_3;

fn main() {

    // Ej. 1
    let mut pop = Vec::new();

    for _i in 0..100 {
        pop.push(functions_2d::Individual2D::new());
        // println!("{:?}", pop[pop.len() - 1]);
    }
    
    // initial sort of population by fitness
    pop.sort_by(|a, b| a.fitness().partial_cmp(&b.fitness()).unwrap());

    for _i in 0..1000000 {
        // selection and reproduction
        for i in 0..10 {
            let new = pop[i].offspring(&pop[i + 1]);
            pop.push(new);
        }

        pop.sort_by(|a, b| a.fitness().partial_cmp(&b.fitness()).unwrap());

        for _i in 0..10 {
            pop.remove(0);
        }
    }

    println!("\nTask 1 Best individual:");
    println!("{:?}", pop[pop.len() - 1]);

    // Ej. 2
    let mut pop = Vec::new();

    for _i in 0..100 {
        pop.push(functions_2d_2::Individual2D::new());
        // println!("{:?}", pop[pop.len() - 1]);
    }
    
    // initial sort of population by fitness
    pop.sort_by(|a, b| a.fitness().partial_cmp(&b.fitness()).unwrap());

    for _i in 0..1000000 {
        // selection and reproduction
        for i in 0..10 {
            let new = pop[i].offspring(&pop[i + 1]);
            pop.push(new);
        }

        pop.sort_by(|a, b| a.fitness().partial_cmp(&b.fitness()).unwrap());

        for _i in 0..10 {
            pop.remove(0);
        }
    }

    println!("\nTask 2 Best individual:");
    println!("{:?}", pop[pop.len() - 1]);

    // Ej. 3
    let mut pop = Vec::new();

    for _i in 0..100 {
        pop.push(functions_2d_3::Individual2D::new());
        // println!("{:?}", pop[pop.len() - 1]);
    }
    
    // initial sort of population by fitness
    pop.sort_by(|a, b| a.fitness().partial_cmp(&b.fitness()).unwrap());

    for _i in 0..1000000 {
        // selection and reproduction
        for i in 0..10 {
            let new = pop[i].offspring(&pop[i + 1]);
            pop.push(new);
        }

        pop.sort_by(|a, b| a.fitness().partial_cmp(&b.fitness()).unwrap());

        for _i in 0..10 {
            pop.remove(0);
        }
    }

    println!("\nTask 3 Best individual:");
    println!("{:?}", pop[pop.len() - 1]);
}
