/// src/tracer_client.rs
use anyhow::Result;
use serde_json::json;
use std::{time::Duration, time::Instant};
use sysinfo::System;

use crate::config_manager::ConfigFile;
use crate::event_recorder::EventRecorder;
use crate::http_client::HttpClient;
use crate::metrics::SystemMetricsCollector;
use crate::process_watcher::ProcessWatcher;

pub struct TracerClient {
    http_client: HttpClient,
    api_key: String,
    system: System,
    service_url: String,
    last_sent: Instant,
    interval: Duration,
    logs: EventRecorder,
    process_watcher: ProcessWatcher,
    metrics_collector: SystemMetricsCollector,
}

impl TracerClient {
    pub fn new(config: ConfigFile) -> Result<TracerClient> {
        let service_url = config.service_url.clone();

        println!("Initializing TracerClient with API Key: {}", config.api_key);
        println!("Service URL: {}", service_url);

        Ok(TracerClient {
            http_client: HttpClient::new(service_url.clone(), config.api_key.clone()),
            api_key: config.api_key,
            system: System::new_all(),
            last_sent: Instant::now(),
            interval: Duration::from_millis(config.process_polling_interval_ms),
            logs: EventRecorder::new(),
            service_url,
            process_watcher: ProcessWatcher::new(config.targets),
            metrics_collector: SystemMetricsCollector::new(),
        })
    }

    pub async fn submit_batched_data(&mut self) -> Result<()> {
        if Instant::now() - self.last_sent >= self.interval {
            self.metrics_collector
                .collect_metrics(&mut self.system, &mut self.logs)?;
            println!(
                "Sending event to {} with API Key: {}",
                self.service_url, self.api_key
            );

            let data = json!({ "logs": self.logs.get_events() });

            println!("{:#?}", data); // Log to file located at `/tmp/tracerd.out`

            self.last_sent = Instant::now();
            self.logs.clear();

            self.http_client.send_http_event(&data).await
        } else {
            Ok(())
        }
    }

    pub async fn poll_processes(&mut self) -> Result<()> {
        self.process_watcher
            .poll_processes(&mut self.system, &mut self.logs)?;
        Ok(())
    }

    pub async fn remove_completed_processes(&mut self) -> Result<()> {
        self.process_watcher
            .remove_completed_processes(&mut self.system, &mut self.logs)?;
        Ok(())
    }

    pub fn refresh(&mut self) {
        self.system.refresh_all();
    }
}