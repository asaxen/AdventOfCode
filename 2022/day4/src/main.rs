use std::env;
use std::fs;


fn part1(lines:&Vec<&str>) {
    let mut number_of_pairs_with_complete_overlapp:u16 = 0;
    // Find smallest 
    for line in lines {
        if !line.is_empty() {
            let pair:Vec<&str> = line.split(",").collect();
            let elf1:Vec<u16> = pair[0].split("-").map(|p| p.parse().unwrap()).collect();
            let elf2:Vec<u16> = pair[1].split("-").map(|p| p.parse().unwrap()).collect();
            println!("Elf1: {:?} Elf2:{:?}", elf1, elf2);
            //if [elf1[0]..elf1[1]].contains()
            if (elf2[0]..=elf2[1]).contains(&elf1[1]) && (elf2[0]..=elf2[1]).contains(&elf1[0]) {
                println!("Elf1 range in Elf2");
                number_of_pairs_with_complete_overlapp += 1;
            } else if (elf1[0]..=elf1[1]).contains(&elf2[1]) && (elf1[0]..=elf1[1]).contains(&elf2[0]) {
                println!("Elf2 range in Elf1");
                number_of_pairs_with_complete_overlapp += 1;
            } else {
                println!("No complete overlap found");
            }
        }
    }

    println!("Solution first star. Total score: {}", number_of_pairs_with_complete_overlapp);
}

fn part2(lines:&Vec<&str>) {
    let mut number_of_pairs_with_complete_overlapp:u16 = 0;
    // Find smallest 
    for line in lines {
        if !line.is_empty() {
            let pair:Vec<&str> = line.split(",").collect();
            let elf1:Vec<u16> = pair[0].split("-").map(|p| p.parse().unwrap()).collect();
            let elf2:Vec<u16> = pair[1].split("-").map(|p| p.parse().unwrap()).collect();
            println!("Elf1: {:?} Elf2:{:?}", elf1, elf2);

            if (elf2[0]..=elf2[1]).contains(&elf1[1]) || (elf2[0]..=elf2[1]).contains(&elf1[0]) {
                println!("Elf1 range partial in Elf2");
                number_of_pairs_with_complete_overlapp += 1;
            } else if (elf1[0]..=elf1[1]).contains(&elf2[1]) || (elf1[0]..=elf1[1]).contains(&elf2[0]) {
                println!("Elf2 range partial in Elf1");
                number_of_pairs_with_complete_overlapp += 1;
            } else {
                println!("No overlap found");
            }
        }
    }

    println!("Solution first star. Total score: {}", number_of_pairs_with_complete_overlapp);
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