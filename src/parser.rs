use crate::parser::Condition::{Failure, Success};

pub enum Condition {
    Success,
    Failure,
}

pub struct Cmd {
    pub executable: String,
    pub args: Vec<String>,
    pub _background: bool,
    pub cond: Option<Condition>,
}

pub fn parse(input: &str) -> Vec<Cmd> {
    let mut split: (&str, &str) = input.trim().split_once(' ').unwrap_or((input, ""));

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
    if !split.1.is_empty() {
        let mut iter = split.1.chars();
        while let Some(ch) = iter.next() {
            if escape {
                escape = false;
                match ch {
                    'n' => curr.push('\n'),
                    'e' => curr.push('\x1b'),
                    c => curr.push(c),
                }
                continue;
            }

            match (ch, &in_str) {
                ('"', _) => in_str.toggle(),
                ('\\', false) => escape = true,
                (' ', false) => {
                    if !curr.is_empty() {
                        cmds.last_mut().unwrap().args.push(curr);
                    }
                    curr = String::new();
                }
                ('&' | '|', false) => {
                    if let Some(next) = iter.next() {
                        if next == ch {
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
                                } else {
                                    // ch == '|'
                                    Some(Failure)
                                },
                            });
                            iter = split.1.chars();
                            curr = String::new();
                        } else {
                            curr.push(ch);
                            curr.push(next);
                        }
                    } else {
                        curr.push(ch);
                    }
                }
                (';', false) => {
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
                        cond: None,
                    });
                    iter = split.1.chars();
                    curr = String::new();
                }
                _ => curr.push(ch),
            }
        }
    }
    cmds.last_mut().unwrap().args.push(curr);

    cmds
}
