# troubleshooting logs

## 21/04/2026 21:40(Done)

### type LPDWORD

pub type c_ulong = u32;Expand descriptionEquivalent to C’s unsigned long type.
This type will always be u32 or u64. Most notably, many Linux-based systems assume an u64, 
but Windows assumes u32. 
The C standard technically only requires that this type be an unsigned integer with the size of a long, 
although in practice, no system would have a ulong that is neither a u32 nor u64.

https://docs.rs/windows-win/latest/windows_win/sys/type.c_ulong.html
