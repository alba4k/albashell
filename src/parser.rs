use crate::WARNING;
use crate::parser::Condition::{Failure, Success};

pub enum Condition {
    Success,
    Failure,
}
impl Condition {
    pub fn allows(&self, retcode: i32) -> bool {
        match self {
            Condition::Success => retcode == 0,
            Condition::Failure => retcode != 0,
        }
    }
}

pub struct Cmd {
    pub executable: String,
    pub args: Vec<String>,
    pub _background: bool,
    pub cond: Option<Condition>,
}
impl Cmd {
    pub fn should_run(&self, retcode: i32) -> bool {
        self.cond.as_ref().is_none_or(|c| c.allows(retcode))
    }
}

pub fn parse(input: &str) -> Vec<Cmd> {
    let mut split: (&str, &str) = input.trim().split_once(' ').unwrap_or((input.trim(), ""));
    if split.0.is_empty() {
        return Vec::new();
    }

    let mut in_str: bool = false;
    let mut escape: bool = false;

    let mut curr: String = String::new();
    let mut cmds: Vec<Cmd> = vec![Cmd {
        executable: split.0.to_string(),
        args: Vec::new(),
        _background: false,
        cond: None,
    }];

    if split.1.is_empty() {
        return cmds;
    }

    let mut iter = split.1.chars();
    while let Some(ch) = iter.next() {
        if escape {
            escape = false;
            curr.push(match ch {
                'n' => '\n',
                'e' => '\x1b',
                c => c,
            });
            continue;
        }

        match ch {
            '"' => in_str.toggle(),
            _ if in_str => curr.push(ch),

            '$' => {
                let Some(rest) = iter.as_str().strip_prefix('{') else {
                    curr.push(ch);
                    continue;
                };
                iter = rest.chars();

                let mut key: String = String::new();
                while let Some(ch) = iter.next()
                    && ch != '}'
                {
                    key.push(ch);
                }

                curr.push_str(std::env::var(&key).unwrap_or(String::new()).as_str());
            }

            '\\' => escape = true,
            ' ' if !curr.is_empty() => {
                cmds.last_mut().unwrap().args.push(curr);
                curr = String::new();
            }
            ' ' => {}
            '&' | '|' | ';' => {
                if ch != ';' {
                    let Some(rest) = iter.as_str().strip_prefix(ch) else {
                        curr.push(ch);
                        continue;
                    };
                    iter = rest.chars();
                }

                cmds.last_mut().unwrap().args.push(curr);
                split = iter
                    .as_str()
                    .trim_start()
                    .split_once(' ')
                    .unwrap_or((input, ""));
                cmds.push(Cmd {
                    executable: split.0.to_string(),
                    args: Vec::new(),
                    _background: false,
                    cond: if ch == '&' {
                        Some(Success)
                    } else if ch == '|' {
                        // ch == '|'
                        Some(Failure)
                    } else {
                        None
                    },
                });
                iter = split.1.chars();
                curr = String::new();
            }
            _ => curr.push(ch),
        }
    }
    cmds.last_mut().unwrap().args.push(curr);

    if in_str {
        eprintln!("{WARNING}: Unclosed quotation marks. Inserting one at the end of your input");
    }

    cmds
}
