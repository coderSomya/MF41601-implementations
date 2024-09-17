mod genetic;

use genetic::{Candidate, generate_population, generate_mating_pool, crossover, mutate};


fn main() {
    let x1min = -10.0;
    let x1max = 10.0;
    let x2min = -10.0;
    let x2max = 10.0;
    let pop_size = 10;
    let mutation_rate = 0.01;

    // Generate initial population
    let population = generate_population(x1min, x1max, x2min, x2max, pop_size);

    // Create mating pool
    let mating_pool = generate_mating_pool(&population, pop_size);

    // Perform crossover and mutation as required
    let (parent1, parent2) = (&mating_pool[0].code, &mating_pool[1].code);
    let (child1, child2) = crossover(parent1, parent2);

    let mut child1_mutable = child1.clone();
    mutate(&mut child1_mutable,  mutation_rate.clone());

    // Print results
    println!("Parent 1: {}", parent1);
    println!("Parent 2: {}", parent2);
    println!("Child 1: {}", child1);
    println!("Mutated Child 1: {}", child1_mutable);
}
