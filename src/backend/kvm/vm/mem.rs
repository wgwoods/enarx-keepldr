// SPDX-License-Identifier: Apache-2.0

use super::KvmUserspaceMemoryRegion;

use lset::Span;
use mmarinus::{perms, Map};
use x86_64::{PhysAddr, VirtAddr};

pub struct Region {
    kvm_region: KvmUserspaceMemoryRegion,
    _backing: Map<perms::ReadWrite>,
}

impl Region {
    pub fn new(kvm_region: KvmUserspaceMemoryRegion, backing: Map<perms::ReadWrite>) -> Self {
        Self {
            kvm_region,
            _backing: backing,
        }
    }

    pub fn backing(&mut self) -> &mut Map<perms::ReadWrite> {
        &mut self._backing
    }

    pub fn as_guest(&self) -> Span<PhysAddr, u64> {
        Span {
            start: PhysAddr::new(self.kvm_region.guest_phys_addr),
            count: self.kvm_region.memory_size,
        }
    }

    pub fn as_virt(&self) -> Span<VirtAddr, u64> {
        Span {
            start: VirtAddr::new(self.kvm_region.userspace_addr),
            count: self.kvm_region.memory_size,
        }
    }
}
