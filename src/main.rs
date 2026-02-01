use rayon::prelude::*;
use rayon::ThreadPoolBuilder;


#[derive(Clone, Copy, Debug)]
struct Position {
    qty: f64,
    price: f64,
}

#[derive(Clone, Copy, Debug)]
struct Account {
    positions: &'static [Position],
}

#[derive(Clone, Copy, Debug)]
struct Risk {
    exposure: f64,
    worst_pnl: f64,
}


#[inline(always)]
fn calc_account_risk(account: &Account) -> Risk {
    let mut exposure = 0.0;
    let mut worst_pnl = 0.0;

    for p in account.positions {
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



fn main() {
    

    let pool = ThreadPoolBuilder::new()
        .num_threads(4) 
        .thread_name(|i| format!("risk-{}", i))
        .build()
        .expect("failed to build rayon pool");

   
    static POSITIONS_A: [Position; 3] = [
        Position { qty: 10.0, price: 100.0 },
        Position { qty: -5.0, price: 200.0 },
        Position { qty: 2.0, price: 150.0 },
    ];

    static POSITIONS_B: [Position; 2] = [
        Position { qty: 1.0, price: 10_000.0 },
        Position { qty: -0.5, price: 20_000.0 },
    ];

    let accounts = vec![
        Account { positions: &POSITIONS_A },
        Account { positions: &POSITIONS_B },
        Account { positions: &POSITIONS_A },
        Account { positions: &POSITIONS_B },
    ];

    let total_risk = pool.install(|| compute_total_risk(&accounts));

    println!("=== TOTAL RISK ===");
    println!("Exposure  : {:.2}", total_risk.exposure);
    println!("Worst PnL : {:.2}", total_risk.worst_pnl);


}
