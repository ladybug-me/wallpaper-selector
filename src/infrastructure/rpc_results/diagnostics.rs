use serde_json::Value;

use crate::contracts::daemon::{
    BugReportResult, DiagnosticResult, DoctorCheck, DoctorResult, WeatherResult,
};

use super::common::{DecodeResult, envelope, required_array, string, strings};

pub fn decode_diagnostic(value: &Value) -> DecodeResult<DiagnosticResult> {
    let object = envelope("diag", value)?;
    Ok(DiagnosticResult { banner: string("diag", object, "banner")? })
}

pub fn decode_weather(value: &Value) -> DecodeResult<WeatherResult> {
    let object = envelope("wall.weather", value)?;
    Ok(WeatherResult { weather: strings(required_array("wall.weather", object, "weather")?) })
}

pub fn decode_doctor(value: &Value) -> DecodeResult<DoctorResult> {
    let object = envelope("status.doctor", value)?;
    let checks =
        required_array("status.doctor", object, "checks")?.iter().map(decode_check).collect();
    Ok(DoctorResult { checks })
}

pub fn decode_bug_report(value: &Value) -> DecodeResult<BugReportResult> {
    let object = envelope("status.bug_report", value)?;
    Ok(BugReportResult { path: string("status.bug_report", object, "path")? })
}

fn decode_check(value: &Value) -> DoctorCheck {
    let text = |field| value.get(field).and_then(Value::as_str).unwrap_or_default().to_string();
    DoctorCheck { status: text("status"), check: text("check"), detail: text("detail") }
}
