//! # Grep - 简化版 grep
//!
//! 支持 -n 行号、-l 文件名、-i 忽略大小写、-v 反向、-x 整行匹配。
//!
//! ## 考点
//! - 文件 I/O、anyhow

use anyhow::{Context, Result};
use std::fs;
use std::path::Path;

#[derive(Debug)]
pub struct Flags {
    n: bool,
    l: bool,
    i: bool,
    v: bool,
    x: bool,
}

impl Flags {
    pub fn new(flags: &[&str]) -> Self {
        Flags {
            n: flags.contains(&"-n"),
            l: flags.contains(&"-l"),
            i: flags.contains(&"-i"),
            v: flags.contains(&"-v"),
            x: flags.contains(&"-x"),
        }
    }
}

pub fn grep(pattern: &str, flags: &Flags, files: &[&str]) -> Result<Vec<String>> {
    let mut out = Vec::new();
    let multi = files.len() > 1;

    for &path in files {
        let content = fs::read_to_string(Path::new(path))
            .with_context(|| format!("File not found: {}", path))?;
        let lines: Vec<&str> = content.lines().collect();

        for (i, line) in lines.iter().enumerate() {
            let (pat, ln) = if flags.i {
                (pattern.to_lowercase(), line.to_lowercase())
            } else {
                (pattern.to_string(), line.to_string())
            };

            let matched = if flags.x {
                ln == pat
            } else {
                ln.contains(&pat)
            };
            let matched = if flags.v { !matched } else { matched };

            if matched {
                if flags.l {
                    out.push(path.to_string());
                    break;
                } else {
                    let mut s = line.to_string();
                    if flags.n {
                        s = format!("{}:{}", i + 1, s);
                    }
                    if multi {
                        s = format!("{}:{}", path, s);
                    }
                    out.push(s);
                }
            }
        }
    }

    Ok(out)
}
