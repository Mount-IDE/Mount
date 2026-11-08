use std::collections::HashMap;
use macroses::log;
use mount_core::domain::Val;
use mount_core::domain::Var;

use crate::traits::{FormatError, TFormatService};

pub struct FormatService;

impl TFormatService for FormatService {
    fn format_string(
        &self,
        string: String,
        vars: &Vec<Var>,
        results: &HashMap<String, Val>,
    ) -> Result<String, FormatError> {
        //   log!("RESULTS {results:?}");
        #[derive(Clone, Copy, Debug)]
        pub enum Type {
            Braces,
            Short,
        }

        const VAR: char = '@';
        const RES: char = '#';
        if !string.contains(VAR) && !string.contains(RES) {
            log!("not vars and results");
            return Ok(string);
        }

        let mut str = String::with_capacity(string.capacity());

        let mut iter = string.chars().peekable();

        let mut captured = false;
        let mut temp = String::new();

        let mut typ: Option<Type> = None;

        let cb_var = |typ: Type,
                      temp: &mut String,
                      str: &mut String,
                      vars: &Vec<Var>|
                      -> Result<(), FormatError> {
            let slice = if let Type::Braces = typ {
                temp[2..temp.len() - 1].to_string().clone()
            } else {
                temp[1..].to_string().clone()
            };
            // log!("VAR {slice} {temp} {typ:?}");
            let found = vars.iter().find(|e| e.name == slice);
            if let Some(found) = found {
                str.push_str(found.value.to_str().as_str());
                temp.clear()
            } else {
                //    log!("NEEDED VAR NOT FOUND {typ:?}\n {temp:?}\n'{str}'\n{vars:?}");
                return Err(FormatError::NeededVarNotFound(slice));
            };
            Ok(())
        };
        let cb_res = |typ: Type,
                      temp: &mut String,
                      str: &mut String,
                      results: &HashMap<String, Val>|
                      -> Result<(), FormatError> {
            let slice = if let Type::Braces = typ {
                temp[2..temp.len() - 1].to_string().clone()
            } else {
                temp[1..].to_string().clone()
            };
            //log!("RES {slice} {temp} {typ:?}");
            let found = results.iter().find(|e| *e.0 == slice);
            if let Some(found) = found {
                str.push_str(found.1.to_str().as_str());
                temp.clear()
            } else {
                // log!("NEEDED RES NOT FOUND {typ:?}\n {temp:?}\n'{str}'\n{results:?}\n");
                return Err(FormatError::NeededResNotFound(slice));
            };
            Ok(())
        };

        while let Some(ch) = iter.next() {
            if ch == VAR && !captured {
                captured = true;
                temp.clear()
            }
            if ch == RES && !captured {
                captured = true;
                temp.clear()
            }

            if captured {
                temp.push(ch)
            } else {
                str.push(ch)
            }

            if captured {
                if ch != VAR && ch != RES {
                    if ch == '{' && typ.is_none() {
                        typ = Some(Type::Braces);
                    //     log!("{{");
                    } else if ch != '{' && typ.is_none() {
                        typ = Some(Type::Short);
                        // log!("not {{")
                    }
                    if ch == '}' {
                        if let Some(Type::Braces) = typ {
                            if temp.starts_with(VAR) {
                                cb_var(Type::Braces, &mut temp, &mut str, &vars)?;
                            } else if temp.starts_with(RES) {
                                cb_res(Type::Braces, &mut temp, &mut str, &results)?;
                            } else {
                                // log!("INVALID DECLARATOR");
                                return Err(FormatError::InvalidDeclarator(
                                    temp.chars().take(0).next().unwrap_or(ch),
                                ));
                            }
                            typ = None;
                            captured = false;
                        }
                    }
                    if ch == ' ' {
                        if let Some(Type::Short) = typ {
                            if temp.starts_with(VAR) {
                                cb_var(Type::Short, &mut temp, &mut str, &vars)?;
                            } else if temp.starts_with(RES) {
                                cb_res(Type::Short, &mut temp, &mut str, &results)?;
                            } else {
                                //  log!("INVALID DECLARATOR");
                                return Err(FormatError::InvalidDeclarator(
                                    temp.chars().take(0).next().unwrap_or(ch),
                                ));
                            }
                            typ = None;
                            captured = false;
                        }
                    }
                }
            }
        }
        if captured {
            if let Some(v) = typ {
                match v {
                    Type::Braces => {
                        if temp.starts_with(VAR) {
                            cb_var(Type::Braces, &mut temp, &mut str, &vars)?;
                        } else if temp.starts_with(RES) {
                            cb_res(Type::Braces, &mut temp, &mut str, &results)?;
                        } else {
                            //log!("INVALID DECLARATOR");
                            return Err(FormatError::InvalidDeclarator(
                                temp.chars().take(0).next().unwrap_or(' '),
                            ));
                        }
                    }
                    Type::Short => {
                        if temp.starts_with(VAR) {
                            cb_var(Type::Short, &mut temp, &mut str, &vars)?;
                        } else if temp.starts_with(RES) {
                            cb_res(Type::Short, &mut temp, &mut str, &results)?;
                        } else {
                            //   log!("INVALID DECLARATOR");
                            return Err(FormatError::InvalidDeclarator(
                                temp.chars().take(0).next().unwrap_or(' '),
                            ));
                        }
                    }
                }
            } else {
                //  log!("INVALID DECLARATOR");
                return Err(FormatError::InvalidDeclarator(' '));
            }
        }

        Ok(str)
    }
}