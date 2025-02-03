const N: usize = 1000;
type Dict = Vec<Option<usize>>;

fn g(n: usize, dg: &mut Dict) -> usize {
    if let Some(g) = dg[n] {
        return g;
    }
    let g = n - g(g(n - 1, dg), dg);
    dg[n] = Some(g);
    g
}

fn f(n: usize, dm: &mut Dict, df: &mut Dict) -> usize {
    if let Some(f) = df[n] {
        return f;
    }
    let f = n - m(f(n - 1, dm, df), dm, df);
    df[n] = Some(f);
    f
}

fn m(n: usize, dm: &mut Dict, df: &mut Dict) -> usize {
    if let Some(m) = dm[n] {
        return m;
    }
    let m = n - f(m(n - 1, dm, df), dm, df);
    dm[n] = Some(m);
    m
}

fn h(n: usize, dh: &mut Dict) -> usize {
    if let Some(h) = dh[n] {
        return h;
    }
    let h = n - h(h(h(n - 1, dh), dh), dh);
    dh[n] = Some(h);
    h
}

fn main() {
    let dg = &mut vec![None; N];
    let dm = &mut vec![None; N];
    let df = &mut vec![None; N];
    let dh = &mut vec![None; N];

    dg[0] = Some(0);
    df[0] = Some(1);
    dm[0] = Some(0);
    dh[0] = Some(0);

    for i in 1..N {
        println!("g: {}, f: {}, m: {}, h: {}", g(i, dg), f(i, dm, df), m(i, dm, df), h(i, dh));
    }

     println!("φ ≈ {}", N as f64 / f(N - 1, dm, df) as f64);
     println!("ψ ≈ {}", N as f64 / h(N - 1, dh) as f64);
}
