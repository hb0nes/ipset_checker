use std::process::Command;
use std::str::from_utf8;
use anyhow::{Context, format_err};
use once_cell::sync::Lazy;
use regex::Regex;
use crate::Config;


static IP_VALIDATE_RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"^\d{1,3}\.\d{1,3}\.\d{1,3}\.\d{1,3}$").unwrap());

pub fn ipsets_to_reasons(ipsets: Vec<&str>) -> Vec<String> {
    let mut reasons: Vec<String> = vec![];
    for ipset in ipsets {
        if let Some(reason) = Config::global().ipset_reason.get(ipset) {
            reasons.push(reason.clone())
        } else {
            reasons.push(format!("IP was found in ipset: {}. Could not find reason in configuration.", ipset))
        }
    }
    reasons
}

pub fn check_ip(ip: &str) -> anyhow::Result<String> {
    if !IP_VALIDATE_RE.is_match(ip) {
        return Err(format_err!("{ip} is not a valid IP"));
    }
    let output = Command::new("ipset").args(["list", "-o", "save"]).output().with_context(||"running ipset list -o save")?;
    let output_str = from_utf8(&output.stdout).with_context(||"converting ipset output to utf8")?;
    let output_filtered: Vec<&str> = output_str.lines().filter(|line| line.contains(ip)).collect();
    let ipsets_filtered: Vec<&str> = output_filtered.iter().map(|l| {
        l.split_whitespace().collect::<Vec<_>>()[1]
    }).collect();
    let reasons = if !ipsets_filtered.is_empty() {
        ipsets_to_reasons(ipsets_filtered)
    } else { vec![String::from("Not found in current ipsets, so most likely not banned.")] };
    Ok(format!("{}", reasons.join("\n")))
}
