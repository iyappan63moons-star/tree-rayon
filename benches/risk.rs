use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use rayon::prelude::*;
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
fn calc_account_risk(account: &Account) -> Risk {
    let mut exposure = 0.0;
    let mut worst_pnl = 0.0;

    for p in &account.positions {
        let notional = p.qty * p.price;
        exposure += notional;

        let stressed_price = p.price * 0.95;
        worst_pnl += p.qty * (stressed_price - p.price);
    }

    Risk { exposure, worst_pnl }
}



#[inline(always)]
fn merge_risk(a: Risk, b: Risk) -> Risk {
    Risk {
        exposure: a.exposure + b.exposure,
        worst_pnl: a.worst_pnl + b.worst_pnl,
    }
}



fn compute_total_risk(accounts: &[Account]) -> Risk {
    accounts
        .par_iter()
        .map(calc_account_risk)
        .reduce(
            || Risk {
                exposure: 0.0,
                worst_pnl: 0.0,
            },
            merge_risk,
        )
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

    let pool = ThreadPoolBuilder::new()
        .num_threads(8) 
        .build()
        .unwrap();

    c.bench_with_input(
        BenchmarkId::new("risk_tree", N_ACCOUNTS),
        &accounts,
        |b, accounts| {
            b.iter(|| {
                pool.install(|| {
                    let _ = compute_total_risk(accounts);
                })
            });
        },
    );
}

criterion_group!(benches, risk_benchmark);
criterion_main!(benches);
