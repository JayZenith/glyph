enum Job {
    Backup,
    Cleanup,
    Sync,
    Report,
}

enum Outcome {
    Completed,
    Deferred,
    Ignored,
    Pending,
}

fn decide(job: &Job) -> Outcome {
    match job {
        Job::Backup => Outcome::Pending,
        Job::Cleanup => Outcome::Completed,
        Job::Sync => Outcome::Ignored,
        Job::Report => Outcome::Deferred,
    }
}

fn label(outcome: &Outcome) -> &'static str {
    match outcome {
        Outcome::Completed => "done",
        Outcome::Deferred => "deferred",
        Outcome::Ignored => "skipped",
        Outcome::Pending => "waiting",
    }
}

fn main() {
    let jobs = [Job::Backup, Job::Cleanup, Job::Sync, Job::Report];
    let mut done = 0;
    let mut skipped = 0;
    let mut waiting = 0;
    let mut deferred = 0;

    for job in jobs.iter() {
        let outcome = decide(job);
        let name = match job {
            Job::Backup => "backup",
            Job::Cleanup => "cleanup",
            Job::Sync => "sync",
            Job::Report => "report",
        };
        let status = label(&outcome);
        println!("{} => {}", name, status);
        match outcome {
            Outcome::Completed => done += 1,
            Outcome::Deferred => deferred += 1,
            Outcome::Ignored => skipped += 1,
            Outcome::Pending => waiting += 1,
        }
    }

    println!(
        "summary: done={}, skipped={}, waiting={}, deferred={}",
        done, skipped, waiting, deferred
    );
}
