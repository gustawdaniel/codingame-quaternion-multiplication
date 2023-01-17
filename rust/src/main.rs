use std::io;
use regex::Regex;

#[cfg(test)]
mod tests {
    use crate::{Quaternion, SignedCoefficient};

    #[test]
    fn simple_parse() {
        let qs = Quaternion::parse("(i+j)");
        assert_eq!(qs.len(), 1);
        assert_eq!(qs[0].r, 0f64);
        assert_eq!(qs[0].i, 1f64);
        assert_eq!(qs[0].j, 1f64);
        assert_eq!(qs[0].k, 0f64);
    }

    #[test]
    fn complex_parse() {
        let qs = Quaternion::parse("(9+i-j)(k-8.4j)");
        assert_eq!(qs.len(), 2);
        assert_eq!(qs[0].r, 9f64);
        assert_eq!(qs[0].i, 1f64);
        assert_eq!(qs[0].j, -1f64);
        assert_eq!(qs[0].k, 0f64);

        assert_eq!(qs[1].r, 0f64);
        assert_eq!(qs[1].i, 0f64);
        assert_eq!(qs[1].j, -8.4f64);
        assert_eq!(qs[1].k, 1f64);
    }

    #[test]
    fn multiply_base() {
        assert_eq!(Quaternion::multiply_base('r', 'r'), SignedCoefficient { c: 1f64, d: 'r' });
        assert_eq!(Quaternion::multiply_base('r', 'i'), SignedCoefficient { c: 1f64, d: 'i' });
        assert_eq!(Quaternion::multiply_base('r', 'j'), SignedCoefficient { c: 1f64, d: 'j' });
        assert_eq!(Quaternion::multiply_base('r', 'k'), SignedCoefficient { c: 1f64, d: 'k' });

        assert_eq!(Quaternion::multiply_base('i', 'r'), SignedCoefficient { c: 1f64, d: 'i' });
        assert_eq!(Quaternion::multiply_base('i', 'i'), SignedCoefficient { c: -1f64, d: 'r' });
        assert_eq!(Quaternion::multiply_base('i', 'j'), SignedCoefficient { c: 1f64, d: 'k' });
        assert_eq!(Quaternion::multiply_base('i', 'k'), SignedCoefficient { c: -1f64, d: 'j' });

        assert_eq!(Quaternion::multiply_base('j', 'r'), SignedCoefficient { c: 1f64, d: 'j' });
        assert_eq!(Quaternion::multiply_base('j', 'i'), SignedCoefficient { c: -1f64, d: 'k' });
        assert_eq!(Quaternion::multiply_base('j', 'j'), SignedCoefficient { c: -1f64, d: 'r' });
        assert_eq!(Quaternion::multiply_base('j', 'k'), SignedCoefficient { c: 1f64, d: 'i' });

        assert_eq!(Quaternion::multiply_base('k', 'r'), SignedCoefficient { c: 1f64, d: 'k' });
        assert_eq!(Quaternion::multiply_base('k', 'i'), SignedCoefficient { c: 1f64, d: 'j' });
        assert_eq!(Quaternion::multiply_base('k', 'j'), SignedCoefficient { c: -1f64, d: 'i' });
        assert_eq!(Quaternion::multiply_base('k', 'k'), SignedCoefficient { c: -1f64, d: 'r' });
    }

    #[test]
    fn simple_multiply() {
        let res = Quaternion::new(vec![String::from("1")])
            .multiply(Quaternion::new(vec![String::from("1")]));

        assert_eq!(res, Quaternion {
            r: 1f64,
            i: 0f64,
            j: 0f64,
            k: 0f64
        })
    }

    #[test]
    fn complex_multiply() {
        let res = Quaternion::new(vec![String::from("2i"), String::from("2j")])
            .multiply(Quaternion::new(vec![String::from("j"), String::from("1")]));

        assert_eq!(res, Quaternion {
            r: -2f64,
            i: 2f64,
            j: 2f64,
            k: 2f64
        })
    }
}

#[derive(Debug)]
struct Quaternion {
    r: f64,
    i: f64,
    j: f64,
    k: f64,
}

impl PartialEq<Quaternion> for Quaternion {
    fn eq(&self, other: &Quaternion) -> bool {
        self.r == other.r && self.i == other.i && self.j == other.j && self.k == other.k
    }
}

#[derive(Debug)]
struct SignedCoefficient {
    c: f64,
    d: char,
}

impl PartialEq<SignedCoefficient> for SignedCoefficient {
    fn eq(&self, other: &SignedCoefficient) -> bool {
        self.c == other.c && self.d == other.d
    }
}

impl Quaternion {
    fn parse(input: &str) -> Vec<Quaternion> {
        let re = Regex::new(r"\((.*?)\)").expect("can't create regex");

        let qs = re.captures_iter(input).filter_map(|cap| Some(cap.get(1)?.as_str()))
            .map(|m| m.to_string()).collect::<Vec<_>>();

        let re = Regex::new(r"\+?(-?[\d.]*[ijk]?)").expect("can't create regex");

        let res = qs.iter().map(|q| {
            let args = re.captures_iter(&q).filter_map(|cap| Some(cap.get(1)?.as_str()))
                .map(|m| m.to_string()).collect::<Vec<_>>();
            Quaternion::new(args)
        });

        res.collect::<Vec<_>>()
    }

    fn get_coefficient(t: &str, input: String) -> f64 {
        let c = input.replace(t, "");
        if Regex::new(r"\d$").expect("ff").is_match(&c) {
            c.parse::<f64>().unwrap()
        } else {
            (c + "1").parse::<f64>().unwrap()
        }
    }

    fn new(args: Vec<String>) -> Quaternion {
        let mut q = Quaternion {
            i: 0f64,
            j: 0f64,
            k: 0f64,
            r: 0f64,
        };

        for arg in args {
            if arg.contains("i") {
                q.i = Quaternion::get_coefficient("i", arg)
            } else if arg.contains("j") {
                q.j = Quaternion::get_coefficient("j", arg)
            } else if arg.contains("k") {
                q.k = Quaternion::get_coefficient("k", arg)
            } else {
                q.r = arg.parse::<f64>().unwrap()
            }
        }

        q
    }

    fn multiply_base(a: char, b: char) -> SignedCoefficient {
        if a == 'r' { return SignedCoefficient { c: 1f64, d: b }; }
        if b == 'r' { return SignedCoefficient { c: 1f64, d: a }; }
        if a == b { return SignedCoefficient { c: -1f64, d: 'r' }; }
        let diff = u32::from(a) as i32 - u32::from(b) as i32;

        SignedCoefficient {
            c: (if diff > 0 { -1f64 } else { 1f64 }) * (if (diff + 2i32) % 2 == 0 { -1f64 } else { 1f64 }),
            d: vec!['i', 'j', 'k'].iter().find(|&&e| e != a && e != b).unwrap().to_owned(),
        }
    }

    fn get(&self, key: char) -> f64 {
        match key {
            'r' => self.r,
            'i' => self.i,
            'j' => self.j,
            'k' => self.k,
            _ => 0f64
        }
    }

    fn set(&mut self, key: char, value: f64) -> &Quaternion {
        match key {
            'r' => self.r = value,
            'i' => self.i = value,
            'j' => self.j = value,
            'k' => self.k = value,
            _ => ()
        }

        self
    }

    fn multiply(&self, a: Quaternion) -> Quaternion {
        let mut res = Quaternion::new(vec![]);
        for p in vec!['r', 'i', 'j', 'k'] {
            for n in vec!['r', 'i', 'j', 'k'] {
                let SignedCoefficient { c, d } = Quaternion::multiply_base(p, n);
                res.set(d, res.get(d) + c * self.get(p) * a.get(n));
            }
        }
        res
    }
}

fn main() {
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let expr = input_line.trim_matches('\n').to_string();

    for res in Quaternion::parse(&expr) {
        println!("{:?}", res);
    }
}