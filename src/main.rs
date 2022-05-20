mod parse_input;
mod rough_selection;
mod optimize_selection;
mod pre_output;
fn main() {
    let (
        monthly_inventory, 
        customers, 
        impressions_per_campaign, 
        revenue_per_campaign
    ) = parse_input::parse_input();

    let rough_campaign_repeats = rough_selection::rough_selection(
        &impressions_per_campaign, &revenue_per_campaign, &monthly_inventory
    );

    let best_selection = optimize_selection::optimize_selection(
        &rough_campaign_repeats, &monthly_inventory, &impressions_per_campaign, &revenue_per_campaign
    );

    let (
        total_impressions,
        total_revenues,
        total_campaigns,
        total_impression,
        total_revenue
    ) = pre_output::pre_output(&best_selection, &impressions_per_campaign, &revenue_per_campaign);

    let output_lines = output_lines(
        &customers, &total_campaigns, &total_impressions, &total_revenues, &total_impression, &total_revenue
    );
    
    for line in &output_lines {
        println!("{}", line);
    }
}

fn output_lines(
    customers:&Vec<String>, 
    total_campaigns:&Vec<i32>, 
    total_impressions:&Vec<i32>, 
    total_revenues:&Vec<i32>,
    total_impression:&i32,
    total_revenue:&i32
) -> Vec<String>{
    let mut output_lines = vec![];
    let n = customers.len();
    for i in 0..n {
        let line = format!("<{}>,<{}>,<{}>,<{}>", customers[i], total_campaigns[i], total_impressions[i], total_revenues[i]);
        output_lines.push(line);
    }
    let last_line = format!("<{}>,<{}>", total_impression, total_revenue);
    output_lines.push(last_line);
    output_lines
}
