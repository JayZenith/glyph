#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Event {
    Connect,
    Disconnect,
    Message { urgent: bool, bytes: usize },
    Heartbeat,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Status {
    Idle,
    Active,
    Busy,
    Offline,
}

pub fn next_status(current: Status, event: Event) -> Status {
    match event {
        Event::Connect => Status::Active,
        Event::Disconnect => Status::Offline,
        Event::Message { urgent, bytes } => match (urgent, bytes) {
            (true, _) => Status::Active,
            (_, 0) => current,
            (false, 1..=32) => Status::Idle,
            (false, _) => Status::Busy,
        },
        Event::Heartbeat => Status::Idle,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn connect_and_disconnect_have_direct_targets() {
        assert_eq!(next_status(Status::Idle, Event::Connect), Status::Active);
        assert_eq!(next_status(Status::Busy, Event::Disconnect), Status::Offline);
    }

    #[test]
    fn urgent_messages_are_busy_even_when_small() {
        assert_eq!(
            next_status(
                Status::Active,
                Event::Message {
                    urgent: true,
                    bytes: 4,
                }
            ),
            Status::Busy
        );
    }

    #[test]
    fn empty_message_keeps_current_status() {
        assert_eq!(
            next_status(
                Status::Active,
                Event::Message {
                    urgent: false,
                    bytes: 0,
                }
            ),
            Status::Active
        );
    }

    #[test]
    fn small_nonurgent_message_marks_active() {
        assert_eq!(
            next_status(
                Status::Idle,
                Event::Message {
                    urgent: false,
                    bytes: 12,
                }
            ),
            Status::Active
        );
    }

    #[test]
    fn heartbeat_preserves_offline_but_idles_others() {
        assert_eq!(next_status(Status::Offline, Event::Heartbeat), Status::Offline);
        assert_eq!(next_status(Status::Busy, Event::Heartbeat), Status::Idle);
    }
}
