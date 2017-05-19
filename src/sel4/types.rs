// Arch-independent types.

pub struct CNode {} // n-bit CNode 16 ∗ 2n bytes (where n ≥ 2)
pub struct Untyped {} // 2n bytes (where n ≥ 4)

pub struct Endpoint {} // 16 bytes
pub struct Notification {} // 16 bytes

pub struct IRQControl {}
pub struct IRQHandler {}


pub mod ia32
{
    // IA32 specific types.
    pub struct ThreadControlBlock {} // 1 KiB
    pub struct Frame {} // 4 KiB
    pub struct BigFrame {} // 4 MiB
    pub struct PageDirectory {} // 4KiB
    pub struct PageTable {} //  4KiB
    pub struct ASIDControl {}
    pub struct ASIDPool {} // 4 KiB
    pub struct Port {}
    pub struct IOSpace {}
    pub struct IOPageTable {} // 4 KiB
}

pub mod arm {
    // ARM specific types
    pub struct ThreadControlBlock {} // 512 bytes
    pub struct SmallFrame {} // 4 KiB
    pub struct LargeFrame {} // 64 KiB
    pub struct Section {} // 1 MiB
    pub struct SuperSection {} // 16 MiB
    pub struct PageDirectory {} // 16 KiB
    pub struct PageTable {} // 1 Kib
    pub struct ASIDControl {}
    pub struct ASIDPool {} // 4 KiB
}