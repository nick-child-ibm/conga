// SPDX-License-Identifier: Apache-2.0
// Copyright 2021 IBM Corp.
use crate::collector::*;

pub fn get_cores(col: &mut Collector) -> Result<CollectorValue, CollectorErr> {
    let captures = col.parse_from_command("lscpu", r"\s+Core\(s\) per socket:\s+(\d+)")?;
    Ok(CollectorValue::Integer(captures[1].parse::<i64>().unwrap()))
}
