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
}
