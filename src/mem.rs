use windows::Win32::{
    Foundation::HINSTANCE,
    System::{Diagnostics::Debug::IMAGE_NT_HEADERS32, SystemServices::IMAGE_DOS_HEADER},
};

fn pattern_scan(module: HINSTANCE, pattern: &str) -> *mut usize {
    unsafe {
        let dos_headers = module as *mut IMAGE_DOS_HEADER;
        let module_addr = module as usize;
        let e_lfanew = (*dos_headers).e_lfanew as i32;
        let nt_headers = (module_addr + e_lfanew as usize) as *mut IMAGE_NT_HEADERS32;
        let size_of_image = (*nt_headers).OptionalHeader.SizeOfImage as usize;
        let pattern_bytes = pattern
            .replace(' ', "")
            .as_bytes()
            .chunks(2)
            .map(std::str::from_utf8)
            .collect::<Result<Vec<&str>, _>>() // Redundant
            .unwrap()
            .into_iter()
            .filter(|&q| !q.contains('?'))
            .map(|q| i32::from_str_radix(q, 16).unwrap())
            .collect::<Vec<i32>>();
        let bytes = module_addr as *mut usize;
        let size = pattern_bytes.len();
        for i in 0..(size_of_image - size as usize) {
            let mut found = true;
            for j in 0..size {
                if *bytes.offset(i as isize + j as isize) != pattern_bytes[j] as _
                    && pattern_bytes[j] != -1
                {
                    found = false;
                    break;
                }
            }

            if found {
                return bytes.offset(i as _) as *mut usize;
            }
        }

        return 0 as _;
    }
}
