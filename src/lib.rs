#[allow(dead_code)]
mod legendre {
    macro_rules! poly {
        () => {
            Box<Fn(f64) -> f64>
        };
    }

    pub struct Legendre {
        order: u32,
    }

    impl Legendre {
        pub fn new(n: u32) -> Legendre {
            Legendre { order: n }
        }

        pub fn order(&self) -> u32 {
            self.order
        }

        pub fn p(&self) -> poly!(){
            legendre_polynomial(self.order())
        }

        pub fn dp(&self) -> poly!(){
            legendre_polynomial_d(self.order())
        }
    }

    #[allow(unused_variables)]
    fn legendre_polynomial(n: u32) -> poly!(){
        let nd = n as f64;
        let a = (2.0 * nd - 1.0) / nd;
        let b = -(nd - 1.0) / nd;

        if n == 0 {
            Box::new(move |x| 1.0)
        } else if n == 1 {
            Box::new(move |x| x)
        } else {
            let f1 = legendre_polynomial(n - 1);
            let f2 = legendre_polynomial(n - 2);
            Box::new(move |x| a * x * f1(x) + b * f2(x))
        }
    }


    #[allow(unused_variables)]
    fn legendre_polynomial_d(n: u32) -> poly!(){
        let nd = n.clone() as f64;

        fn is_one(x: f64) -> bool {
            (x.abs() - 1.0).abs() < 1.0e-8
        }

        if n == 0 {
            Box::new(move |x| 0.0)
        } else if n == 1 {
            Box::new(move |x| 1.0)
        } else {
            Box::new(move |x| if is_one(x) {
                if n % 2 == 0 {
                    nd * (nd + 1.0) * 0.5 * x.signum()
                } else {
                    nd * (nd + 1.0) * 0.5
                }
            } else {
                let f = legendre_polynomial(n);
                let f1 = legendre_polynomial(n - 1);
                nd * (f1(x) - x * f(x)) / (1.0 - x.powi(2))
            })
        }
    }

}

#[cfg(test)]
#[allow(dead_code)]
mod tests {
    fn order3() {
        use legendre;
        let p3 = legendre::Legendre::new(3);
        let f = p3.p();
        assert!(f(5.0) - 305.0 < 1.0e-8);
    }
}
