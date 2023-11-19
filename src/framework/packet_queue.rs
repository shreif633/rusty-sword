
#[derive(Debug)]
pub struct PacketQueue {
    pub buffer: Vec<u8>
}

impl PacketQueue {
    pub fn push(self: &mut PacketQueue, buffer: &[u8]) {
        self.buffer.extend_from_slice(buffer);
    }

    pub fn size(self: &mut PacketQueue) -> u16 {
        if self.buffer.len() >= 2 {
            let slice = &self.buffer[0..2];
            u16::from_le_bytes(slice.try_into().unwrap())
        } else {
            0
        }
    }

    pub fn pop(self: &mut PacketQueue) -> Option<Vec<u8>> {
        if self.buffer.is_empty() {
            return None;
        }
        let size = self.size();
        if size == 0 {
            self.buffer.remove(0);
            return Some(vec![0]);
        }
        if self.buffer.len() < size as usize {
            return None;
        }
        let (head, tail) = self.buffer.split_at(size as usize);
        let result = head.to_vec();
        self.buffer = tail.to_vec();
        Some(result)
    }
}