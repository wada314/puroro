// Copyright 2021 Google LLC
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//      http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Case {
    Unknown,
    CamelCase,
    UpperSnakeCase,
    LowerSnakeCase,
}

pub fn guess_case(s: &str) -> Case {
    let mut has_upper = false;
    let mut has_lower = false;
    let mut has_underscore = false;
    for c in s.chars() {
        if c.is_uppercase() {
            has_upper = true;
        } else if c.is_lowercase() {
            has_lower = true;
        } else if c == '_' {
            has_underscore = true;
        }
    }
    match (has_upper, has_lower, has_underscore) {
        (true, true, false) => Case::CamelCase,
        (false, true, true) => Case::LowerSnakeCase,
        (true, false, true) => Case::UpperSnakeCase,
        _ => Case::Unknown,
    }
}

pub fn split_into_words(s: &str) -> impl Iterator<Item = &str> {
    let mut pos = 0;
    #[derive(Debug, PartialEq, Eq, Clone, Copy)]
    enum State {
        Lower,
        Upper,
        Underscore,
        Neutral,
    }
    let mut state = State::Neutral;
    ::std::iter::from_fn(move || {
        use State::*;
        let mut start_pos = pos;
        while pos < s.len() {
            let c = s.as_bytes()[pos];
            let last_state = state;
            state = if c.is_ascii_lowercase() {
                State::Lower
            } else if c.is_ascii_uppercase() {
                State::Upper
            } else if c == b'_' {
                State::Underscore
            } else {
                State::Neutral
            };
            match (last_state, state) {
                (Lower | Neutral, Upper) if start_pos != pos => {
                    return Some(&s[start_pos..pos]);
                }
                (Underscore, _) => {
                    start_pos = pos;
                }
                (_, Underscore) if start_pos != pos => {
                    return Some(&s[start_pos..pos]);
                }
                _ => {}
            }
            pos += 1;
        }
        (start_pos != pos).then_some(&s[start_pos..pos])
    })
}

pub fn convert_into_case(s: &str, case: Case) -> String {
    let words = split_into_words(s).collect::<Vec<_>>();
    match case {
        Case::CamelCase => words.into_iter().map(|w| w).collect::<String>(),
        Case::LowerSnakeCase => words.join("_").to_lowercase(),
        Case::UpperSnakeCase => words.join("_").to_uppercase(),
        Case::Unknown => s.to_string(),
    }
}

pub fn capitalize(s: &str) -> String {
    if let Some((first, rem)) = s.split_at_checked(1) {
        first.to_uppercase() + rem
    } else {
        s.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_guess_case() {
        assert_eq!(guess_case("FooBarDam"), Case::CamelCase);
        assert_eq!(guess_case("foo_bar_dam"), Case::LowerSnakeCase);
        assert_eq!(guess_case("FOO_BAR_DAM"), Case::UpperSnakeCase);
        assert_eq!(guess_case("FooBar_"), Case::Unknown);
        assert_eq!(guess_case("Foo_Bar"), Case::Unknown);
        assert_eq!(guess_case("_FooBar"), Case::Unknown);
    }

    #[test]
    fn test_split_into_words() {
        fn do_test(s: &str, expected: Vec<&str>) {
            let words: Vec<_> = split_into_words(s).collect();
            assert_eq!(words, expected);
        }
        do_test("FooBarDam", vec!["Foo", "Bar", "Dam"]);
        do_test("foo_bar_dam", vec!["foo", "bar", "dam"]);
        do_test("FOO_BAR_DAM", vec!["FOO", "BAR", "DAM"]);
        do_test("__FooBar", vec!["Foo", "Bar"]);
        do_test("Foo__BarDam", vec!["Foo", "Bar", "Dam"]);
        do_test("Foo123Bar123Dam123", vec!["Foo123", "Bar123", "Dam123"]);
        do_test("Foo123bar123Dam123", vec!["Foo123bar123", "Dam123"]);
    }
}
