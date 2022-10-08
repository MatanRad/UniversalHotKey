use anyhow::Result;

pub type EventTimestamp = [u8; 16];
pub type EventCode = u16;
pub type EventValue = i32;

#[derive(Debug, PartialEq)]
pub enum KeyEventValue {
    DOWN = 0,
    UP = 1,
    HELD = 2,
}

#[derive(Debug, PartialEq)]
pub enum EventType {
    SYN = 0x00,
    KEY = 0x01,
    REL = 0x02,
    ABS = 0x03,
    MSC = 0x04,
    SW = 0x05,
    LED = 0x11,
    SND = 0x12,
    REP = 0x14,
    FF = 0x15,
    PWR = 0x16,
    FFSTATUS = 0x17,
}

#[repr(C)]
pub struct LinuxInputEvent {
    pub timeval: EventTimestamp,
    pub event_type: EventType,
    pub code: EventCode,
    pub value: EventValue,
}

impl EventType {
    pub fn from_u16(value: u16) -> Result<EventType> {
        match value {
            0x00 => Ok(EventType::SYN),
            0x01 => Ok(EventType::KEY),
            0x02 => Ok(EventType::REL),
            0x03 => Ok(EventType::ABS),
            0x04 => Ok(EventType::MSC),
            0x05 => Ok(EventType::SW),
            0x11 => Ok(EventType::LED),
            0x12 => Ok(EventType::SND),
            0x14 => Ok(EventType::REP),
            0x15 => Ok(EventType::FF),
            0x16 => Ok(EventType::PWR),
            0x17 => Ok(EventType::FFSTATUS),
            _ => return Err(anyhow::anyhow!("Invalid Event Type!")),
        }
    }
}

impl KeyEventValue {
    pub fn from_i32(value: i32) -> Result<KeyEventValue> {
        match value {
            0x00 => Ok(KeyEventValue::DOWN),
            0x01 => Ok(KeyEventValue::UP),
            0x02 => Ok(KeyEventValue::HELD),
            _ => return Err(anyhow::anyhow!("Invalid Key Code ({})!", value)),
        }
    }
}
