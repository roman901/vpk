use std::io::{Read, Write, Seek, SeekFrom, Error};
use std::path::Path;
use std::fs::File;
use std::cmp;

#[derive(Debug)]
pub struct VPKEntry {
    pub dir_entry: VPKDirectoryEntry,
    pub archive_path: String,
    pub preload_data: Vec<u8>
}

impl Read for VPKEntry {
    fn read(&mut self, mut buf: &mut [u8]) -> Result<usize, Error> {
        if self.dir_entry.archive_index == 0x7fff {
            // Return internal preload_data
            buf.copy_from_slice(&self.preload_data);
            return Ok(buf.len());
        }
        let mut file = File::open(&self.archive_path)?;
        file.seek(SeekFrom::Start(self.dir_entry.archive_offset as u64));
        file.take(self.dir_entry.file_length as u64).read(buf);

        Ok(self.dir_entry.file_length as usize)
    }
}

#[derive(Debug)]
#[repr(packed)]
pub struct VPKDirectoryEntry {
    pub crc32: u32,
    pub preload_length: u16,
    pub archive_index: u16,
    pub archive_offset: u32,
    pub file_length: u32,
    pub suffix: u16
}