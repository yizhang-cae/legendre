#[allow(dead_code)]
mod legendre {
    #[allow(unused_variables)]
    pub fn legendre(n: u32) -> Box<Fn(f64) -> f64> {
        let nd = n as f64;
        let a = (2.0 * nd - 1.0) / nd;
        let b = -(nd - 1.0) / nd;

        if n == 0 {
            Box::new(move |x| 1.0)
        } else if n == 1 {
            Box::new(move |x| x)
        } else {
            let f1 = legendre(n - 1);
            let f2 = legendre(n - 2);
            Box::new(move |x| a * x * f1(x) + b * f2(x))
        }
    }
}

#[cfg(test)]
#[allow(dead_code)]
mod tests {
    fn order3() {
        use legendre;
        let f = legendre::legendre(3);
        assert!(f(5.0) - 305.0 < 1.0e-8);
    }
}
