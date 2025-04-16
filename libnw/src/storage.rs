use core::cmp;
use core::ffi::CStr;
use core::mem;
use core::ptr;
use core::slice;

pub fn reverse32(value: u32) -> u32 {
    value.swap_bytes()
}

pub fn extapp_file_list(filenames: &mut [&'static str], extension: &str) -> i32 {
    let storage_address = extapp_address() as *mut u8;
    let mut offset = storage_address;
    let end_address = unsafe { storage_address.add(extapp_size() as usize) };

    if !extapp_is_valid(offset as *const u32) {
        return -1;
    }

    unsafe {
        offset = offset.add(4);
        let mut current_record = 0;

        while current_record < filenames.len() && offset < end_address {
            let size = *(offset as *const u16);
            if size == 0 {
                break;
            }

            let name = offset.add(2);
            let cstr = CStr::from_ptr(name as *const u8);
            filenames[current_record] = cstr.to_str().unwrap_or_default();
            offset = offset.add(size as usize);
            current_record += 1;
        }

        current_record as i32
    }
}

pub fn extapp_file_exists(filename: &str) -> bool {
    let storage_address = extapp_address() as *mut u8;
    let mut offset = storage_address;
    let end_address = unsafe { storage_address.add(extapp_size() as usize) };

    if !extapp_is_valid(offset as *const u32) {
        return false;
    }

    unsafe {
        offset = offset.add(4);

        while offset < end_address {
            let size = *(offset as *const u16);
            if size == 0 {
                break;
            }

            let name_ptr = offset.add(2) as *const u8;
            let name = CStr::from_ptr(name_ptr).to_str().unwrap_or_default();
            if name == filename {
                return true;
            }

            offset = offset.add(size as usize);
        }

        false
    }
}

pub fn extapp_file_read(filename: &str) -> Option<&'static [u8]> {
    let storage_address = extapp_address() as *mut u8;
    let mut offset = storage_address;
    let end_address = unsafe { storage_address.add(extapp_size() as usize) };

    if !extapp_is_valid(offset as *const u32) {
        return None;
    }

    unsafe {
        offset = offset.add(4);

        while offset < end_address {
            let size = *(offset as *const u16);
            if size == 0 {
                break;
            }

            let name_ptr = offset.add(2);
            let name = CStr::from_ptr(name_ptr as *const u8)
                .to_str()
                .unwrap_or_default();

            if name == filename {
                let name_len = name.len() + 1;
                let data_ptr = name_ptr.add(name_len);
                let data_len = size as usize - 2 - name_len;

                return Some(slice::from_raw_parts(data_ptr, data_len));
            }

            offset = offset.add(size as usize);
        }

        None
    }
}

pub fn extapp_file_write(filename: &str, content: &[u8]) -> bool {
    unsafe {
        let record_start_ptr = extapp_next_free() as *mut u8;
        let total_size = 2 + filename.len() + 1 + content.len();
        let record_end_ptr = record_start_ptr.add(total_size);
        let storage_end_ptr = (extapp_address() + extapp_size()) as *mut u8;

        if storage_end_ptr < record_end_ptr {
            return false;
        }

        ptr::write(record_start_ptr as *mut u16, total_size as u16);
        ptr::copy_nonoverlapping(filename.as_ptr(), record_start_ptr.add(2), filename.len());
        *record_start_ptr.add(2 + filename.len()) = 0;
        ptr::copy_nonoverlapping(
            content.as_ptr(),
            record_start_ptr.add(2 + filename.len() + 1),
            content.len(),
        );

        true
    }
}

pub fn extapp_file_erase(filename: &str) -> bool {
    let storage_address = extapp_address() as *mut u8;
    let mut offset = storage_address;
    let end_address = unsafe { storage_address.add(extapp_size() as usize) };

    if !extapp_is_valid(offset as *const u32) {
        return false;
    }

    unsafe {
        offset = offset.add(4);
        let mut record_address = ptr::null_mut();

        while offset < end_address {
            let size = *(offset as *const u16);
            if size == 0 {
                break;
            }

            let name_ptr = offset.add(2);
            let name = CStr::from_ptr(name_ptr as *const u8)
                .to_str()
                .unwrap_or_default();
            if name == filename {
                record_address = offset;
                break;
            }

            offset = offset.add(size as usize);
        }

        if record_address.is_null() {
            return false;
        }

        let len = *(record_address as *const u16) as usize;
        let next_free = extapp_next_free() as *mut u8;
        ptr::copy(
            record_address.add(len),
            record_address,
            next_free.offset_from(record_address.add(len)) as usize,
        );
        ptr::write_bytes(next_free.sub(len), 0, len);

        true
    }
}

pub fn extapp_is_valid(address: *const u32) -> bool {
    unsafe { *address == reverse32(0xBADD0BEE) }
}

pub fn extapp_next_free() -> *const u32 {
    let mut offset = extapp_address() as *mut u8;
    let end_address = unsafe { offset.add(extapp_size() as usize) };

    if !extapp_is_valid(offset as *const u32) {
        return ptr::null();
    }

    unsafe {
        offset = offset.add(4);

        while offset < end_address {
            let size = *(offset as *const u16);
            if size == 0 {
                return offset as *const u32;
            }

            offset = offset.add(size as usize);
        }

        (extapp_address() + extapp_size()) as *const u32
    }
}

pub fn extapp_used() -> u32 {
    unsafe { (extapp_next_free() as usize - extapp_address() as usize) as u32 }
}

pub fn extapp_calculator_model() -> u8 {
    let a0110_a = unsafe { *(0x90010000 as *const *const u32) };
    let a0110_b = unsafe { *(0x90410000 as *const *const u32) };
    let a0120_a = unsafe { *(0x90020000 as *const *const u32) };
    let a0120_b = unsafe { *(0x90420000 as *const *const u32) };

    let valid = |slot: *const u32| unsafe { reverse32(0xfeedc0de) == slot as u32 };

    let n0110 = valid(a0110_a) as u8 + valid(a0110_b) as u8;
    let n0120 = valid(a0120_a) as u8 + valid(a0120_b) as u8;

    match (n0110, n0120) {
        (n, 0) if n > 0 => 1,
        (0, n) if n > 0 => 2,
        (a, b) if a > b => 1,
        (a, b) if b > a => 2,
        _ => 0,
    }
}

pub fn extapp_userland_address() -> *const u32 {
    match extapp_calculator_model() {
        1 => 0x20000008 as *const u32,
        2 => 0x24000008 as *const u32,
        _ => 0x24000008 as *const u32,
    }
}

pub fn extapp_address() -> u32 {
    unsafe { *extapp_userland_address().add(3) }
}

pub fn extapp_size() -> u32 {
    unsafe { *extapp_userland_address().add(4) }
}
