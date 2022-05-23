pub mod file_reader {
    use crate::curve::CurveType;
    use std::error::Error;
    use std::fmt;

    #[derive(Debug)]
    pub enum FnvReadError {
        /// file size too small to process
        FileTooSmall,
        /// invalid curve type in file
        InvalidCurveType,
    }

    impl fmt::Display for FnvReadError {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "{:?}", self)
        }
    }

    impl Error for FnvReadError {}

    pub fn read_fnv_file(_file: &Vec<u8>) -> Result<(), Box<dyn Error>> {
        if _file.len() < 56 { // smallest size i've found
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
            _ => Err(FnvReadError::InvalidCurveType),
        }
    }
}
