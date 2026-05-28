#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Status {
    Open,
    Held,
    Closed,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Event {
    Opened,
    PutOnHold { until_day: u32 },
    AdvanceToDay(u32),
    Resume,
    Close,
    Reopen,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Ticket {
    status: Status,
    day: u32,
    hold_until: Option<u32>,
}

impl Ticket {
    pub fn new() -> Self {
        Self {
            status: Status::Open,
            day: 0,
            hold_until: None,
        }
    }

    pub fn status(&self) -> Status {
        self.status
    }

    pub fn day(&self) -> u32 {
        self.day
    }

    pub fn hold_until(&self) -> Option<u32> {
        self.hold_until
    }

    fn expire_hold_if_needed(&mut self) {
        if self.status == Status::Held {
            if let Some(until) = self.hold_until {
                if self.day > until {
                    self.status = Status::Open;
                    self.hold_until = None;
                }
            }
        }
    }

    pub fn apply(&mut self, event: Event) {
        match event {
            Event::Opened => {
                self.status = Status::Open;
                self.hold_until = None;
            }
            Event::PutOnHold { until_day } => {
                if self.status != Status::Closed && until_day > self.day {
                    self.status = Status::Held;
                    self.hold_until = Some(until_day);
                }
            }
            Event::AdvanceToDay(day) => {
                if day >= self.day {
                    self.day = day;
                    self.expire_hold_if_needed();
                }
            }
            Event::Resume => {
                if self.status == Status::Held {
                    self.status = Status::Open;
                    self.hold_until = None;
                }
            }
            Event::Close => {
                self.status = Status::Closed;
                self.hold_until = None;
            }
            Event::Reopen => {
                if self.status == Status::Closed {
                    self.status = Status::Open;
                }
            }
        }
    }
}

pub fn replay(events: &[Event]) -> Ticket {
    let mut ticket = Ticket::new();
    for &event in events {
        ticket.apply(event);
    }
    ticket
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hold_remains_active_through_its_until_day() {
        let ticket = replay(&[
            Event::PutOnHold { until_day: 3 },
            Event::AdvanceToDay(3),
        ]);

        assert_eq!(ticket.status(), Status::Held);
        assert_eq!(ticket.hold_until(), Some(3));
    }

    #[test]
    fn hold_expires_only_after_passing_until_day() {
        let ticket = replay(&[
            Event::PutOnHold { until_day: 3 },
            Event::AdvanceToDay(4),
        ]);

        assert_eq!(ticket.status(), Status::Open);
        assert_eq!(ticket.hold_until(), None);
    }

    #[test]
    fn closed_ticket_ignores_new_hold_requests() {
        let ticket = replay(&[
            Event::Close,
            Event::PutOnHold { until_day: 10 },
            Event::AdvanceToDay(10),
        ]);

        assert_eq!(ticket.status(), Status::Closed);
        assert_eq!(ticket.hold_until(), None);
    }

    #[test]
    fn reopen_after_close_returns_to_open_without_restoring_old_hold() {
        let ticket = replay(&[
            Event::PutOnHold { until_day: 5 },
            Event::Close,
            Event::Reopen,
        ]);

        assert_eq!(ticket.status(), Status::Open);
        assert_eq!(ticket.hold_until(), None);
    }

    #[test]
    fn earlier_day_advance_is_ignored() {
        let ticket = replay(&[
            Event::AdvanceToDay(5),
            Event::PutOnHold { until_day: 7 },
            Event::AdvanceToDay(4),
        ]);

        assert_eq!(ticket.day(), 5);
        assert_eq!(ticket.status(), Status::Held);
        assert_eq!(ticket.hold_until(), Some(7));
    }

    #[test]
    fn explicit_resume_clears_hold_even_before_expiry() {
        let ticket = replay(&[
            Event::PutOnHold { until_day: 8 },
            Event::Resume,
            Event::AdvanceToDay(9),
        ]);

        assert_eq!(ticket.status(), Status::Open);
        assert_eq!(ticket.hold_until(), None);
    }
}
