use std::ffi::CStr;
use std::os::raw::c_char;
use std::slice;

use c2pa::ManifestStore;

// Pull in default certs so the binary can self config
const SIGN_ALG: c2pa::SigningAlg = c2pa::SigningAlg::Es256;
const SIGN_CERTS: &[u8] = include_bytes!("../rsc/es256_certs.pem");
const PRIVATE_KEY: &[u8] = include_bytes!("../rsc/es256_private.key");

#[no_mangle]
// verify an asset with the given mime type from a byte array of length
pub extern "C" fn verify_bytes(format: *const c_char, bytes: *const u8, length: usize) {
    // convert C pointers into Rust
    let format = unsafe { CStr::from_ptr(format).to_string_lossy().into_owned() };
    let bytes: &[u8] = unsafe { slice::from_raw_parts(bytes, length as usize) };

    // Verify the manifests
    match ManifestStore::from_bytes(&format, bytes, true) {
        Ok(manifest_store) => println!("{}", manifest_store),
        Err(e) => eprintln!("Error {:?}", e),
    }
}

//#[cfg(feature = "sign")]
#[no_mangle]
pub extern "C" fn sign_bytes(
    format: *const c_char,
    source_bytes: *const u8,
    length: usize,
    dest_bytes: *mut u8,
    dest_size: usize,
) -> i64 {
    // convert C pointers into Rust
    let format = unsafe { CStr::from_ptr(format).to_string_lossy().into_owned() };
    let source_bytes: &[u8] = unsafe { slice::from_raw_parts(source_bytes, length as usize) };
    let dest_bytes: &mut [u8] =
        unsafe { slice::from_raw_parts_mut(dest_bytes, dest_size as usize) };

    let signer = match c2pa::create_signer::from_keys(SIGN_CERTS, &PRIVATE_KEY, SIGN_ALG, None) {
        Ok(signer) => signer,
        Err(e) => {
            eprintln!("Create Signer Error: {}", e);
            return -1;
        }
    };

    // convert buffer to cursor with Read/Write/Seek capability
    let mut stream = std::io::Cursor::new(source_bytes.to_vec());

    // this example creates a minimal manifest as an example,
    // a real manifest should include a thumbnail
    let mut manifest = c2pa::Manifest::new("my_app".to_owned());
    manifest.set_title("EmbedStream");
    manifest
        .add_assertion(&c2pa::assertions::User::new(
            "org.contentauth.mylabel",
            r#"{"my_tag":"Anything I want"}"#,
        ))
        .unwrap();

    // Embed a manifest using the signer.
    let _manifest_bytes = match manifest.embed_stream(&format, &mut stream, signer.as_ref()) {
        Ok(manifest) => manifest,
        Err(e) => {
            eprintln!("Embed Stream Error: {}", e);
            return -1;
        }
    };

    // get the updated asset
    let bytes = stream.into_inner();

    // copy the signed asset into the output buffer
    if dest_size >= bytes.len() {
        dest_bytes[..bytes.len()].clone_from_slice(&bytes);
    } else {
        eprintln!("dest_size too small, {} bytes required", bytes.len());
        return -1;
    }

    // Verify the manifests
    match ManifestStore::from_bytes(&format, &bytes, true) {
        Ok(manifest_store) => println!("{}", manifest_store),
        Err(e) => {
            eprintln!("Manifest from bytes Error {:?}", e);
            return -1;
        }
    };
    bytes.len() as i64
}
