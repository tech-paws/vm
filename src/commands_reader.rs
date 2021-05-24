use vm_buffers::BytesReader;
use vm_buffers::IntoVMBuffers;

pub struct CommandsReader<'a> {
    pub bytes_reader: &'a mut BytesReader,
    pub address: String,
    pub count: u64,
    command_breakpoint: u64,
    command_len: u64,
    read_commands: u64,
}

pub struct Command<'a> {
    pub id: u64,
    pub bytes_reader: &'a mut BytesReader,
}

impl<'a> CommandsReader<'a> {
    pub fn new(bytes_reader: &'a mut BytesReader) -> Self {
        let count = bytes_reader.read_u64();
        let address = String::read_from_buffers(bytes_reader);
        let command_breakpoint = bytes_reader.current_offset();

        Self {
            bytes_reader,
            address,
            count,
            command_breakpoint,
            command_len: 0,
            read_commands: 0,
        }
    }

    pub fn len(&self) -> usize {
        self.count as usize
    }

    pub fn next(&mut self) -> Option<Command> {
        assert!(
            self.bytes_reader.current_offset() <= self.command_breakpoint + self.command_len,
            "last command payload overflow"
        );

        if self.read_commands == self.count {
            return None;
        }

        let skip_bytes =
            self.command_breakpoint + self.command_len - self.bytes_reader.current_offset();

        self.bytes_reader.skip(skip_bytes);

        let command_id = self.bytes_reader.read_u64();
        self.command_len = self.bytes_reader.read_u64();
        self.command_breakpoint = self.bytes_reader.current_offset();
        self.read_commands += 1;

        Some(Command {
            id: command_id,
            bytes_reader: self.bytes_reader,
        })
    }
}

#[cfg(test)]
mod tests {
    use vm_buffers::IntoVMBuffers;
    use vm_buffers::{ByteOrder, BytesReader, BytesWriter};
    use vm_memory::RegionAllocator;

    use super::CommandsReader;

    fn write_demo(allocator: &RegionAllocator) {
        let mut bytes_writer = BytesWriter::new(ByteOrder::LittleEndian, allocator);
        bytes_writer.write_u64(5); // Commands count
        let from_address = String::from("tech.paws.tests");
        from_address.write_to_buffers(&mut bytes_writer);

        // Command 1
        bytes_writer.write_u64(1); // Command id
        bytes_writer.write_u64(8); // Command payload len
        bytes_writer.write_u32(5); // Some payload
        bytes_writer.write_u32(9); // Some payload

        // Command 2
        bytes_writer.write_u64(2); // Command id
        bytes_writer.write_u64(12); // Command payload len
        bytes_writer.write_u32(2); // Some payload
        bytes_writer.write_u32(4); // Some payload
        bytes_writer.write_u32(91); // Some payload

        // Command 3
        bytes_writer.write_u64(3); // Command id
        bytes_writer.write_u64(4); // Command payload len
        bytes_writer.write_u32(1); // Some payload

        // Command 4
        bytes_writer.write_u64(4); // Command id
        bytes_writer.write_u64(32); // Command payload len
        bytes_writer.write_u32(2); // Some payload
        bytes_writer.write_u32(4); // Some payload
        bytes_writer.write_u32(5); // Some payload
        bytes_writer.write_u32(92); // Some payload
        bytes_writer.write_u32(2); // Some payload
        bytes_writer.write_u32(4); // Some payload
        bytes_writer.write_u32(5); // Some payload
        bytes_writer.write_u32(92); // Some payload

        // Command 5
        bytes_writer.write_u64(5); // Command id
        bytes_writer.write_u64(8); // Command payload len
        bytes_writer.write_u32(99); // Some payload
        bytes_writer.write_u32(123); // Some payload
    }

    #[test]
    fn skip_commands() {
        let allocator = RegionAllocator::new(1024);
        write_demo(&allocator);
        let mut bytes_reader = BytesReader::new(ByteOrder::LittleEndian, &allocator);
        let mut commands_reader = CommandsReader::new(&mut bytes_reader);

        let command = commands_reader.next().unwrap();
        assert_eq!(command.id, 1);

        let command = commands_reader.next().unwrap();
        assert_eq!(command.id, 2);

        let command = commands_reader.next().unwrap();
        assert_eq!(command.id, 3);

        let command = commands_reader.next().unwrap();
        assert_eq!(command.id, 4);

        let command = commands_reader.next().unwrap();
        assert_eq!(command.id, 5);
    }

    #[test]
    fn partially_read_commands_payloads() {
        let allocator = RegionAllocator::new(1024);
        write_demo(&allocator);
        let mut bytes_reader = BytesReader::new(ByteOrder::LittleEndian, &allocator);
        let mut commands_reader = CommandsReader::new(&mut bytes_reader);

        // Command 1
        let command = commands_reader.next().unwrap();
        assert_eq!(command.id, 1);
        let data = command.bytes_reader.read_u32();
        assert_eq!(data, 5);

        // Command 2
        let command = commands_reader.next().unwrap();
        assert_eq!(command.id, 2);
        let data = command.bytes_reader.read_u32();
        assert_eq!(data, 2);
        let data = command.bytes_reader.read_u32();
        assert_eq!(data, 4);
        let data = command.bytes_reader.read_u32();
        assert_eq!(data, 91);

        // Command 3
        let command = commands_reader.next().unwrap();
        assert_eq!(command.id, 3);
        let data = command.bytes_reader.read_u32();
        assert_eq!(data, 1);

        let command = commands_reader.next().unwrap();
        assert_eq!(command.id, 4);
        let data = command.bytes_reader.read_u32();
        assert_eq!(data, 2);
        let data = command.bytes_reader.read_u32();
        assert_eq!(data, 4);
        let data = command.bytes_reader.read_u32();
        assert_eq!(data, 5);

        let command = commands_reader.next().unwrap();
        assert_eq!(command.id, 5);
        let data = command.bytes_reader.read_u32();
        assert_eq!(data, 99);
        let data = command.bytes_reader.read_u32();
        assert_eq!(data, 123);
    }

    #[test]
    #[should_panic]
    fn payload_overflow() {
        let allocator = RegionAllocator::new(1024);
        write_demo(&allocator);
        let mut bytes_reader = BytesReader::new(ByteOrder::LittleEndian, &allocator);
        let mut commands_reader = CommandsReader::new(&mut bytes_reader);

        let command = commands_reader.next().unwrap();
        assert_eq!(command.id, 1);
        let data = command.bytes_reader.read_u32();
        assert_eq!(data, 5);
        command.bytes_reader.read_u64();
        commands_reader.next();
    }

    #[test]
    fn read_all_commands() {
        let allocator = RegionAllocator::new(1024);
        write_demo(&allocator);
        let mut bytes_reader = BytesReader::new(ByteOrder::LittleEndian, &allocator);
        let mut commands_reader = CommandsReader::new(&mut bytes_reader);

        for _ in 0..5 {
            let command = commands_reader.next();
            assert!(command.is_some());
        }

        let command = commands_reader.next();
        assert!(command.is_none());
    }
}
