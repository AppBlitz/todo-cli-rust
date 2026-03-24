use std::{fmt::format, fs::File, time};

fn main() {
    let days = 86400;
    let auxiliary = time::SystemTime::now()
        .duration_since(time::UNIX_EPOCH)
        .unwrap()
        .as_secs();
    let day = auxiliary / days;

    format_date(day);
}

fn format_date(days: u64) {
    let (year, day) = calculator_year(days);
    let (mounths, day_months) = calculator_months(day);
    let date: String =
        year.to_string() + "/" + &mounths.to_string() + "/" + &day_months.to_string();
    println!("date is:{:?}", date)
}
fn calculator_months(days: u64) -> (u64, u64) {
    let mut months: u64 = 1;
    let (year, day) = calculator_year(days);
    let mut aux_day = day;
    let mut aux_months = 1;
    while aux_months <= 12 && aux_day >= 31 {
        if aux_months == 2 {
            if verification_year(year) {
                aux_day -= 29;
                months += 1;
            } else {
                aux_day -= 28;
                months += 1;
            }
        } else if aux_months % 2 == 0 {
            aux_day -= 30;
            months += 1;
        } else {
            aux_day -= 31;
            months += 1;
        }
        aux_months += 1;
    }
    (months, aux_day + 1)
}

fn calculator_year(days: u64) -> (u64, u64) {
    let mut year: u64 = 1970;
    let mut auxiliary_days: u64 = days;
    while auxiliary_days >= 366 {
        if verification_year(year) {
            auxiliary_days -= 366;
            year += 1;
        } else {
            auxiliary_days -= 365;
            year += 1;
        }
    }
    (year, auxiliary_days)
}

fn verification_year(mut days: u64) -> bool {
    let mut auxiliary: bool = false;
    let mut sum_auxiliary: u64 = 0;
    while days != 0 {
        sum_auxiliary += days % 10;
        days /= 10;
    }
    if sum_auxiliary % 4 == 0 || sum_auxiliary % 400 == 0 {
        auxiliary = true
    }
    auxiliary
}
