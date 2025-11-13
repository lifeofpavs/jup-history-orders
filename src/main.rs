use std::env;

use jup_history_orders::*;

#[tokio::main]
async fn main() {
    //NOTE:  Assuming all swaps where done between token and USDC

    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        panic!("You should pass the user pubkkey and asset mint as the two first arguments");
    }

    let client = reqwest::Client::new();

    let pk = &args[1];
    let output_asset = &args[2];
    let mut has_more = true;
    let mut page = 1;

    let mut history: Vec<Activity> = vec![];

    while has_more {
        println!("Running for page: {:?}", page);

        let url = format!("https://worker.jup.ag/v2/activities/{}", pk);
        let result: JupActivityResponse = client
            .get(&url)
            .query(&[
                ("product", "SWAP"),
                ("mintFilter", output_asset),
                ("page", &page.to_string()),
            ])
            .send()
            .await
            .unwrap()
            .json()
            .await
            .unwrap();

        if result.has_more_data {
            page += 1;
        }

        has_more = result.has_more_data;
        println!("Found {} activities in history", result.histories.len());
        result.histories.into_iter().for_each(|a| history.push(a));
        println!("There are {} activities in history", history.len());
    }

    let output_mint_swaps: Vec<Activity> = history
        .clone()
        .into_iter()
        .filter(|f| f.output_mint == *output_asset)
        .collect();

    let total_input: f64 = output_mint_swaps
        .iter()
        .filter_map(|f| f.input_amount.parse::<f64>().ok())
        .sum();

    let total_output: f64 = output_mint_swaps
        .iter()
        .filter_map(|f| f.output_amount.parse::<f64>().ok())
        .sum();

    let cost_basis_per_unit: f64 = if total_output != 0.0 {
        total_input / total_output
    } else {
        0.0
    };

    println!("Cost basis computation for output swaps:");
    println!("Total Quote Input Spent: {}", total_input);
    println!("Total Quote Output Spent: {}", total_output);
    println!(
        "Average Cost Basis per Output on Quote:  {}",
        cost_basis_per_unit
    );
    println!(
        "Number of swaps for Quote {} :  {}",
        output_asset,
        output_mint_swaps.len()
    );
}
