# troubleshooting logs

## 22/04/2026 00:57(Done)

### the Buffer-Pointer Patter

> not yet XD, Gemini
> i found 2 on "QueryFullProcessImageNameW". but both return nothing.
> https://microsoft.github.io/windows-docs-rs/doc/windows/Win32/System/Threading/fn.QueryFullProcessImageNameW.html
>  -> Result<()>
> https://docs.rs/windows-win/latest/windows_win/sys/fn.QueryFullProcessImageNameW.html
>  -> bool

The reason they seem to return "nothing" is that this function follows the Buffer-Pointer Pattern we discussed. It doesn't return the string; it writes the string into a buffer you provide and updates a length variable you also provide. by Google's Gemini

### QueryFullProcessImageNameW

The "W" Factor: Remember that since it's a W (Wide) function, the buffer must be u16, not u8.

https://microsoft.github.io/windows-docs-rs/doc/windows/Win32/System/Threading/fn.QueryFullProcessImageNameW.html
https://microsoft.github.io/windows-docs-rs/doc/windows/Win32/System/Threading/struct.PROCESS_ACCESS_RIGHTS.html

### pub const PROCESS_QUERY_LIMITED_INFORMATION: PROCESS_ACCESS_RIGHTS;
-> #[repr(transparent)]pub struct PROCESS_ACCESS_RIGHTS(pub u32);

DesiredAccess is an access mask parameter used in Windows API and driver development to specify requested types of access—such as read, write, or delete—to an object, such as a file, process, or device

bInheritHandle is a field within the Windows API SECURITY_ATTRIBUTES structure that determines whether a specific handle (a reference to a kernel object like a file, pipe, or socket) can be inherited by a new child process created by the parent

https://microsoft.github.io/windows-docs-rs/doc/windows/Win32/System/Threading/constant.PROCESS_QUERY_LIMITED_INFORMATION.html

---

## 21/04/2026 21:40(Done)

### type LPDWORD

pub type c_ulong = u32;Expand descriptionEquivalent to C’s unsigned long type.
This type will always be u32 or u64. Most notably, many Linux-based systems assume an u64, 
but Windows assumes u32. 
The C standard technically only requires that this type be an unsigned integer with the size of a long, 
although in practice, no system would have a ulong that is neither a u32 nor u64.

https://docs.rs/windows-win/latest/windows_win/sys/type.c_ulong.html
