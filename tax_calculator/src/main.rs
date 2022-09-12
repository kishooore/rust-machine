
#[derive(Debug)]
struct TaxSlab {
    lower_limit: u128,
    upper_limit: u128,
    percentage: u8,
}

impl TaxSlab {
    fn new(lower_limit: u128, upper_limit: u128, percentage: u8) -> TaxSlab {
        TaxSlab {
            lower_limit,
            upper_limit,
            percentage,
        }
    }
}

fn main() {
    let slabs = new_regime();
    let mut taxable_salary = 1370000;
    let mut total_tax = 0;
    for slab in slabs {
        let diff = slab.upper_limit - slab.lower_limit;
        if taxable_salary > diff {
            taxable_salary -= diff;
            total_tax += diff * (slab.percentage as u128) / 100;
        } else {
            total_tax += taxable_salary * (slab.percentage as u128) / 100;
            break;
        }
    }
    println!("total tax: {}", total_tax);
}

fn new_regime() -> Vec<TaxSlab> {
    let mut slabs: Vec<TaxSlab> = Vec::new();
    let slab = TaxSlab::new(1, 250000, 0);
    slabs.push(slab);
    let slab = TaxSlab::new(250001, 500000, 5);
    slabs.push(slab);
    let slab = TaxSlab::new(500001, 750000, 10);
    slabs.push(slab);
    let slab = TaxSlab::new(750001, 1000000, 15);
    slabs.push(slab);
    let slab = TaxSlab::new(1000001, 1250000, 20);
    slabs.push(slab);
    let slab = TaxSlab::new(1250000, 1500000, 25);
    slabs.push(slab);
    slabs
}

