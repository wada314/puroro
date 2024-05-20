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
        (true, false, false) => Case::CamelCase,
        (false, true, false) => Case::LowerSnakeCase,
        (false, false, true) => Case::UpperSnakeCase,
        _ => Case::Unknown,
    }
}

pub fn split_into_words(s: &str, case: Option<Case>) -> impl Iterator<Item = &str> {
    let case = case.unwrap_or_else(|| guess_case(s));
    // WTF
    split_camel_case_into_words(if matches!(case, Case::CamelCase) {
        s
    } else {
        ""
    })
    .chain(
        split_snake_case_into_words(
            if matches!(case, Case::UpperSnakeCase | Case::LowerSnakeCase) {
                s
            } else {
                ""
            },
        )
        .chain(split_unkown_into_words(if matches!(case, Case::Unknown) {
            s
        } else {
            ""
        })),
    )
}

fn split_unkown_into_words(s: &str) -> impl Iterator<Item = &str> {
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
            pos += 1;
            let c = s.chars().nth(pos).unwrap();
            let last_state = state;
            state = if c.is_lowercase() {
                State::Lower
            } else if c.is_uppercase() {
                State::Upper
            } else if c == '_' {
                State::Underscore
            } else {
                State::Neutral
            };
            match (last_state, state) {
                (Lower | Neutral, Upper) => {
                    return Some(&s[start_pos..pos]);
                }
                (Underscore, _) => {
                    start_pos = pos;
                }
                (_, Underscore) => {
                    return Some(&s[start_pos..pos]);
                }
                _ => {}
            }
        }
        Some(&s[start_pos..pos])
    })
}

fn split_camel_case_into_words(s: &str) -> impl Iterator<Item = &str> {
    let mut pos = 0;
    ::std::iter::from_fn(move || {
        let start_pos = pos;
        while pos < s.len() {
            pos += 1;
            let c = s.chars().nth(pos).unwrap();
            if c.is_uppercase() {
                return Some(&s[start_pos..pos]);
            }
        }
        Some(&s[start_pos..pos])
    })
}

fn split_snake_case_into_words(s: &str) -> impl Iterator<Item = &str> {
    s.split('_')
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
    fn test_split_camel_case_into_words() {
        let s = "FooBarDam";
        let words: Vec<_> = split_camel_case_into_words(s).collect();
        assert_eq!(words, vec!["foo", "Bar", "Dam"]);

        let s = "foo";
        let words: Vec<_> = split_camel_case_into_words(s).collect();
        assert_eq!(words, vec!["foo"]);
    }

    #[test]
    fn test_split_snake_case_into_words() {
        let s = "foo_bar_dam";
        let words: Vec<_> = split_snake_case_into_words(s).collect();
        assert_eq!(words, vec!["foo", "bar", "dam"]);

        let s = "foo";
        let words: Vec<_> = split_snake_case_into_words(s).collect();
        assert_eq!(words, vec!["foo"]);
    }

    #[test]
    fn test_split_unknown_case_into_words() {
        fn do_test(s: &str, expected: Vec<&str>) {
            let words: Vec<_> = split_unkown_into_words(s).collect();
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
