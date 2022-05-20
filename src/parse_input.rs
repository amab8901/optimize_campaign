use std::env;
use std::fs;

pub fn parse_input() -> (u32, Vec<String>, Vec<i32>, Vec<i32>) {
    let contents_vector = contents_vector();
    (  
        monthly_inventory(&contents_vector),
        customers(&contents_vector),
        impressions_per_campaign(&contents_vector),
        revenue_per_campaign(&contents_vector)
    )
}


fn contents_vector() -> Vec<String> {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let contents = fs::read_to_string(filename)
        .expect("Can't read input file");
    contents.split("\n").map(|x|x.to_owned() ).collect::<Vec<String>>()
}


fn monthly_inventory(contents_vector: &Vec<String>) -> u32 {
    contents_vector.iter().next()
        .map(|x| 
            x.chars().filter(|ch| 
                ch.to_string()!="<".to_string() && ch.to_string()!=">".to_string()
            ).collect::<String>().parse::<u32>().expect("failed to parse inventory from String to u32")
        )
        .expect("failed to extract monthly inventory")
}

fn customers(contents_vector: &Vec<String>) -> Vec<String>{
    contents_vector[1..].iter()
        .map(|line| 
            line.split(",").next().expect("failed to cut non-customer info").to_string() ) 
        .map(|customer| 
            customer.chars().filter(|ch|
                ch.to_string() != ">".to_string() && ch.to_string() != "<".to_string()
            ).collect::<String>()
        )
        .collect::<Vec<String>>()
}

fn impressions_per_campaign(contents_vector: &Vec<String>) -> Vec<i32> {
    contents_vector[1..].iter()
        .map(|line|
            line.split(",").nth(1).expect("failed to cut non-impressions info").to_string() )
        .map(|imp_count|
            imp_count.chars().filter(|ch|
                ch.to_string() != ">".to_string() && ch.to_string() != "<".to_string()
            ).collect::<String>().parse::<i32>().expect("failed to parse impressions to i32")
        )
        .collect::<Vec<i32>>()
}

fn revenue_per_campaign(contents_vector: &Vec<String>) -> Vec<i32> {
    contents_vector[1..].iter()
        .map(|line|
            line.split(",").nth(2).expect("failed to cut non-impressions info").to_string() )
        .map(|imp_count|
            imp_count.chars().filter(|ch|
                ch.to_string() != ">".to_string() && ch.to_string() != "<".to_string()
            ).collect::<String>().parse::<i32>().expect("failed to parse impressions to i32")
        )
        .collect::<Vec<i32>>()
}