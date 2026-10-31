use std::sync::Arc;
use std::sync::atomic::{AtomicU64, Ordering};

#[derive(Clone, Default)]
pub struct OperatorMetrics {
    status_patches_total: Arc<AtomicU64>,
    completed_missions_total: Arc<AtomicU64>,
    active_reconciliations: Arc<AtomicU64>,
}

impl OperatorMetrics {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn record_status_patch(&self) {
        self.status_patches_total.fetch_add(1, Ordering::Relaxed);
    }

    pub fn record_completed_mission(&self) {
        self.completed_missions_total
            .fetch_add(1, Ordering::Relaxed);
    }

    pub fn inc_active_reconciliation(&self) {
        self.active_reconciliations.fetch_add(1, Ordering::Relaxed);
    }

    pub fn dec_active_reconciliation(&self) {
        self.active_reconciliations.fetch_sub(1, Ordering::Relaxed);
    }

    pub fn status_patches_total(&self) -> u64 {
        self.status_patches_total.load(Ordering::Relaxed)
    }

    pub fn completed_missions_total(&self) -> u64 {
        self.completed_missions_total.load(Ordering::Relaxed)
    }

    pub fn active_reconciliations(&self) -> u64 {
        self.active_reconciliations.load(Ordering::Relaxed)
    }

    /// Computes the patch-to-mission write ratio
    pub fn patch_write_ratio(&self) -> f64 {
        let missions = self.completed_missions_total();
        if missions == 0 {
            0.0
        } else {
            self.status_patches_total() as f64 / missions as f64
        }
    }

    /// Renders Prometheus text exposition format
    pub fn render_prometheus(&self) -> String {
        format!(
            "# HELP factory_operator_status_patches_total Cumulative status patch writes\n\
             # TYPE factory_operator_status_patches_total counter\n\
             factory_operator_status_patches_total {}\n\
             # HELP factory_operator_completed_missions_total Cumulative completed missions\n\
             # TYPE factory_operator_completed_missions_total counter\n\
             factory_operator_completed_missions_total {}\n\
             # HELP factory_operator_active_reconciliations Active reconciliation loops\n\
             # TYPE factory_operator_active_reconciliations gauge\n\
             factory_operator_active_reconciliations {}\n\
             # HELP factory_operator_patch_write_ratio Average patches per completed mission\n\
             # TYPE factory_operator_patch_write_ratio gauge\n\
             factory_operator_patch_write_ratio {:.2}\n",
            self.status_patches_total(),
            self.completed_missions_total(),
            self.active_reconciliations(),
            self.patch_write_ratio()
        )
    }
}
