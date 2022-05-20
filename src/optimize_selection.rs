pub fn optimize_selection(
    rough_campaign_repeats:&Vec<(usize,usize)>,
    monthly_inventory:&u32,
    impressions_per_campaign:&Vec<i32>,
    revenue_per_campaign:&Vec<i32>
) -> Vec<(usize,usize)> {
    let stepbacks = stepbacks(rough_campaign_repeats);
    let reforwards = reforwards(&stepbacks, monthly_inventory, impressions_per_campaign);
    let ends = ends(&reforwards, &monthly_inventory, impressions_per_campaign);
    let best_selection = best_selection(&ends, revenue_per_campaign);
    best_selection
}

fn stepbacks(
    rough_campaign_repeats: &Vec<(usize, usize)>, 
) -> Vec<Vec<(usize,usize)>>{
    let n = rough_campaign_repeats.len();
    let mut stepbacks = vec![];
    for i in 0..n {
        if rough_campaign_repeats[i].1 > 0 {
            let mut stepback = rough_campaign_repeats.clone();
            stepback[i].1 -= 1;
            stepbacks.push(stepback);
        }
    }
    stepbacks
}

fn reforwards(
    stepbacks:&Vec<Vec<(usize,usize)>>, 
    monthly_inventory:&u32, 
    impressions_per_campaign:&Vec<i32>, 
) -> Vec<Vec<(usize,usize)>>{
    let mut reforwards = vec![];
    for stepback in stepbacks {
        let total_impressions = stepback.iter().map(|(i,repeats)| {
            impressions_per_campaign[*i] * (*repeats as i32)
        }).sum::<i32>();
        let remaining_inventory = (monthly_inventory.clone() as i32) - total_impressions;
        for i in 0..stepback.len() {
            let mut reforward = stepback.clone();
            reforward[i].1 += 1;
            if remaining_inventory - impressions_per_campaign[stepback[i].0] >= 0 {
                reforwards.push(reforward);
            }
        }
    }
    reforwards
}

fn ends(
    reforwards:&Vec<Vec<(usize,usize)>>, 
    monthly_inventory:&u32, 
    impressions_per_campaign:&Vec<i32>, 
) -> Vec<Vec<(usize,usize)>>{
    let mut ends = vec![];
    for path in reforwards {
        let mut end = path.clone();
        let total_impressions = path.iter().map(|(i,repeats)| {
            impressions_per_campaign[*i] * (*repeats as i32)
        }).sum::<i32>();
        let mut remaining_inventory = (monthly_inventory.clone() as i32) - total_impressions;
        for i in 0..end.len() {
            while remaining_inventory > 0 {
                end[i].1 += 1;
                remaining_inventory -= impressions_per_campaign[end[i].0];
            }
            if remaining_inventory < 0 {
                end[i].1 -= 1;
                remaining_inventory += impressions_per_campaign[end[i].0]
            }
        }
        if !ends.contains(&end) {
            ends.push(end);
        }        
    }
    ends
}

fn best_selection(ends:&Vec<Vec<(usize,usize)>>, revenue_per_campaign:&Vec<i32>) -> Vec<(usize,usize)>{
    let mut best_selection = ends[0].clone();
    let mut highest_revenue = ends[0].iter().map(|(a,b)| {
        revenue_per_campaign[*a] * (*b as i32)
    }).sum::<i32>();
    for end in ends {
        let revenue = end.iter().map(|(a,b)| {
            revenue_per_campaign[*a] * (*b as i32)
        }).sum::<i32>();
        if revenue > highest_revenue {
            highest_revenue = revenue;
            best_selection = end.clone();
        }
    }
    best_selection
}