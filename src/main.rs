use rayon::prelude::*;
use core_affinity::CoreId;
use rayon::ThreadPoolBuilder;
use core_affinity::get_core_ids;

// core_affinity::set_for_current(2);

#[derive(Clone, Copy)]
struct Position {
    qty: f64,
    price: f64,
}

#[derive(Clone)]
struct Account {
    positions: [Position; 10],
}

#[derive(Clone, Copy)]
struct Risk {
    exposure: f64,
    worst_pnl: f64,
}


fn calc_position_risk(position: &Position) -> Risk {

    let notional = position.qty * position.price;
    let stressed_price = position.price * 0.95;
    let worst_pnl = position.qty * (stressed_price - position.price);

    Risk {
        exposure: notional,
        worst_pnl,
    }

}

fn calc_account_risk(account: &Account) -> Risk {

    let mut exposure = 0.0;
    let mut worst_pnl = 0.0;

    for position in &account.positions {

        let risk = calc_position_risk(position);
        exposure += risk.exposure;
        worst_pnl += risk.worst_pnl;

    }

    Risk { exposure, worst_pnl }


}

fn compute_account_risks(accounts: &[Account]) -> Vec<Risk> {

    accounts
        .par_iter()
        .map(calc_account_risk)
        .collect()

    
}

fn compute_account_risks_seq(accounts: &[Account]) -> Vec<Risk> {
    accounts
        .iter()
        .map(calc_account_risk)
        .collect()
}



fn main(){


    const N_ACCOUNTS: usize = 1_000_000;
    // const N_ACCOUNTS: usize = 10;

    

    let accounts: Vec<Account> = (0..N_ACCOUNTS)
        .map(|i| Account {
            positions: [Position {
                qty: (i as f64 % 10.0) - 5.0,
                price: 100.0 + (i % 100) as f64,
            }; 10],
        })
        .collect();
    
    let cores = get_core_ids().expect("Failed to get core IDs");
    core_affinity::set_for_current(cores[3]);
    
    let allowed_cores = vec![
        cores[4],
        cores[5],
        cores[6],
        cores[7],
    ];

    let pool = ThreadPoolBuilder::new()
        .num_threads(4)
        .start_handler({
            let allowed_cores = allowed_cores.clone();
            move |i| {
                core_affinity::set_for_current(allowed_cores[i]);
                println!(
                    "Rayon worker {} pinned to core {:?}",
                    i, allowed_cores[i]
                );
            }
        })
        .build()
        .unwrap();

    pool.install(|| {
        let account_risks = compute_account_risks(&accounts);

        //  for (i, risk) in account_risks.iter().enumerate() {
        //                 println!(
        //                     "Account {} - Exposure: {:.2}, Worst PnL: {:.2}",
        //                     i + 1, 
        //                     risk.exposure,
        //                     risk.worst_pnl
        //                 );
        //             }

    })


}