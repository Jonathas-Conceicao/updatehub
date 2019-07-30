// Copyright (C) 2019 O.S. Systems Sofware LTDA
//
// SPDX-License-Identifier: Apache-2.0

use failure::format_err;
use std::{
    fs,
    io::{BufRead, BufReader},
    path::{Path, PathBuf},
};

pub(crate) fn target_device_from_ubi_volume_name(volume: &str) -> Result<PathBuf, failure::Error> {
    let re = regex::Regex::new(r"^Volume ID:   (\d) \(on ubi(\d)\)$").unwrap();
    let path = fs::read_dir("/dev")?
        .filter(|entry| entry.is_ok())
        .map(|entry| format!("{:?}", entry.unwrap().path()))
        .find(|path| path.starts_with("ubi"))
        .ok_or_else(|| format_err!("Unable to find coorespoing ubi volume"))?;

    let dev_number = path.replace("ubi", "");

    let output = easy_process::run(&format!("ubinfo -d {} -N {}", dev_number, volume))?;
    let line = output
        .stdout
        .lines()
        .next()
        .ok_or_else(|| format_err!("Unable to read first line of ubinfo"))?;

    let re_match = re
        .captures(line)
        .ok_or_else(|| format_err!("Unable to extract any matches for Volume ID"))?;

    Ok(PathBuf::from(format!(
        "/dev/ubi{}_{}",
        dev_number, &re_match[0]
    )))
}

#[allow(unused_variables)]
pub(crate) fn is_nand(device: &Path) -> Result<bool, failure::Error> {
    unimplemented!("FIXME: impl is_nand")
}

pub(crate) fn target_device_from_mtd_name(name: &str) -> Result<PathBuf, failure::Error> {
    let re = regex::Regex::new(r#"^(mtd\d): (\d+) (\d+) "(.*)"$"#).unwrap();
    let proc = fs::File::open("/proc/mtd")?;

    for line in BufReader::new(proc).lines() {
        let line = line?;
        let re_match = re
            .captures(&line)
            .ok_or_else(|| format_err!("Unable to extract device match"))?;
        if &re_match[4] == name {
            return Ok(PathBuf::from(format!("/dev/{}", &re_match[1])));
        }
    }

    Err(format_err!("Unable to find match for mtd device: {}", name))
}
