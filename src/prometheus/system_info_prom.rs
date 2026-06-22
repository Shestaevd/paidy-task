use sysinfo::System;
use tokio::time::Duration;

use crate::prometheus::prom::METRICS;

pub async fn collect_metrics_thread() {
    let system: System = System::new_all();

    tokio::spawn(async move { collect_metrics(system).await });
}

pub async fn collect_metrics(mut system: System) {
    let mut interval = tokio::time::interval(Duration::from_secs(1));

    loop {
        interval.tick().await;
        let metrics = METRICS.lock().await;

        system.refresh_cpu_usage();
        system.refresh_memory();

        for (i, cpu) in system.cpus().iter().enumerate() {
            metrics
                .cpu_load_gauge
                .with_label_values(&[&format!("core_{i}")])
                .set(cpu.cpu_usage() as f64);
        }

        metrics.ram_used_gauge.set(system.used_memory() as f64);
        metrics
            .ram_available_gauge
            .set(system.available_memory() as f64);

        metrics
            .cpu_load_total_gauge
            .set(system.global_cpu_usage() as f64);
    }
}
