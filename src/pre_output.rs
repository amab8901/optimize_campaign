pub fn pre_output(
    best_selection: &Vec<(usize,usize)>, 
    impressions_per_campaign:&Vec<i32>, 
    revenue_per_campaign:&Vec<i32>
) -> (Vec<i32>, Vec<i32>, Vec<i32>, i32, i32) {
    let total_impressions = total_impressions(best_selection, impressions_per_campaign);
    let total_revenues = total_revenues(best_selection, revenue_per_campaign);
    let total_campaigns = total_campaigns(best_selection, revenue_per_campaign);
    let total_impression = total_impression(&total_impressions);
    let total_revenue = total_revenue(&total_revenues);
    (
        total_impressions,
        total_revenues,
        total_campaigns,
        total_impression,
        total_revenue,
    )
}

fn total_impressions(best_selection:&Vec<(usize,usize)>, impressions_per_campaign:&Vec<i32>) -> Vec<i32>{
    let n = impressions_per_campaign.len();
    let mut total_impressions = vec![0;n];
    best_selection.iter().for_each(|(i,c)| {
        total_impressions[*i] = impressions_per_campaign[*i] * (*c as i32);
    });
    total_impressions
}

fn total_revenues(best_selection:&Vec<(usize,usize)>, revenue_per_campaign:&Vec<i32>) -> Vec<i32>{
    let n = revenue_per_campaign.len();
    let mut total_revenues = vec![0;n];
    best_selection.iter().for_each(|(i,c)| {
        total_revenues[*i] = revenue_per_campaign[*i] * (*c as i32)
    });

    total_revenues
}

fn total_campaigns(best_selection:&Vec<(usize,usize)>, revenue_per_campaign:&Vec<i32>) -> Vec<i32> {
    let n = revenue_per_campaign.len();
    let mut total_campaigns = vec![0;n];
    best_selection.iter().for_each(|(i,c)| {
        total_campaigns[*i] = *c as i32
    });
    total_campaigns
}

fn total_impression(total_impressions:&Vec<i32>) ->i32{
    total_impressions.iter().sum::<i32>()
}

fn total_revenue(total_revenues:&Vec<i32>) -> i32 {
    total_revenues.iter().sum::<i32>()
}