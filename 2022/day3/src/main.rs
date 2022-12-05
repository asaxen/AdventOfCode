use std::env;
use std::fs;
use std::collections::HashSet;

fn compute_priorities(found_chars:&mut Vec<char>) -> u32 {
    found_chars.sort_by(|a, b| b.cmp(a));

    println!("Priorities: {}", found_chars.iter().collect::<String>());

    let ascii_a_z_range:std::ops::Range<u32> = 96..123;
    let priorities_total:u32 = found_chars.into_iter()
        .map(|item: &mut char| *item as u32)
        .map(|item| if ascii_a_z_range.contains(&item) {item-97+1} else {item-65+27})
        .sum();
         
    println!("Priorities total sum: {}", priorities_total);
    return priorities_total;
}

fn part1(lines:&Vec<&str>) {

    let mut found_items:Vec<char> = Vec::new();
 
    for line in lines {
        if !line.is_empty() {
            let mut unique_items = HashSet::new();
            let compartment1:String = line[0..line.len()/2].to_string();
            for item in compartment1.to_string().chars() 
            {
                unique_items.insert(item);
            };
            
            let compartment2:String = line[line.len()/2..line.len()].to_string();
            for item in compartment2.to_string().chars() 
            {
                if unique_items.contains(&item) {
                    println!("Total rucksack: {} ({})", line, line.len());
                    println!("Compartment1: {} ({}), Compartment2: {} ({})", compartment1, compartment1.len(), compartment2, compartment2.len());
                    println!("Duplicate found: {}", item);
                    found_items.push(item);
                    break;
                }
            };
        }
    }
    let total_priority_score = compute_priorities(&mut found_items);

    println!("Solution first star. Total score: {}", total_priority_score);
}

fn part2(lines:&Vec<&str>) {
    let mut found_badges:Vec<char> = Vec::new();
    for i in (0..lines.len()).step_by(3) {
        if !lines[i].is_empty() {
            let first_elf = lines[i];
            let second_elf = lines[i+1];
            let third_elf = lines[i+2];

            for item in first_elf.to_string().chars() 
            {
                //Check if elf 2 rucksack
                if second_elf.contains(item) && third_elf.contains(item) {
                    found_badges.push(item);
                    println!("Found badge for {first_elf} {second_elf} {third_elf}!");
                    println!("Badge: {item}");
                    break
                }
            };

            let total_priority_score = compute_priorities(&mut found_badges);
            println!("Solution second star. Total score: {}", total_priority_score);
        }
    };
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let file_path = &args[1];

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    let lines: Vec<&str>= contents.split("\n").collect();

    part1(&lines);
    part2(&lines);

}