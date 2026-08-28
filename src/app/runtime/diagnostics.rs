use crate::contracts::daemon::DoctorCheck;

pub(crate) const TOAST_MS: u128 = 5000;

pub(crate) fn doctor_summary(checks: &[DoctorCheck]) -> String {
    let mut pass = 0usize;
    let mut issues: Vec<String> = Vec::new();
    for check in checks {
        match check.status.as_str() {
            "pass" => pass += 1,
            "warn" | "fail" => issues.push(check.check.clone()),
            _ => {}
        }
    }
    if issues.is_empty() {
        crate::i18n::status_diagnostics_passed(pass)
    } else {
        let shown: Vec<&str> = issues.iter().take(3).map(String::as_str).collect();
        let more = issues.len() - shown.len();
        let suffix = if more > 0 {
            format!(" {}", crate::i18n::tr_args!("status-diagnostics-more", count => more))
        } else {
            String::new()
        };
        crate::i18n::status_diagnostics_issues(issues.len(), &shown.join("; "), &suffix)
    }
}

pub(crate) fn apply_error_message(kind: &str, detail: &str) -> String {
    let heading = match kind {
        "file_missing" => crate::i18n::tr("status-apply-file-missing"),
        "renderer_unavailable" | "renderer_spawn_failed" => {
            crate::i18n::tr("status-apply-renderer-failed")
        }
        "decode_failed" => crate::i18n::tr("status-apply-decode-failed"),
        "no_outputs" => crate::i18n::tr("status-apply-no-outputs"),
        "bad_request" => crate::i18n::tr("status-apply-invalid-request"),
        _ => crate::i18n::tr("status-apply-failed"),
    };
    let detail = detail.trim();
    if detail.is_empty() || kind == "bad_request" {
        heading.to_string()
    } else {
        crate::i18n::tr_args!("status-apply-detail", heading => heading, detail => detail)
    }
}
