


pub struct Parser<'a, T> {
    parse: Box<dyn Fn(&'a str) -> (Result<T, String>, &'a str) + 'a>
}


fn parse_whitespace<'a>(s: &'a str) -> (Result<(), String>, &'a str) {
    let err_tuple = (Err(format!("Expected whitespace in {s}")), s);
    let new_s = s.trim_start();
    if new_s.len() < s.len() {
        (Ok(()), new_s)
    } else {
        err_tuple
    }
}


pub fn whitespacep<'a>() -> Parser<'a, ()> {
    Parser{parse: Box::new(parse_whitespace)}
}

pub fn stringp<'a>(s_match: &'a str) -> Parser<'a, String> {
    let parse_string = move |s: &'a str| {
        let err_tuple = (Err(format!("Expected {} in {s}", String::from(s_match))), s);
        if s.starts_with(s_match) {
            (Ok(String::from(s_match)), &s[s_match.len()..])
        } else {
            err_tuple
        }
    };
    Parser{parse: Box::new(parse_string)}
}

pub fn charp<'a>(c: char) -> Parser<'a, char> {
    let parse_char  = move |s: &'a str| { 
        let err_tuple = (Err(format!("Expected {} in {s}", c)), s);
        if let Some(first_char ) = s.chars().next() {
            if c == first_char {
                (Ok(c), &s[1..])
            } else {
                err_tuple
            }
        } else {
            err_tuple
        }
    };
    Parser{parse: Box::new(parse_char)}
}

pub fn seq<'a, T: 'a>(ps: Vec<Parser<'a, T>>) -> Parser<'a, Vec<T>> {
    let parse_seq = move |s: &'a str|  {
        let mut ts = vec![];
        let mut rest =  s;
        let mut err = None;
        for p in ps.iter() {
            let result = (p.parse)(rest);
            match result {
                (Ok(t), s_remain) => {
                    rest = s_remain;
                    ts.push(t);
                }
                (Err(s_err), _) => {
                    err = Some((Err(s_err), s));
                    break
                }
            }
        };
        match err {
            Some(e) => e,
            None => (Ok(ts), rest)
        }
    };
    Parser{parse: Box::new(parse_seq)}
}

pub fn or<'a, T: 'a>(ps: Vec<Parser<'a, T>>) -> Parser<'a, T> {
    let parse_or = move |s: &'a str| {
        let mut last_res = None;
        for p in ps.iter() {
            let result = (p.parse)(s);
            match result {
                (Ok(_), _) => {
                    last_res = Some(result);
                    break
                }
                (Err(_), _) => {
                    last_res = Some(result)
                }
            }
        }
        match last_res {
            Some(r) => r,
            None => (Err(String::from("No parsers given to or.")), s),
        }
    };
    Parser{parse: Box::new(parse_or)}
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn test_charp_pass() {
        let cp1 = charp('c');
        let result = (cp1.parse)("hello");
        match result {
            (Err(_), s) => assert_eq!(s, "hello"),
            _ => assert!(false),
        }
    }

    #[test]
    fn test_charp_fail1() {
        let cp1 = charp('c');
        let result = (cp1.parse)("cello");
        assert_eq!(result, (Ok('c'), "ello"));
    }

    #[test]
    fn test_charp_fail_emp() {
        let cp1 = charp('c');
        let result = (cp1.parse)("");
        match result {
            (Err(_), s) => assert_eq!(s, ""),
            _ => assert!(false),
        }
    }

    #[test]
    fn test_stringp_pass() {
        let sp1 = stringp("hell");
        let result = (sp1.parse)("hello");
        assert_eq!(result, (Ok(String::from("hell")), "o"))
    }

    #[test]
    fn test_stringp_fail() {
        let sp1 = stringp("blu");
        let result = (sp1.parse)("hello");
        match result {
            (Err(_), s) => assert_eq!(s, "hello"),
            _ => assert!(false)
        }
    }

    #[test]
    fn test_whitespacep_pass() {
        let wp1 = whitespacep();
        let result = (wp1.parse)("\nFFF");
        assert_eq!(result, (Ok(()), "FFF"));
    }

    #[test]
    fn test_whitespacep_fail() {
        let wp1 = whitespacep();
        let result = (wp1.parse)("FFF");
        match result {
            (Err(_), s) => assert_eq!(s, "FFF"),
            _ => assert!(false),
        }
    }

    #[test]
    fn test_seq_pass() {
        let ps = vec![charp('h'), charp('e'), charp('l')];
        let combined = seq(ps);
        let result = (combined.parse)("hello");
        assert_eq!(result, (Ok(vec!['h', 'e', 'l']), "lo"));
    }

    #[test]
    fn test_seq_fail() {
        let ps = vec![charp('h'), charp('e'), charp('r')];
        let combined = seq(ps);
        let result = (combined.parse)("hello");
        match result {
            (Err(_), rest) => assert_eq!(rest, "hello"),
            _ => assert!(false),
        }
    }

    #[test]
    fn test_or_pass() {
        let ps = vec![charp('.'), charp('h'), charp('l')];
        let best_one = or(ps);
        let result = (best_one.parse)("hello");
        assert_eq!(result, (Ok('h'), "ello"));
    }

    #[test]
    fn test_or_fail() {
        let ps = vec![charp('.'), charp('b'), charp('l')];
        let best_one = or(ps);
        let result = (best_one.parse)("hello");
        match result {
            (Err(_), s) => assert_eq!(s, "hello"),
            _ => assert!(false),
        }
    }

    #[test]
    fn test_or_empty() {
        let ps : Vec<Parser<char>> = vec![];
        let best_one = or(ps);
        let result = (best_one.parse)("hello");
        match result {
            (Err(_), s) => assert_eq!(s, "hello"),
            _ => assert!(false),
        }
    }



}



