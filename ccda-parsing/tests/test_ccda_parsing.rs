#![allow(clippy::all)]

use clinical_formats::ccda::{parse, AdministrativeGender, ContactPointUse, Patient, TeleContactPoint, TelecomSystem};
use pretty_assertions::assert_eq;


#[test]
fn test_1_patient_role() {
    let content =
        include_bytes!("../../ccda-samples/360 Oncology/Alice_Newman_health_summary Delegate.xml");
    let content = std::str::from_utf8(content).unwrap();
    let arena = bumpalo::Bump::new();
    let top = parse(content, &arena).unwrap();

    assert_eq!(
        top.doc.record_targets,
        vec![Patient {
            id_root: Some("2.16.840.1.113883.4.1"),
            id_extension: Some("T-10118"),
            given_names: vec![],
            family_name: None,
            suffix: None,
            birth_sex: None,
            telecontacts: vec![
                TeleContactPoint {
                    system: TelecomSystem::Phone,
                    value: "+1(555)-777-1234",
                    use_pt: Some(ContactPointUse::Mobile),
                    rank: None,
                },
                TeleContactPoint {
                    system: TelecomSystem::Phone,
                    value: "+1(555)-723-1544",
                    use_pt: Some(ContactPointUse::Home),
                    rank: None,
                },
                TeleContactPoint {
                    system: TelecomSystem::Email,
                    value: "360mu.alice.newman@gmail.com",
                    use_pt: None,
                    rank: None,
                },
            ],
            address: None,
            administrativeGender: Some(AdministrativeGender::Female),
            birth_time: None,
            race: None,
            language: None,
            ethnicity: None,
        }]
    );
}
