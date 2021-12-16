pub mod advertising {
    use defmt::{write, Format, Formatter};
    use heapless::Vec;
    use crate::PB_ADV;
    use crate::provisioning::Gpcf;

    pub struct PDU {
        link_id: u32,
        transaction_number: u8,
        gpcf: Gpcf,
        generic_provisioning_pdu: Vec<u8, 24>,
    }

    impl Format for PDU {
        fn format(&self, fmt: Formatter) {
            write!(fmt, "link_id: {}; transaction_number: {}; gpcf: {}", self.link_id, self.transaction_number, self.gpcf);
        }
    }

    impl PDU {
        pub fn parse(data: &[u8]) -> Option<PDU> {
            if data.len() >= 8 {
                if data[1] != PB_ADV {
                    return None;
                }
                let link_id = u32::from_be_bytes([ data[2], data[3], data[4], data[5] ] );
                let transaction_number = data[6];

                let gpcf: Gpcf = data[7].into();
                return Some(PDU {
                    link_id,
                    transaction_number,
                    gpcf,
                    generic_provisioning_pdu: Vec::new(),
                });
            }

            None
        }
    }
}