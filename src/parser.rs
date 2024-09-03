use crate::*;

use nom::{
    branch::alt,
    bytes::complete::{tag, take_while, take_while1},
    character::complete::{digit1, one_of, space0},
    multi::separated_list,
    sequence::{delimited, pair, preceded},
    IResult,
};
use rayon::prelude::*;

pub fn query(input: &str) -> IResult<&str, Query> {
    alt((add, done, search))(input)
}

pub fn ws(input: &str) -> IResult<&str, char> {
    one_of(" \t")(input)
}

pub fn add(input: &str) -> IResult<&str, Query> {
    match preceded(
        pair(tag("add"), ws),
        pair(description, preceded(space0, tags)),
    )(input)
    {
        Err(e) => Err(e),
        Ok((rest, (d, ts))) => Ok((rest, Query::Add(Description::new(&d), ts))),
    }
}

pub fn is_lowecase_or_dash_or_whitespace(c: char) -> bool {
    c.is_ascii_lowercase() || c.is_whitespace() || c == '-'
}

pub fn is_lowecase_or_dash(c: char) -> bool {
    c.is_ascii_lowercase() || c == '-'
}

pub fn sentence(input: &str) -> IResult<&str, &str> {
    take_while(is_lowecase_or_dash_or_whitespace)(input)
}

pub fn word(input: &str) -> IResult<&str, &str> {
    take_while1(is_lowecase_or_dash)(input)
}

pub fn todo_tag(input: &str) -> IResult<&str, &str> {
    preceded(tag("#"), word)(input)
}

pub fn description(input: &str) -> IResult<&str, String> {
    match delimited(tag("\""), sentence, tag("\""))(input) {
        Err(e) => Err(e),
        Ok((rest, d)) => Ok((rest, d.to_string())),
    }
}

pub fn tags(input: &str) -> IResult<&str, Vec<Tag>> {
    match separated_list(ws, todo_tag)(input) {
        Err(e) => Err(e),
        Ok((rest, ts)) => Ok((rest, ts.iter().map(|w| Tag::new(w)).collect())),
    }
}

pub fn done(input: &str) -> IResult<&str, Query> {
    match preceded(pair(tag("done"), ws), digit1)(input) {
        Err(e) => Err(e),
        Ok((rest, i)) => Ok((rest, Query::Done(Index::new(i.parse::<u64>().unwrap())))),
    }
}

pub fn vec_to_u64(dss: Vec<&str>) -> u64 {
    let ds = dss.concat();
    ds.parse::<u64>().unwrap()
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SearchWordOrTag {
    RawWord(String),
    RawTag(String),
}

pub fn search(input: &str) -> IResult<&str, Query> {
    match preceded(
        pair(tag("search"), ws),
        separated_list(tag(" "), search_word_or_tag),
    )(input)
    {
        Err(e) => Err(e),
        Ok((rest, mash)) => Ok((rest, mash_to_query(mash))),
    }
}

pub fn search_word_or_tag(input: &str) -> IResult<&str, SearchWordOrTag> {
    match alt((todo_tag, word))(input) {
        Err(e) => Err(e),
        Ok((rest, wot)) => {
            if input.starts_with('#') {
                Ok((rest, SearchWordOrTag::RawTag(input[1..].to_string())))
            } else {
                Ok((rest, SearchWordOrTag::RawWord(wot.to_string())))
            }
        }
    }
}

pub fn mash_to_query(mash: Vec<SearchWordOrTag>) -> Query {
    let search_words: Vec<SearchWord> = Vec::with_capacity(mash.len());
    let tags: Vec<Tag> = Vec::with_capacity(mash.len());

    use std::sync::Mutex;

    let search_words_mutex = Mutex::new(search_words);
    let tags_mutex = Mutex::new(tags);

    mash.into_par_iter().for_each(|i| match i {
        SearchWordOrTag::RawWord(w) => {
            search_words_mutex.lock().unwrap().push(SearchWord::new(&w));
        }
        SearchWordOrTag::RawTag(t) => {
            tags_mutex.lock().unwrap().push(Tag::new(&t));
        }
    });

    let search_words = search_words_mutex.into_inner().unwrap();
    let tags = tags_mutex.into_inner().unwrap();

    Query::Search(SearchParams {
        words: search_words,
        tags,
    })
}
