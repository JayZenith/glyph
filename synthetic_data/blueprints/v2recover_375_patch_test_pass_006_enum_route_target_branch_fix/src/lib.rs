#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Target {
    Cpu,
    Gpu,
    Disk,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Job {
    Render,
    Backup,
    Analyze,
    Archive,
}

pub fn default_target(job: Job, urgent: bool) -> Target {
    match job {
        Job::Render => {
            if urgent {
                Target::Cpu
            } else {
                Target::Gpu
            }
        }
        Job::Backup => {
            if urgent {
                Target::Disk
            } else {
                Target::Disk
            }
        }
        Job::Analyze => {
            if urgent {
                Target::Gpu
            } else {
                Target::Cpu
            }
        }
        Job::Archive => Target::Disk,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn render_prefers_gpu_unless_urgent() {
        assert_eq!(default_target(Job::Render, false), Target::Gpu);
        assert_eq!(default_target(Job::Render, true), Target::Cpu);
    }

    #[test]
    fn backup_always_uses_disk() {
        assert_eq!(default_target(Job::Backup, false), Target::Disk);
        assert_eq!(default_target(Job::Backup, true), Target::Disk);
    }

    #[test]
    fn analyze_uses_cpu_even_when_urgent() {
        assert_eq!(default_target(Job::Analyze, false), Target::Cpu);
        assert_eq!(default_target(Job::Analyze, true), Target::Cpu);
    }

    #[test]
    fn archive_always_uses_disk() {
        assert_eq!(default_target(Job::Archive, false), Target::Disk);
        assert_eq!(default_target(Job::Archive, true), Target::Disk);
    }
}
