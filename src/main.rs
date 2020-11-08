mod tsp;
mod functions_2d;
mod functions_2d_2;
mod functions_2d_3;

// use tsp::Individual;

fn main() {
    // the population
    // let mut pop = Vec::new();

    // for _i in 0..100 {
    //     pop.push(Individual::new());
    // }

    // // sort population according to fitness
    // pop.sort_by_key(|a| a.fitness);

    // for _ in 0..100000 {
    //     // println!("iteration {}", i);

    //     // reproduction
    //     for i in 0..5 {
    //         let new = pop[i].crossover(&pop[i + 1]);
    //         pop.push(new);
    //     }

    //     // sort by fitness
    //     pop.sort_by_key(|a| a.fitness);

    //     // drop worst 5
    //     for _i in 0..5 {
    //         pop.pop();
    //     }
        
    //     // population at end of iteration
    //     // for individual in &pop {
    //     //     println!("{:?}", individual);
    //     // }
    //     // println!("");
    // }

    // println!("best individual {:?}", pop[0]);

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
