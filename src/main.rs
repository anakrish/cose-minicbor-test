use minicbor::{Encode, CborLen};

#[derive(Encode, CborLen)]
#[cbor(array)]
struct COSESign1<'a> {
    #[n(0)]
    pub protected_header_bstr : &'a [u8],

    #[n(1)]
    pub unprocted_header: UnprotectedHeader,
}

#[derive(Encode, CborLen)]
#[cbor(map)]
struct UnprotectedHeader {
}

#[derive(Encode, CborLen)]
#[cbor(map)]
struct ProtectedHeader<'a> {
    #[n(1)]
    pub alg: i32,

    #[n(3)]
    pub content_type: &'a str,

    #[n(34)]
    pub x5t : COSECertHash<'a>
}

#[derive(Encode, CborLen)]
#[cbor(array)]
struct COSECertHash<'a> {
    #[n(0)]
    pub alg : i32,

    #[n(1)]
    pub hash: &'a [u8]
}

fn main() {
    let mut header_buffer = [0u8; 128];
    let protected_header = ProtectedHeader {
	alg: 1,
	content_type : "application/cbor",
	x5t: COSECertHash {
	    alg: 2,
	    hash : &[0, 0, 0, 0, 0, 0, 0, 0]
	}
    };

    let result = minicbor::encode(&protected_header, header_buffer.as_mut()).unwrap();
    let header_len = minicbor::len(&protected_header);
    println!("header_bytes = {}", header_len);
}
