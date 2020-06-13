use crate::{RoutePath, RoutePathPart};
use nom::{
    branch::alt,
    bytes::complete::{tag, take_while1},
    combinator::opt,
    multi::separated_list,
    IResult,
};

pub(crate) fn parse_route(input: &str) -> IResult<&str, RoutePath> {
    let (input, _) = opt(tag("/"))(input)?;
    let (input, parts) = separated_list(tag("/"), alt((parse_variable, parse_constant)))(input)?;
    let (input, _) = opt(tag("/"))(input)?;
    Ok((input, RoutePath::from_raw_parts(parts)))
}

pub(crate) fn parse_constant(input: &str) -> IResult<&str, RoutePathPart> {
    let (input, name) = take_while1(char::is_alphabetic)(input)?;
    Ok((input, RoutePathPart::Constant(name.to_owned())))
}

pub(crate) fn parse_variable(input: &str) -> IResult<&str, RoutePathPart> {
    let (input, _) = tag(":")(input)?;
    let (input, name) = take_while1(char::is_alphabetic)(input)?;
    Ok((input, RoutePathPart::Variable(name.to_owned())))
}
