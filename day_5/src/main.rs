use std::fs::File;
use std::io::Read;

fn parse_int(str_int: &str) -> u64 {
    str_int.parse::<u64>().unwrap()
}

struct Map {
    source: u64,
    dest: u64,
    range: u64,
}

struct Translation {
    maps: Vec<Map>,
}

impl Translation {
    fn from_str(mapping_str: &str) -> Self {
        let mut lines = mapping_str.lines();
        let _title = lines.next();

        let mut maps = vec![];
        for line in lines {
            let mut nums = line.split(' ');

            let dest = parse_int(nums.next().unwrap());
            let source = parse_int(nums.next().unwrap());
            let range = parse_int(nums.next().unwrap());

            maps.push(Map {
                source,
                dest,
                range,
            })
        }
        Translation { maps }
    }

    fn get(&self, key: &u64) -> u64 {
        for map in &self.maps {
            if map.source <= *key && *key < map.source + map.range {
                return map.dest + (key - map.source);
            }
        }
        *key
    }

    fn get_rev(&self, key: &u64) -> u64 {
        for map in &self.maps {
            if map.dest <= *key && *key < map.dest + map.range {
                return map.source + (key - map.dest);
            }
        }
        *key
    }
}

fn main() {
    let mut f = File::open("data/input.txt").unwrap();
    let mut input = String::new();
    f.read_to_string(&mut input).unwrap();

    let mut catagories = input.split("\n\n");

    let seeds: Vec<u64> = catagories
        .next()
        .unwrap()
        .split(":")
        .last()
        .unwrap()
        .trim()
        .split(" ")
        .map(|x| parse_int(x))
        .collect();

    fn seed_in_range(seeds: &Vec<u64>, seed: u64) -> bool {
        let mut s = seeds.iter();
        while let Some(start) = s.next() {
            let range = s.next().unwrap();
            if *start <= seed && seed < *start + range {
                return true;
            }
        }
        false
    }

    let seed_to_soil = Translation::from_str(catagories.next().unwrap());
    let soil_to_fert = Translation::from_str(catagories.next().unwrap());
    let fert_to_water = Translation::from_str(catagories.next().unwrap());
    let water_to_light = Translation::from_str(catagories.next().unwrap());
    let light_to_temp = Translation::from_str(catagories.next().unwrap());
    let temp_to_hum = Translation::from_str(catagories.next().unwrap());
    let hum_to_loc = Translation::from_str(catagories.next().unwrap());

    // part 1
    let mut min_loc = std::u64::MAX;
    for seed in &seeds {
        let soil = seed_to_soil.get(&seed);
        let fert = soil_to_fert.get(&soil);
        let water = fert_to_water.get(&fert);
        let light = water_to_light.get(&water);
        let temp = light_to_temp.get(&light);
        let hum = temp_to_hum.get(&temp);
        let loc = hum_to_loc.get(&hum);

        if loc < min_loc {
            min_loc = loc;
        }
    }
    println!("part 1: {}", min_loc);

    // part 2
    let mut loc = 1;
    loop {
        let hum = hum_to_loc.get_rev(&loc);
        let temp = temp_to_hum.get_rev(&hum);
        let light = light_to_temp.get_rev(&temp);
        let water = water_to_light.get_rev(&light);
        let fert = fert_to_water.get_rev(&water);
        let soil = soil_to_fert.get_rev(&fert);
        let seed = seed_to_soil.get_rev(&soil);

        if seed_in_range(&seeds, seed) {
            println!("Part 2: {}", loc);
            break;
        }

        loc += 1;
    }
}
