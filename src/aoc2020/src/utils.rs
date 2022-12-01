use std::str::FromStr;

use nom::{
    character::complete::digit1,
    combinator::{map_res, recognize},
    IResult,
};

pub fn num_parser<T>() -> impl FnMut(&str) -> IResult<&str, T>
where
    T: FromStr,
{
    move |input| map_res(recognize(digit1), str::parse)(input)
}
