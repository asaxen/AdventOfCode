use std::cmp::Ordering;
use std::env;
use std::fs;
use std::str::Split;

#[derive(Debug)]
struct Elf {
    id: u32,
    total_calories: u32
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let file_path = &args[1];

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    //println!("With text:\n{contents}");

    let lines: Split<&str> = contents.split("\n");
    let mut elfs: Vec<Elf> = Vec::new();
    elfs.push( Elf {id: 0, total_calories: 0} );

    for line in lines {
        if line.is_empty() {
            elfs.push( Elf { id: elfs.last().unwrap().id+1, total_calories: 0}); 
        }
        else {
            let old_elf = elfs.last().unwrap();
            let updated_elf = Elf {
                id: old_elf.id,
                total_calories: old_elf.total_calories + line.parse::<u32>().unwrap()
            };
            elfs.pop();
            elfs.push(updated_elf);
        }
    }

    elfs.sort_by(|a, b| {
        if a.total_calories < b.total_calories {
            Ordering::Greater
        } else if a.total_calories == b.total_calories {
            Ordering::Equal
        } else {
            Ordering::Less
        }
    });

    println!("Number of elfs: {}", elfs.len());

    println!("Solution first star. Elf with most calories: {:?}", elfs.first().unwrap());

    let total_calories_top_three: u32 = elfs[0..3].iter().map(|e| e.total_calories).sum();

    println!("Solution second star. Three top Elfs combined calories: {:?}",total_calories_top_three);


}