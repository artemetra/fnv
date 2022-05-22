pub mod file_reader {
    use std::error::Error;
    use crate::curve::CurveType;
    use std::fmt;

    #[derive(Debug)]
    pub struct InvalidCurveTypeError;

    impl fmt::Display for InvalidCurveTypeError {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "{:?}", self)
        }
    }

    impl Error for InvalidCurveTypeError {}

    pub fn read_fnv_file(_file: &Vec<u8>) -> Result<(), Box<dyn Error>>{
        let file_iter = _file.iter();
        // TODO: fix whatever this is
        let curve_type = curve_type(file_iter.next().unwrap())?;

        Ok(()) // should be removed
    }

    pub fn curve_type(byte: &u8) -> Result<CurveType, InvalidCurveTypeError> {
        match byte {
            1 => Ok(CurveType::Envelope),
            2 => Ok(CurveType::Lfo),
            3 => Ok(CurveType::Graph),
            7 => Ok(CurveType::Map),
            _ => Err(InvalidCurveTypeError),
        }
    }

}