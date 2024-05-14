use x86_64::{structures::paging::PageTable, VirtAddr};

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
