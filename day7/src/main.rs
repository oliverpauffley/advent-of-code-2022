use nom::{
    branch::alt,
    bytes::complete::{tag, take, take_till, take_until, take_while, take_while1},
    character::{complete::line_ending, is_alphabetic},
    combinator::{all_consuming, map, map_res, opt},
    sequence::{delimited, preceded, tuple},
    Finish, IResult,
};

use indextree::Arena;

fn main() {
    println!("Hello, world!");
}

#[derive(Debug, PartialEq)]
enum Command {
    Cd(Arena<Directory>),
    Ls,
}

#[derive(Debug, PartialEq)]
struct Directory {
    name: String,
    files: Vec<Files>,
}

#[derive(Debug, PartialEq)]
struct Files(u32);

fn parse_command_name(i: &str) -> IResult<&str, &str> {
    preceded(tag("$ "), take(2_usize))(i)
}

fn parse_command_directory(i: &str) -> IResult<&str, &str> {
    preceded(tag(" "), take(1_usize))(i)
}

fn parse_command(i: &str) -> IResult<&str, Command> {
    map(
        tuple((parse_command_name, opt(parse_command_directory))),
        |(name, directory)| match name {
            let arena = Arena::new();
            "cd" => Command::Cd(Directory {
                name: directory.unwrap().to_owned(),
                files: vec![],
            }),
            "ls" => Command::Ls,
            _ => panic!("got a command we didnt expect"),
        },
    )(i)
}

fn parse_ls_directory(i: &str) -> IResult<&str, &str> {
    preceded(tag("dir "), take(1_usize))(i)
}

fn parse_file_size(i: &str) -> IResult<&str, u32> {
    map(take_while(|a: char| a.is_ascii_digit()), |a: &str| {
        a.parse::<u32>().unwrap()
    })(i)
}

#[cfg(test)]
mod tests {
    use nom::Parser;

    use super::*;

    #[test]
    fn test_commands() {
        let mut parser = parse_command;

        assert_eq!(parser.parse("$ ls").unwrap().1, Command::Ls);
        assert_eq!(
            parser.parse("$ cd a").unwrap().1,
            Command::Cd(Directory {
                name: "a".to_string(),
                files: vec![]
            })
        )
    }

    #[test]
    fn test_files() {
        let mut parser = parse_file_size;

        assert_eq!(parser.parse("2557 g").unwrap().1, 2557);
        assert_eq!(parser.parse("2000 test.txt").unwrap().1, 2000);
    }
}
