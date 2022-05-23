pub mod file_reader {
    use crate::curve::CurveType;
    use std::error::Error;
    use std::fmt;

    #[derive(Debug)]
    pub struct FnvReadError {
        pub kind: FnvReadErrorKind,
    }

    #[derive(Debug)]
    pub enum FnvReadErrorKind {
        /// file size too small to process
        FileTooSmall,
        /// invalid curve type in file
        InvalidCurveType(u8),
    }

    impl fmt::Display for FnvReadError {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "{:?}", self)
        }
    }
    impl fmt::Display for FnvReadErrorKind {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "{:?}", self.to_string())
        }
    }

    impl Error for FnvReadError {}

    pub fn read_fnv_file(_file: &Vec<u8>) -> Result<(), FnvReadError> {
        // smallest size i've found
        if _file.len() < 56 {
            return Err(FnvReadError {
                kind: FnvReadErrorKind::FileTooSmall,
            });
        }
        let mut file_iter = _file.iter();

        let curve_type = curve_type(file_iter.next().unwrap())?;
        
        Ok(()) // should be removed
    }

    pub fn curve_type(byte: &u8) -> Result<CurveType, FnvReadError> {
        match byte {
            1 => Ok(CurveType::Envelope),
            2 => Ok(CurveType::Lfo),
            3 => Ok(CurveType::Graph),
            7 => Ok(CurveType::Map),
            n @ _ => Err(FnvReadError {
                kind: FnvReadErrorKind::InvalidCurveType(*n),
            }),
        }
    }

    // huge thanks to this:
    // https://reverseengineering.stackexchange.com/questions/30363/floating-point-number-mangled-in-a-proprietary-file/30364#30364
    pub fn read_fnv_float(bytes: u32) -> f32 {
        const FNV_FLOAT_MASK: u32 = 0x7000000;
        let f = ror32(bytes, 3);
        let f = swaplow3(f);
        let f = f ^ FNV_FLOAT_MASK;
        f32::from_bits(f)
    }

    fn ror32(x: u32, n: u32) -> u32 {
        let n = n & 31;
        let low = x >> n;
        let high = (x << (32 - n)) & 0xFFFFFFFF;
        high|low
    }

    fn swaplow3(x: u32) -> u32 {
        let x0 = x & 0xFF;
        let x1 = (x >> 8) & 0xFF;
        let x2 = (x >> 16) & 0xFF;
        let x3 = (x >> 24) & 0xFF;
        (x3<<24) | (x0<<16) | (x1<<8) | x2
    }


}
