#[derive(Debug)]
pub struct Money {
    bill_200: u8,
    bill_100: u8,
    bill_50: u8,
    bill_20: u8,
    bill_10: u8,
    bill_5: u8,
    coin_2_eu: u8,
    coin_1_eu: u8,
    coin_50_cents: u8,
    coin_20_cents: u8,
    coin_10_cents: u8,
    coin_5_cents: u8,
    coin_2_cents: u8,
    coin_1_cent: u8,
}

pub fn change(mut diff: f32) -> Money {
    let mut change = Money{
        bill_200: 0,
        bill_100: 0,
        bill_50: 0,
        bill_20: 0,
        bill_10: 0,
        bill_5: 0,
        coin_2_eu: 0,
        coin_1_eu: 0,
        coin_50_cents: 0,
        coin_20_cents: 0,
        coin_10_cents: 0,
        coin_5_cents: 0,
        coin_2_cents: 0,
        coin_1_cent: 0,
    };
    while diff > 0.01 {
        let tuple_return = return_type(change, diff);
        change = tuple_return.0;
        diff = tuple_return.1;
        println!("{}", diff);
    }
    change
}
fn return_type(mut money: Money, mut diff: f32) -> (Money, f32) {
    // ! The floating_point cannot be used in patterns. This will become an error
    match diff {
        200.0.. => {
            money.bill_200 += 1;
            diff -= 200.0;
            return (money, diff);
        },
        100.0..=199.00 => {
            money.bill_100 += 1;
            diff -= 100.0;
            return (money, diff);
        },
        50.0..=99.99 => {
            money.bill_50 += 1;
            diff -= 50.0;
            return (money, diff);
        },
        20.0..=49.99 => {
            money.bill_20 += 1;
            diff -= 20.0;
            return (money, diff);
        },
        10.0..=19.99 => {
            money.bill_10 += 1;
            diff -= 10.0;
            return (money, diff);
        },
        5.0..=9.99 => {
            money.bill_5 += 1;
            diff -= 5.0;
            return (money, diff);
        },
        2.0..=4.99 => {
            money.coin_2_eu += 1;
            diff -= 2.0;
            return (money, diff);
        },
        1.0..=1.99 => {
            money.coin_1_eu += 1;
            diff -= 1.0;
            return (money, diff);
        },
        0.5..=0.99 => {
            money.coin_50_cents += 1;
            diff -= 0.5;
            return (money, diff);
        },
        0.2..=0.49 => {
            money.coin_20_cents += 1;
            diff -= 0.2;
            return (money, diff);
        },
        0.1..=0.19 => {
            money.coin_10_cents += 1;
            diff -= 0.1;
            return (money, diff);
        },
        0.05..=0.1 => {
            money.coin_5_cents += 1;
            diff -= 0.05;
            return (money, diff);
        },
        0.02..=0.05 => {
            money.coin_2_cents += 1;
            diff -= 0.02;
            return (money, diff);
        },
        0.01..=0.02 => {
            money.coin_1_cent += 1;
            diff -= 0.01;
            return (money, diff);
        },
        _ => (money, diff),
    }
}