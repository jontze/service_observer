use rayon::prelude::*;
use serde::Deserialize;
use serde::Serialize;
use std::fmt::Debug;

use crate::error::JournalError;

pub trait Parser {
    type ParserTarget;

    fn parse<S>(str_payload: S) -> Result<Self::ParserTarget, JournalError>
    where
        S: AsRef<str> + Debug;
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum OutputStatus {
    SUCCESSFUL,
    WARNINGS,
    FAILED,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct LogOutput {
    pub amount: usize,
    pub logs: Vec<LogEntry>,
    pub log_failures: Vec<LogEntryFailure>,
    pub failed_amount: usize,
    pub status: OutputStatus,
}

impl Parser for LogOutput {
    type ParserTarget = Self;

    fn parse<S>(str_payload: S) -> Result<Self::ParserTarget, JournalError>
    where
        S: AsRef<str> + Debug,
    {
        let mut output: LogOutput = str_payload
            .as_ref()
            .par_split('\n')
            .fold(
                || Self {
                    amount: 0,
                    logs: Vec::new(),
                    log_failures: Vec::new(),
                    failed_amount: 0,
                    status: OutputStatus::SUCCESSFUL,
                },
                |mut collected_output, line| {
                    if !line.is_empty() {
                        match LogEntry::parse(line) {
                            Ok(parsed_log) => {
                                collected_output.logs.push(parsed_log);
                                collected_output.amount += 1;
                            }
                            Err(err) => {
                                collected_output.log_failures.push(LogEntryFailure {
                                    log_string: line.to_owned(),
                                    error_string: err.to_string(),
                                    message: "Unable to parse Logentry".to_owned(),
                                });
                                collected_output.failed_amount += 1;
                                collected_output.status = OutputStatus::WARNINGS;
                            }
                        };
                        collected_output
                    } else {
                        collected_output
                    }
                },
            )
            .reduce(
                || Self {
                    amount: 0,
                    logs: Vec::new(),
                    log_failures: Vec::new(),
                    failed_amount: 0,
                    status: OutputStatus::SUCCESSFUL,
                },
                |mut output_1, output_2| {
                    output_1.amount += output_2.amount;
                    output_1.failed_amount += output_2.failed_amount;
                    output_1.logs.extend(output_2.logs);
                    output_1.log_failures.extend(output_2.log_failures);
                    output_1
                },
            );
        if output.amount == 0 && output.failed_amount > 0 {
            output.status = OutputStatus::FAILED;
        }
        Ok(output)
    }
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
pub struct LogEntry {
    #[serde(rename(deserialize = "TID"))]
    pub tid: Option<String>,
    #[serde(rename(deserialize = "_HOSTNAME"))]
    pub hostname: String,
    #[serde(rename(deserialize = "JOB_RESULT"))]
    pub job_result: Option<String>,
    #[serde(rename(deserialize = "SYSLOG_IDENTIFIER"))]
    pub syslog_identifier: String,
    #[serde(rename(deserialize = "_PID"))]
    pub pid: String,
    #[serde(rename(deserialize = "_GID"))]
    pub gid: String,
    #[serde(rename(deserialize = "_UID"))]
    pub uid: String,
    #[serde(rename(deserialize = "_SYSTEMD_INVOCATION_ID"))]
    pub systemd_invocation_id: Option<String>,
    #[serde(rename(deserialize = "PRIORITY"))]
    pub priority: String,
    #[serde(rename(deserialize = "MESSAGE"))]
    pub message: String,
    #[serde(rename(deserialize = "MESSAGE_ID"))]
    pub message_id: Option<String>,
    #[serde(rename(deserialize = "_SOURCE_REALTIME_TIMESTAMP"))]
    pub source_realtime_timestamp: String,
    #[serde(rename(deserialize = "__MONOTONIC_TIMESTAMP"))]
    pub monotonic_timestamp: String,
    #[serde(rename(deserialize = "SYSLOG_TIMESTAMP"))]
    pub syslog_timestamp: Option<String>,
    #[serde(rename(deserialize = "__REALTIME_TIMESTAMP"))]
    pub realtime_timestamp: String,
    #[serde(rename(deserialize = "_CMDLINE"))]
    pub cmdline: Option<String>,
    #[serde(rename(deserialize = "_SYSTEMD_CGROUP"))]
    pub systemd_cgroup: String,
    #[serde(rename(deserialize = "_SYSTEMD_SLICE"))]
    pub systemd_slice: String,
    #[serde(rename(deserialize = "SYSLOG_FACILITY"))]
    pub syslog_facility: String,
    #[serde(rename(deserialize = "_BOOT_ID"))]
    pub boot_id: String,
    #[serde(rename(deserialize = "_CAP_EFFECTIVE"))]
    pub cap_effective: String,
    #[serde(rename(deserialize = "__CURSOR"))]
    pub cursor: String,
    #[serde(rename(deserialize = "_MACHINE_ID"))]
    pub machine_id: String,
    #[serde(rename(deserialize = "_RUNTIME_SCOPE"))]
    pub runtime_scope: Option<String>,
    #[serde(rename(deserialize = "SYSLOG_PID"))]
    pub syslog_pid: Option<String>,
    #[serde(rename(deserialize = "_TRANSPORT"))]
    pub transport: String,
    #[serde(rename(deserialize = "_SYSTEMD_UNIT"))]
    pub systemd_unit: String,
    #[serde(rename(deserialize = "_EXE"))]
    pub exe: Option<String>,
    #[serde(rename(deserialize = "_COMM"))]
    pub comm: String,
    #[serde(rename(deserialize = "CPU_USAGE_NSEC"))]
    pub cpu_usage_nsec: Option<String>,
}

impl Parser for LogEntry {
    type ParserTarget = Self;

    fn parse<S>(payload: S) -> Result<Self::ParserTarget, JournalError>
    where
        S: AsRef<str> + Debug,
    {
        let res = serde_json::from_str::<LogEntry>(payload.as_ref().trim())?;
        Ok(res)
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct LogEntryFailure {
    pub log_string: String,
    pub message: String,
    pub error_string: String,
}
