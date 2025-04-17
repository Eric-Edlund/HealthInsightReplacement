#![allow(clippy::all)]

use clinical_formats::ccda::{parse, Patient, Sex};

const ALL_VALID_SAMPLES: &[&[u8]] = &[
    include_bytes!("../../ccda-samples/360 Oncology/Alice_Newman_health_summary Delegate.xml")
];


#[test]
fn test() {
    let content = include_bytes!("../../ccda-samples/360 Oncology/Alice_Newman_health_summary Delegate.xml");
    let content = std::str::from_utf8(content).unwrap();
    let arena = bumpalo::Bump::new();
    let top = parse(content, &arena).unwrap();

    assert_eq!(top.doc.record_targets.len(), 1);
    assert_eq!(top.doc.record_targets[0], Patient {
            id_root: Some("2.16.840.1.113883.4.1"),
            id_extension: Some("T-10118"),
            sex: Sex::Unspecified,
        }
    );

}
