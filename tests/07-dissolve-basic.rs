//! First tests for new `dissolve` functionality
use serde::{Serialize, Deserialize};
use derive_getters::{Dissolve, DissolveMut, DissolveRef};

#[derive(Dissolve, DissolveMut, DissolveRef, Serialize, Deserialize)]
struct Number {
    num: u64,
}

#[derive(Copy, Clone, PartialEq, Eq, Serialize, Deserialize)]
struct Inner {
    a: u64,
    b: i64,
}

#[derive(Dissolve, DissolveMut, DissolveRef, Serialize, Deserialize)]
struct ManyStuff {
    name: String,
    price: f64,
    count: usize,
    inner: Inner,
}

#[derive(Dissolve, DissolveMut, DissolveRef, Serialize, Deserialize)]
#[dissolve(rename = "shatter")]
#[dissolve_mut(rename = "shatter_mut")]
#[dissolve_ref(rename = "shatter_ref")]
struct LotsOfStuff {
    name: String,
    price: f64,
    count: usize,
    inner: Inner,
}

impl LotsOfStuff {
    fn dissolve(&self) -> f64 {
        self.inner.b as f64 * self.price
    }
    fn dissolve_ref(&self) {}
    fn dissolve_mut(&mut self) {}
}

fn main() {
    let mut n = Number { num: 64 };

    let number_mut = n.dissolve_mut();
    *number_mut += 1;
    assert!(*number_mut == 65);
    *number_mut -= 1;
    
    let number_ref = n.dissolve_ref();
    assert!(*number_ref == 64);
    
    let number = n.dissolve();
    assert!(number == 64);

    let inner = Inner { a: 22, b: -33 };
    let mut stuff = ManyStuff {
        name: "Hogie".to_owned(),
        price: 123.4f64,
        count: 100,
        inner,
    };

    let (n, p, c, i): (&mut String, &mut f64, &mut usize, &mut Inner) = stuff.dissolve_mut();
    *n = "Bogie".to_owned();
    *p = 432.1f64;
    *c = 1;
    i.b = 4;
    assert!(*n == "Bogie");
    assert!(*p == 432.1f64);
    assert!(*c == 1);
    assert!(i.a == inner.a);
    assert!(i.b == 4);
    *n = "Hogie".to_owned();
    *p = 123.4f64;
    *c = 100;
    i.b = -33;

    let (n, p, c, i) = stuff.dissolve_ref();
    assert!(*n == "Hogie");
    assert!(*p == 123.4f64);
    assert!(*c == 100);
    assert!(*i == inner);
    let (n, p, c, i) = stuff.dissolve();
    assert!(n == "Hogie");
    assert!(p == 123.4f64);
    assert!(c == 100);
    assert!(i == inner);
    
    //let _ = stuff.dissolve();

    let mut stuff = LotsOfStuff {
        name: "Hogie".to_owned(),
        price: 123.4f64,
        count: 100,
        inner,
    };

    let (n, p, c, i) = stuff.shatter_mut();
    *n = "Bogie".to_owned();
    *p = 432.1f64;
    *c = 1;
    i.b = 4;
    assert!(*n == "Bogie");
    assert!(*p == 432.1f64);
    assert!(*c == 1);
    assert!(i.a == inner.a);
    assert!(i.b == 4);
    *n = "Hogie".to_owned();
    *p = 123.4f64;
    *c = 100;
    i.b = -33;

    let (n, p, c, i) = stuff.shatter_ref();
    assert!(*n == "Hogie");
    assert!(*p == 123.4f64);
    assert!(*c == 100);
    assert!(*i == inner);
    let (n, p, c, i) = stuff.shatter();
    assert!(n == "Hogie");
    assert!(p == 123.4f64);
    assert!(c == 100);
    assert!(i == inner);

    //let _ = stuff.shatter();
}
