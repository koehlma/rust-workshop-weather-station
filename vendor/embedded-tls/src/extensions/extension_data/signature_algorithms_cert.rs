use crate::buffer::CryptoBuffer;
use crate::extensions::extension_data::signature_algorithms::SignatureScheme;

use crate::TlsError;
use crate::parse_buffer::{ParseBuffer, ParseError};

use heapless::Vec;

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct SignatureAlgorithmsCert<const N: usize> {
    pub supported_signature_algorithms: Vec<SignatureScheme, N>,
}

impl<const N: usize> SignatureAlgorithmsCert<N> {
    pub fn parse(buf: &mut ParseBuffer) -> Result<Self, ParseError> {
        let data_length = buf.read_u16()? as usize;
        let mut data = buf.slice(data_length)?;
        let mut supported_signature_algorithms = Vec::new();
        // Skip unknown signature schemes per RFC 8446 Section 4.2.3
        while !data.is_empty() {
            match SignatureScheme::parse(&mut data) {
                Ok(scheme) => supported_signature_algorithms
                    .push(scheme)
                    .map_err(|_| ParseError::InsufficientSpace)?,
                Err(ParseError::InvalidData) => {} // unknown scheme, skip
                Err(e) => return Err(e),
            }
        }
        Ok(Self {
            supported_signature_algorithms,
        })
    }

    pub fn encode(&self, buf: &mut CryptoBuffer) -> Result<(), TlsError> {
        buf.with_u16_length(|buf| {
            for &a in &self.supported_signature_algorithms {
                buf.push_u16(a.as_u16())
                    .map_err(|_| TlsError::EncodeError)?;
            }
            Ok(())
        })
    }
}
