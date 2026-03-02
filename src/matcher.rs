use crate::store::Entry;

pub enum MatchResult<'a> {
    None,
    One(&'a Entry),
    Ambiguous(Vec<&'a Entry>),
}

pub fn find<'a>(query: &str, entries: &'a [Entry]) -> MatchResult<'a> {
    let exact: Vec<_> = entries.iter().filter(|e| e.name == query).collect();
    if exact.len() == 1 {
        return MatchResult::One(exact[0]);
    }

    let prefix: Vec<_> = entries
        .iter()
        .filter(|e| e.name.starts_with(query))
        .collect();
    match prefix.len() {
        1 => return MatchResult::One(prefix[0]),
        n if n > 1 => return MatchResult::Ambiguous(prefix),
        _ => {}
    }

    let sub: Vec<_> = entries.iter().filter(|e| e.name.contains(query)).collect();
    match sub.len() {
        1 => MatchResult::One(sub[0]),
        n if n > 1 => MatchResult::Ambiguous(sub),
        _ => MatchResult::None,
    }
}
