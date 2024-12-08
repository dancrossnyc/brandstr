fn cpuid_ext_name_str() -> String {
    let mut buf = [0; 48];
    for (i, chunk) in buf.chunks_mut(16).enumerate() {
        const CPUID_LEAF_EXT_NAME_BASE: u32 = 0x8000_0002;

        let mut eax: u32;
        let mut ebx: u32;
        let mut ecx: u32;
        let mut edx: u32;
        unsafe {
            core::arch::asm!(
                // RBX is magical, so we can't use it directly.
                // Apologies for the Intel syntax.
                "push rbx; cpuid; mov rsi, rbx; pop rbx",
                inout("eax") CPUID_LEAF_EXT_NAME_BASE + i as u32 => eax,
                out("esi") ebx,
                inout("ecx") 0 => ecx,
                out("edx") edx,
            );
        }
        chunk[0..4].copy_from_slice(eax.to_ne_bytes().as_slice());
        chunk[4..8].copy_from_slice(ebx.to_ne_bytes().as_slice());
        chunk[8..12].copy_from_slice(ecx.to_ne_bytes().as_slice());
        chunk[12..16].copy_from_slice(edx.to_ne_bytes().as_slice());
    }
    String::from_utf8_lossy(&buf).into()
}

fn main() {
    let s = cpuid_ext_name_str();
    println!("{s}");
}
