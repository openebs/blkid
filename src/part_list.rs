use crate::partition::Partition;
use blkid_sys::*;
use errors::*;
use std::ptr;

pub struct PartList(pub(crate) blkid_partlist);

impl PartList {
    pub fn get_partition(&self, partition: i32) -> Option<Partition> {
        unsafe {
            let res = blkid_partlist_get_partition(self.0, partition as libc::c_int);
            if res == ptr::null_mut() {
                None
            } else {
                Some(Partition(res))
            }
        }
    }

    pub fn get_partition_by_partno(&self, partition: i32) -> Option<Partition> {
        unsafe {
            let res = blkid_partlist_get_partition_by_partno(self.0, partition as libc::c_int);
            if res == ptr::null_mut() {
                None
            } else {
                Some(Partition(res))
            }
        }
    }

    pub fn numof_partitions(&self) -> Result<u32, BlkIdError> {
        unsafe { cvt(blkid_partlist_numof_partitions(self.0)).map(|v| v as u32) }
    }
}
