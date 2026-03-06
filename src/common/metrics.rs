// Copyright (c) 2026 Silas Tang | Axiom 042
// SPDX-License-Identifier: MIT

#[derive(Debug, Clone, Default)]
pub struct MetricPoint {
    pub pid: i32,
    pub vmrss_kb: u64,
    pub voluntary_ctxt: u64,
    pub nonvoluntary_ctxt: u64,
}