use x86_64::{structures::paging::PageTable, PhysAddr, VirtAddr};

/// Returns mut ref to active L4 page table.
///
/// Unsafe, caller must guarantee that the complete
/// phys mem is mapped to virt mem at the passed
/// `phys_mem_offset`.
///
/// Must only be called once to avoid aliasing `&mut` refrences (UB).`
pub unsafe fn active_level_4_table(phys_mem_offset: VirtAddr) -> &'static mut PageTable {
    use x86_64::registers::control::Cr3;

    let (level_4_table_frame, _) = Cr3::read();
    let phys = level_4_table_frame.start_address();
    let virt = phys_mem_offset + phys.as_u64();
    let page_table_ptr: *mut PageTable = virt.as_mut_ptr();

    &mut *page_table_ptr // unsafe
}

/// Translates given virt addr to mapped phys addr or `None`.
///
/// Unsafe, caller must guarantee that the complete
/// phys mem is mapped to virt mem at the passed
/// `phys_mem_offset`.
pub unsafe fn translate_addr(addr: VirtAddr, phys_mem_offset: VirtAddr) -> Option<PhysAddr> {
    translate_addr_inner(addr, phys_mem_offset)
}

// Safe to limit the scope of `unsafe` to specific blocks rather
// than entire function scope. Must only be reachable externally via `unsafe fn`.
fn translate_addr_inner(addr: VirtAddr, phys_mem_offset: VirtAddr) -> Option<PhysAddr> {
    use x86_64::registers::control::Cr3;
    use x86_64::structures::paging::page_table::FrameError;
    // read active L4 entry from CR3
    let (level_4_table_frame, _) = Cr3::read();
    let table_indexes = [
        addr.p4_index(),
        addr.p3_index(),
        addr.p2_index(),
        addr.p1_index(),
    ];
    let mut frame = level_4_table_frame;

    // add phys offset to each index.
    for &index in &table_indexes {
        // convert into page table reference
        let virt = phys_mem_offset + frame.start_address().as_u64();
        let table_ptr: *const PageTable = virt.as_ptr();
        let table = unsafe { &*table_ptr };

        // read PT entry and update `frame`
        let entry = &table[index];
        frame = match entry.frame() {
            Ok(frame) => frame,
            Err(FrameError::FrameNotPresent) => return None,
            Err(FrameError::HugeFrame) => panic!("huge pages not supported"),
        };
    }
    // Calculate phys addr by adding page offset
    Some(frame.start_address() + u64::from(addr.page_offset()))
}
