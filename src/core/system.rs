use sysinfo::{System, SystemExt, CpuExt, DiskExt, NetworkExt};

pub struct SystemStats {
    pub cpu: f32,
    pub memory_used: u64,
    pub memory_total: u64,
    pub uptime: u64,
    pub disks: Vec<(String, u64, u64)>,
    pub network_rx: u64,
    pub network_tx: u64,
}

pub struct SystemMonitor {
    system: System,
}

impl SystemMonitor {
    pub fn new() -> Self {
        let mut system = System::new_all();
        system.refresh_all();

        Self {system}
    }

    pub fn update(&mut self) -> SystemStats {
        self.system.refresh_all();

        self.system.refresh_cpu();
        self.system.refresh_memory();

        let cpu = self.system.global_cpu_info().cpu_usage();
        let memory_used = self.system.used_memory();
        let memory_total = self.system.total_memory();
        let uptime = self.system.uptime();
        let per_core = self.system.cpus().iter().map(|c| c.cpu_usage()).collect();

        let disks = self.system.disks()
            .iter()
            .map(|d| {
                (
                    d.name().to_string_lossy().to_string(),
                    d.available_space(),
                    d.total_space(),
                )
            })
            .collect();

        let mut rx = 0;
        let mut tx = 0;
        for (_, data) in self.system.networks() {
            rx += data.received();
            tx += data.transmitted();
        }

        SystemStats {
            cpu,
            memory_used,
            memory_total,
            uptime,
            disks,
            network_rx: rx,
            network_tx: tx,
        }
    }
}
