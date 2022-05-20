
pub fn rough_selection(impressions_per_campaign:&Vec<i32>, revenue_per_campaign:&Vec<i32>, monthly_inventory: &u32) -> 
    Vec<(usize, usize)> 
{
    let impressions_per_revenue: Vec<f64> = impressions_per_revenue(impressions_per_campaign, revenue_per_campaign);
    let qualified_customers: Vec<i32> = qualified_customers(revenue_per_campaign, impressions_per_campaign, &impressions_per_revenue);
    let customer_priority:Vec<usize> = customer_priority(&qualified_customers, &impressions_per_revenue);
    let rough_campaign_repeats:Vec<(usize, usize)> = rough_campaign_repeats(&customer_priority, monthly_inventory, impressions_per_campaign);
    rough_campaign_repeats
}

fn impressions_per_revenue(impressions_per_campaign:&Vec<i32>, revenue_per_campaign:&Vec<i32>) -> Vec<f64> {
    impressions_per_campaign.iter().zip(revenue_per_campaign.iter()).map(|(a,b)|
        if *b != 0 { (*a as f64) / (*b as f64) } else {-1_f64}
    ).collect::<Vec<f64>>()
}

fn qualified_customers(revenue_per_campaign:&Vec<i32>, impressions_per_campaign:&Vec<i32>, impressions_per_revenue:&Vec<f64>) -> Vec<i32>{
    let n = revenue_per_campaign.clone().len();
    let mut customers = vec![1;n];
    (0..n).for_each(|i|{
        (i+1..n).for_each(|j|{
            if revenue_per_campaign[i as usize]==0 {
                customers[i as usize] = -1;
            } else if impressions_per_revenue[i]==impressions_per_revenue[j] {
                if impressions_per_campaign[i]>impressions_per_campaign[j] {customers[i] = -1}
                if impressions_per_campaign[i]<impressions_per_campaign[j] {customers[j] = -1}
                if impressions_per_campaign[i]==impressions_per_campaign[j] {customers[i] = -1}
            }
        });
    });
    let mut qualified_customers = customers.clone();
    for i in 0..n {
        qualified_customers[i] = match customers[i] {
            x if x>0 => 1,
            _ => -1,
        }
    }
    qualified_customers
}

fn customer_priority(qualified_customers: &Vec<i32>, impressions_per_revenue:&Vec<f64>) -> Vec<usize>{
    let valid_impressions_per_revenue = impressions_per_revenue.iter()
        .enumerate()
        .filter(|(i,_v)| {
            qualified_customers[*i] as i32 > 0
        }).map(|(i,v)|(*v,i))
        .collect::<Vec<(f64, usize)>>();
    let mut valid_impressions_per_revenue = valid_impressions_per_revenue.iter().map(|(v,i)|{
        ( (*v*10000_f64) as i32 ,*i)
    }).collect::<Vec<(i32, usize)>>();
    valid_impressions_per_revenue.sort();
    let customer_priority = valid_impressions_per_revenue.into_iter()
        .map(|(_v,i)| i)
        .collect::<Vec<usize>>();
    customer_priority
}

fn rough_campaign_repeats(customer_priority:&Vec<usize>, monthly_inventory:&u32, impressions_per_campaign:&Vec<i32>) -> Vec<(usize, usize)>{
    let mut monthly_inventory = monthly_inventory.clone() as i32;
    let n = customer_priority.clone().len();
    let zero_vector = vec![0;n];
    let count = customer_priority.iter()
        .zip(zero_vector.iter())
        .map(|(a,b)| (a,*b as usize))
        .collect::<Vec<(&usize, usize)>>();
    let mut count= count.into_iter().map(|(a,b)|
        (*a, b)
    ).collect::<Vec<(usize, usize)>>();
    for i in 0..n {
        while monthly_inventory > 0 {
            count[i].1 += 1;
            monthly_inventory -= impressions_per_campaign[count[i].0];
        }
        if monthly_inventory < 0 {
            count[i].1 -= 1;
            monthly_inventory += impressions_per_campaign[count[i].0];
        }
    }
    count.iter().map(|(a,b)| (*a, *b)).collect::<Vec<(usize, usize)>>()
}