#[derive(Debug, Clone)]
pub struct Packet {
    pub buffer: Vec<u8>,
    cursor: u32
}

impl Packet {
    pub fn new(buffer: &[u8]) -> Self {
        Packet { buffer: buffer.to_vec(), cursor: 0 }
    }
}

impl From<u8> for Packet {
    fn from(header: u8) -> Self {
        let mut buffer = Vec::<u8>::with_capacity(128);
        buffer.push(0);
        buffer.push(0);
        buffer.push(header);
        Packet { buffer, cursor: 0 }
    }
}

impl Packet {
    pub fn size(&mut self) -> usize {
        self.buffer.len()
    }

    pub fn get_header(&mut self) -> u8 {
        self.cursor = 3;
        if let Some(header) = self.buffer.get(2) {
            *header
        } else {
            0
        }
    }

    pub fn get_string(&mut self) -> String {
        let mut i = self.cursor as usize;
        loop {
            if let Some(&bit) = self.buffer.get(i) {
                if bit == 0 {
                    let data = self.buffer[self.cursor as usize..i].to_vec();
                    self.cursor = i as u32 + 1;
                    return String::from_utf8_lossy(&data).to_string();
                } else {
                    i += 1;
                }
            }
        }
    }

    pub fn write_string(&mut self, value: &String) {
        let bytes: &[u8] = value.as_bytes();
        let mut vector = Vec::<u8>::with_capacity(bytes.len() + 1);
        vector.extend_from_slice(bytes);
        vector.push(0);
        self.buffer.extend_from_slice(&vector);
    }

    pub fn get_buffer(&mut self, size: u32) -> Vec<u8> {
        let from = self.cursor as usize;
        let to = from + size as usize;
        self.cursor += size;
        self.buffer[from..to].to_vec()
    }

    pub fn write_buffer(&mut self, value: &[u8]) {
        self.buffer.extend_from_slice(value);
    }

    pub fn get_u8(&mut self) -> u8 {
        let i = self.cursor as usize;
        self.cursor += 1;
        if let Some(&data) = self.buffer.get(i) {
            data
        } else {
            0
        }
    }

    pub fn write_u8(&mut self, value: u8) {
        self.buffer.push(value);
    }

    pub fn get_u16(&mut self) -> u16 {
        let i = self.cursor as usize;
        let slice = &self.buffer[i..i + 2];
        self.cursor += 2;
        u16::from_le_bytes(slice.try_into().unwrap())
    }

    pub fn write_u16(&mut self, value: u16) {
        self.buffer.extend_from_slice(&value.to_le_bytes());
    }

    pub fn get_u32(&mut self) -> u32 {
        let i = self.cursor as usize;
        let slice = &self.buffer[i..i + 4];
        self.cursor += 4;
        u32::from_le_bytes(slice.try_into().unwrap())
    }

    pub fn write_u32(&mut self, value: u32) {
        self.buffer.extend_from_slice(&value.to_le_bytes());
    }

    #[allow(dead_code)]
    pub fn write_i32(&mut self, value: i32) {
        self.buffer.extend_from_slice(&value.to_le_bytes());
    }

    #[allow(dead_code)]
    pub fn get_i32(&mut self) -> i32 {
        let i = self.cursor as usize;
        let slice = &self.buffer[i..i + 4];
        self.cursor += 4;
        i32::from_le_bytes(slice.try_into().unwrap())
    }

    pub fn get_i64(&mut self) -> i64 {
        let i = self.cursor as usize;
        let slice = &self.buffer[i..i + 8];
        self.cursor += 8;
        i64::from_le_bytes(slice.try_into().unwrap())
    }

    pub fn write_i64(&mut self, value: i64) {
        self.buffer.extend_from_slice(&value.to_le_bytes());
    }

    pub fn serialize(&mut self) -> Vec<u8> {
        let size: u16 = self.buffer.len().try_into().unwrap();
        let [a, b] = size.to_le_bytes();
        self.buffer[0] = a;
        self.buffer[1] = b;
        self.buffer.shrink_to_fit();
        self.buffer.clone()
    }
}