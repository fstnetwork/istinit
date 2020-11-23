#[derive(Debug, Clone)]
pub struct Config {
    pub enable_process_subreaper: bool,
    pub process: Process,
    pub istio: Option<Istio>,
}

#[derive(Debug, Clone)]
pub struct Istio {
    pub pilot_agent_endpoint: String,
    pub kill_istio: bool,
}

#[derive(Debug, Clone)]
pub struct Process {
    pub command: String,
    pub args: Vec<String>,
}
