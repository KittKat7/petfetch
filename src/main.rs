/**
 * (c) 2025 KittKat
 */

use std::env;
use std::collections::HashMap;
mod pets;
mod colors;

use colors::{get_color_code, get_color_set};
use os_info;
// use sysinfo::System;
use sysinfo::System;


fn main() {
    let osinfo = os_info::get();

    let mut info_map: HashMap<String, String> = HashMap::new();

    let args: Vec<String> = env::args().collect();

    let info_items: [String; 4] = [
        "TITLE".to_string(),
        "OS".to_string(),
        "KERNEL".to_string(),
        "CPU".to_string()];

    let mut sys = System::new();
    sys.refresh_cpu_all();

    let cpu_brand: String;
    if let Some(cpu) = sys.cpus().first() {
        cpu_brand = cpu.brand().to_string();
    } else {
        cpu_brand = "CPU info not available".to_string();
    } 
    // OS info
    info_map.insert(
        "OS".to_string(),
        osinfo.to_string()
    );
    
    let kernel: String;
    let kv = System::kernel_version();
    if !kv.is_none() {
        kernel = kv.unwrap();
    } else {
        kernel = "Kernel info not available".to_string();
    }

    // KERNEL info
    info_map.insert(
        "KERNEL".to_string(),
        kernel
    );

    // CPU info
    info_map.insert(
        "CPU".to_string(),
        cpu_brand
    );

    let mut pet = "cat";
    let mut color = "";

    for i in 0..args.len() {
        let n: &str = &args[i];
        match n {
            "-p"|"--pet"    => pet = &args[i + 1],
            "-c"|"--color"  => color = &args[i + 1],
            _ => continue
        }
    }

    info_map.insert(
        "TITLE".to_string(),
        ("Petfetch - ".to_string() + &pet).to_string()
    );
    
    // let binding = fs::read_to_string(pet.to_string()).expect("Should have read a file");
    let binding = pets::get_pet(pet);
    let logo = binding.0.split("\n");
    let mut line_length = 0;
    let mut lines = 0;
    for n in logo {
        lines += 1;
        if n.len() > line_length {
            line_length = n.len();
        }
    }

    let logo = binding.0.split("\n");
    let colors;
    if color == "" {
        colors = get_color_set(binding.1);
    } else {
        colors = get_color_set(color);
    }

    println!();
    let mut i = 0;
    let color_vec = colors.split(",").collect::<Vec<&str>>();
    let mut c = "whi";
    let r = get_color_code("res");
    let color_ratio: f32 = lines as f32 / color_vec.len() as f32;
    for n in logo {
        if color_vec.len() > 0 {

            let x: i32 = (i as f32 / color_ratio) as i32;
            c = get_color_code(color_vec.get(x as usize).unwrap());
            
        }
        let mut s = n.to_string();
        while s.len() < line_length {
            s = s.to_string() + " ";
        }
        let mut info_str = "";
        if i < info_items.len() {
            info_str = info_map.get(&info_items[i]).unwrap();
        }
        println!(" {c}{s}{r} : {}", info_str);
        i += 1;
    }
    println!("{}", get_color_code("res"));
}


