use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use rayon::prelude::*;
use core_affinity::CoreId;
use rayon::ThreadPoolBuilder;



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

#[inline(always)]
fn calc_position_risk(position: &Position) -> Risk {

    let notional = position.qty * position.price;
    let stressed_price = position.price * 0.95;
    let worst_pnl = position.qty * (stressed_price - position.price);

    Risk {
        exposure: notional,
        worst_pnl,
    }

}

#[inline(always)]
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


fn risk_benchmark(c: &mut Criterion) {

    const N_ACCOUNTS: usize = 1_000_000;

    let accounts: Vec<Account> = (0..N_ACCOUNTS)
        .map(|i| Account {
            positions: [Position {
                qty: (i as f64 % 10.0) - 5.0,
                price: 100.0 + (i % 100) as f64,
            }; 10],
        })
        .collect();

    let cores = core_affinity::get_core_ids().expect("No cores found");

    let pool = ThreadPoolBuilder::new()
        .num_threads(8) 
        .start_handler(move |thread_index| {
            let core = cores[thread_index % cores.len()];
            core_affinity::set_for_current(core);
        })
        .build()
        .unwrap();

    c.bench_with_input(
        BenchmarkId::new("account_risk", N_ACCOUNTS),
        &accounts,
        |b, accounts| {
            b.iter(|| {
                pool.install(|| {

                    let _account_risks = compute_account_risks(accounts);

                    // for (i, risk) in account_risks.iter().enumerate() {
                    //     println!(
                    //         "Account {} - Exposure: {:.2}, Worst PnL: {:.2}",
                    //         i + 1,  // Account index (1-based)
                    //         risk.exposure,
                    //         risk.worst_pnl
                    //     );
                    // }

                })
            });
        },
    );

}

criterion_group!(benches, risk_benchmark);
criterion_main!(benches);
