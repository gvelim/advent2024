use std::{str::FromStr, rc::Rc};
use nom::{
    bytes::complete::tag,
    character::complete::{space0, space1, u64},
    combinator::map, multi::separated_list1,
    sequence::{separated_pair, tuple}, IResult
};

#[derive(Debug)]
pub(crate) struct Equation {
    result: u64,
    coeff: Rc<[u64]>
}

impl Equation {
    pub(crate) fn solver(&self, cop: bool) -> Option<u64> {
        Self::solve(self.result, &self.coeff, cop)
    }

    fn solve(total: u64, coeff: &[u64],cop: bool) -> Option<u64> {
        fn ct(a:u64, b:u64) -> u64 { format!("{a}{b}").parse::<u64>().unwrap() }

        let idx = coeff.len() - 1;

        if idx == 0 { return Some(coeff[idx]) }

        let res_1 = Self::solve(total / coeff[idx], &coeff[..idx],cop).map(|s| s * coeff[idx]);
        let res_2 = if total >= coeff[0] {
            Self::solve(total - coeff[idx], &coeff[..idx],cop).map(|s| s + coeff[idx])
        } else { None };
        let res_3 = if cop && total >= coeff[0] {
            Self::solve((total - coeff[idx])/10u64.pow(coeff[idx].ilog10()+1), &coeff[..idx],cop).map(|s| ct(s, coeff[idx]))
        } else { None };

        match (res_1 == Some(total), res_2 == Some(total), res_3 == Some(total)) {
            (true, _, _) => res_1,
            (_, true, _) => res_2,
            (_, _, true) => res_3,
            _ => None,
        }
    }
}

impl FromStr for Equation {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match parse_equation(s) {
            Ok(e) => Ok(e.1),
            Err(e) => Err(e.to_string()),
        }
    }
}

fn parse_equation(s: &str) -> IResult<&str, Equation> {
    map(
        separated_pair(
        u64,
        tuple(( space0, tag(":") )),
        tuple(( space0, separated_list1(space1,u64) ))
        ),
        |(result, (_, coeff))| Equation { result, coeff: coeff.into() }
    )(s)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn parse_input() {
        assert!("190: 10 19".parse::<Equation>().is_ok());
        assert!("3267: 81 40 27".parse::<Equation>().is_ok());
        assert!("83:17 5".parse::<Equation>().is_ok());
        assert!("83 :17 5".parse::<Equation>().is_ok());
        assert!("83   :    17     5".parse::<Equation>().is_ok());
        assert!("83 : ".parse::<Equation>().is_err());
        assert!("363816188802: 5 601 3 603 2 2 93 6 3 5".parse::<Equation>().is_ok());
    }
}
