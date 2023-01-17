use std::io;
use regex::Regex;

#[cfg(test)]
mod tests {
    use crate::Quaternion;

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
}

#[derive(Debug)]
struct Quaternion {
    r: f64,
    i: f64,
    j: f64,
    k: f64,
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
}

fn main() {
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let expr = input_line.trim_matches('\n').to_string();

    for res in Quaternion::parse(&expr) {
        println!("{:?}", res);
    }
}