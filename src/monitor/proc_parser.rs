pub fn fetch_self_status() -> Option<MetricPoint> {
    let content = fs::read_to_string("/proc/self/status").ok()?;
    let mut metric = MetricPoint::default();

    for line in content.lines() {
        
        if let Some((key, value)) = line.split_once(':') {
            match key {
                "VmRSS" => metric.vmrss_kb = parse_num(value),
                "voluntary_ctxt_switches" => metric.voluntary_ctxt = parse_num(value),
                "nonvoluntary_ctxt_switches" => metric.nonvoluntary_ctxt = parse_num(value),
                _ => {}
            }
        }
    }
    Some(metric)
}

fn parse_num(s: &str) -> u64 {
    
    s.trim_start()
     .split_whitespace()
     .next()
     .and_then(|v| v.parse().ok())
     .unwrap_or(0)
}