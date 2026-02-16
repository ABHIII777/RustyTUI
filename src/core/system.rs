use sysinfo::{System, Disks, Networks};

pub struct SystemStats {
    pub cpu: f32,
    pub memory_used: u64,
    pub memory_total: u64,
    pub uptime: u64,
    pub disks: Vec<(String, u64, u64)>,
    pub network_rx: u64,
    pub network_tx: u64,
    pub per_core: Vec<f32>,
}

pub struct SystemMonitor {
    system: System,
    disks: Disks,
    networks: Networks,
}

impl SystemMonitor {
    pub fn new() -> Self {
        let mut system = System::new_all();
        system.refresh_all();
        let disks = Disks::new_with_refreshed_list();
        let networks = Networks::new_with_refreshed_list();

        Self {
            system,
            disks,
            networks,
        }
    }

    pub fn update(&mut self) -> SystemStats {
        self.system.refresh_cpu_all();
        self.system.refresh_memory();
        self.disks.refresh(true);
        self.networks.refresh(true);

        let cpu = self.system.global_cpu_usage();
        let per_core: Vec<f32> = self.system.cpus().iter().map(|cpu| cpu.cpu_usage()).collect();

        let memory_used = self.system.used_memory();
        let memory_total = self.system.total_memory();

        let uptime = System::uptime();

        let disks = self.disks.list()
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
        for (_name, data) in self.networks.iter() {
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
            per_core,
        }
    }

}
