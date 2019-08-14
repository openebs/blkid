use super::*;
use blkid_sys::*;

pub struct Partition(pub(crate) blkid_partition);

impl Partition {
    // TODO: get_flags();

    pub fn get_name<'a>(&'a self) -> Option<&'a str> {
        unsafe { cstr_to_str(blkid_partition_get_name(self.0)) }
    }

    pub fn get_partno(&self) -> i32 {
        unsafe { blkid_partition_get_partno(self.0) }
    }

    pub fn get_size(&self) -> i64 {
        unsafe { blkid_partition_get_size(self.0) }
    }

    pub fn get_start(&self) -> i64 {
        unsafe { blkid_partition_get_start(self.0) }
    }

    // pub fn get_table();

    pub fn get_type(&self) -> i32 {
        unsafe { blkid_partition_get_type(self.0) }
    }

    pub fn get_type_string<'a>(&'a self) -> Option<&'a str> {
        unsafe { cstr_to_str(blkid_partition_get_type_string(self.0)) }
    }

    /// Obtains the UUID from the partition table, most commonly known as the PartUUID.
    pub fn get_uuid<'a>(&'a self) -> Option<&'a str> {
        unsafe { cstr_to_str(blkid_partition_get_uuid(self.0)) }
    }

    pub fn is_extended(&self) -> bool {
        unsafe { blkid_partition_is_extended(self.0) != 0 }
    }

    pub fn is_logical(&self) -> bool {
        unsafe { blkid_partition_is_logical(self.0) != 0 }
    }

    pub fn is_primary(&self) -> bool {
        unsafe { blkid_partition_is_primary(self.0) != 0 }
    }
}
