#![allow(
    non_snake_case,
    clippy::needless_lifetimes,
    clippy::needless_if,
    clippy::let_and_return,
    clippy::single_match
)]

use bumpalo::Bump;
use core::str;
use quick_xml::{
    Reader,
    events::{BytesStart, Event},
};
use std::ops::Deref;
use std::cell::RefCell;

#[derive(Debug, Eq, PartialEq)]
pub enum Sex {
    Male,
    Female,
}

// https://www.hl7.org/documentcenter/public/standards/vocabulary/vocabulary_tables/infrastructure/vocabulary/AdministrativeGender.html
#[derive(Default, Debug, Eq, PartialEq)]
pub enum AdministrativeGender {
    // TODO: Do we care about this special value?
    #[default]
    Undifferentiated,
    Male,
    Female,
}

/// A time type which preserves the source format's ambiguity
#[derive(Debug, Default, Eq, PartialEq)]
pub struct AmbiguousTime {}

#[derive(Debug, Default, Eq, PartialEq)]
pub enum Race {
    #[default]
    Unknown,
}
#[derive(Debug, Default, Eq, PartialEq)]
pub enum Ethnicity {
    #[default]
    Unknown,
}
#[derive(Debug, Default, Eq, PartialEq)]
pub enum SpokenLanguage {
    #[default]
    Unknown,
}

/// http://hl7.org/fhir/R5/valueset-contact-point-use.html
#[derive(Debug, Eq, PartialEq)]
pub enum ContactPointUse {
    Home,
    Work,
    Mobile,
    Temporary,
    // Address no longer in use or was never correct to begin with.
    Old,
}

#[derive(Debug, Eq, PartialEq)]
pub enum AddrType {
    Postal,
    Physical,
    Both,
}

/// https://www.hl7.org/fhir/us/qicore/STU2/quick/pages/Address.html
/// But we left out the time period that it was active.
/// TODO: Is period important? How best respresent?
#[derive(Debug, Eq, PartialEq)]
pub struct Address<'a> {
    association: Option<ContactPointUse>,
    typ: Option<AddrType>,
    city: Option<&'a str>,
    country: Option<rust_iso3166::CountryCode>,
    district: Option<&'a str>,
    line: Vec<&'a str>,
    postal_code: Option<&'a str>,
    state: Option<&'a str>,
}

// http://hl7.org/fhir/R5/valueset-contact-point-system.html
#[derive(Debug, Eq, PartialEq)]
pub enum TelecomSystem {
    Phone,
    Email,
    Fax,
    Pager,
    Url,
    Sms,
    Other(Option<String>),
}

/// http://hl7.org/fhir/R5/datatypes.html#ContactPoint
/// Did not include period TODO: Should period be represented here?
#[derive(Debug, Eq, PartialEq)]
pub struct TeleContactPoint<'a> {
    pub system: TelecomSystem,
    pub value: &'a str,
    pub use_pt: Option<ContactPointUse>,
    pub rank: Option<u16>,
}

#[derive(Default, Debug, Eq, PartialEq)]
pub struct Patient<'a> {
    pub id_root: Option<&'a str>,
    pub id_extension: Option<&'a str>,

    pub given_names: Vec<&'a str>,
    pub family_name: Option<&'a str>,
    pub suffix: Option<&'a str>,

    pub administrativeGender: Option<AdministrativeGender>,
    // TODO: Standard says this should be in an Observation. Should we represent it here?
    pub birth_sex: Option<Sex>,
    pub birth_time: Option<AmbiguousTime>,

    pub race: Option<Race>,
    pub ethnicity: Option<Ethnicity>,
    pub language: Option<SpokenLanguage>,

    pub telecontacts: Vec<TeleContactPoint<'a>>,
    pub address: Option<Address<'a>>,
}

pub struct ClinicalDocument<'a> {
    /// The records which this document's content applies to.
    /// Usually one person, except in the rare case of group note taking.
    pub record_targets: Vec<Patient<'a>>,
}

pub struct CcdaTopLevel<'a> {
    pub doc: ClinicalDocument<'a>,
    pub data: &'a Bump,
}

#[derive(Debug)]
pub enum CcdaParseError {}

#[allow(dead_code)]
pub fn parse<'r>(src: &str, arena: &'r Bump) -> PResult<CcdaTopLevel<'r>> {
    let reader = Reader::from_str(src);

    let ctx = Context {
        reader: RefCell::new(reader),
        last_given: RefCell::new(None),
    };

    let doc: CcdaTopLevel<'r> = p_top_level(ctx, arena)?;
    Ok(doc)
}

type PResult<T> = Result<T, CcdaParseError>;

struct Context<'src> {
    reader: RefCell<Reader<&'src [u8]>>,
    last_given: RefCell<Option<Event<'src>>>,
}

impl<'src> Context<'src> {
    /// Skips the element matching the Start event we just received.
    /// Must only be called after a start event.
    fn skip_rec(&self) {
        let mut depth = 1;
        let Some(tag) = self.last_given.borrow().clone() else {
            todo!()
        };
        let Event::Start(tag) = tag else { todo!() };
        let start_tag = str::from_utf8(tag.local_name().into_inner()).unwrap();
        // println!("Skipping rec: {}", start_tag);
        while depth > 0 {
            let Ok(ev) = self.reader.borrow_mut().read_event() else {
                todo!()
            };
            match ev {
                Event::Start(ref t) => {
                    // println!("<{:?}>", t);
                    depth += 1;
                }
                Event::End(ref t) => {
                    depth -= 1;
                    // println!("</{:?}> -> {}", t, depth);
                }
                _ => {}
            }

            *self.last_given.borrow_mut() = Some(ev.clone());
        }

        let Some(end_tag) = &*self.last_given.borrow() else {
            todo!()
        };
        let Event::End(end_tag) = end_tag else {
            todo!()
        };
        assert_eq!(
            start_tag,
            str::from_utf8(end_tag.local_name().into_inner()).unwrap()
        )
    }
}

impl<'src> Iterator for &Context<'src> {
    type Item = Event<'src>;

    /// Handles xml parse errors and end of file.
    /// Returns none after encountering end tag
    fn next(&mut self) -> Option<Self::Item> {
        match self.reader.borrow_mut().read_event() {
            Err(_) => todo!(),
            Ok(ev) => match ev {
                Event::Eof => None,
                Event::End(tag) => {
                    // println!("End tag: {:?}", tag);
                    None
                }
                ev => {
                    // println!("Pulled {:?}", ev);
                    *self.last_given.borrow_mut() = Some(ev.clone());
                    Some(ev)
                }
            },
        }
    }
}

fn p_top_level<'src, 'r>(ctx: Context<'src>, arena: &'r Bump) -> PResult<CcdaTopLevel<'r>> {
    let mut clinical_doc: Option<ClinicalDocument> = None;

    for ev in &ctx {
        use Event::*;
        match ev {
            Start(open_tag) => {
                match open_tag.local_name().into_inner() {
                    b"ClinicalDocument" => {
                        assert!(clinical_doc.is_none());
                        let patient = p_clinical_document(&ctx, arena, &open_tag)?;
                        clinical_doc = Some(patient);
                    }
                    t => todo!("{:?}", str::from_utf8(t).unwrap()),
                };
            }
            Empty(_bytes_start) => todo!(),
            CData(_bytes_cdata) => todo!(),
            Decl(_bytes_decl) => {}
            DocType(_bytes_text) => todo!(),
            Text(..) | PI(..) | Comment(..) => {} // Meaningless
            v => panic!("{:?}", v),
        }
    }

    let Some(clinical_document) = clinical_doc else {
        todo!()
    };

    let res = Ok(CcdaTopLevel {
        doc: clinical_document,
        data: arena,
    });

    res
}

fn p_clinical_document<'r>(
    ctx: &Context,
    arena: &'r Bump,
    opening: &BytesStart,
) -> PResult<ClinicalDocument<'r>> {
    let mut record_targets = Vec::<Patient<'r>>::new();

    for ev in ctx {
        // println!("Clinical Doc: {:?}", ev);
        match ev {
            Event::Start(open_tag) => {
                println!("{:?}", open_tag);
                match open_tag.local_name().into_inner() {
                    b"recordTarget" => {
                        let patient = p_record_target(ctx, arena, &open_tag)?;
                        record_targets.push(patient);
                    }
                    _ => {
                        ctx.skip_rec();
                    }
                };
            }
            Event::Empty(tag) => {
                match tag.local_name().into_inner() {
                    b"recordTarget" => {
                        let patient = p_record_target(ctx, arena, &tag)?;
                        record_targets.push(patient);
                    }
                    _ => {}
                }
            }
            Event::CData(_bytes_cdata) => todo!(),
            Event::Decl(_bytes_decl) => {}
            Event::DocType(_bytes_text) => todo!(),
            _ => {}
        }
    }

    if record_targets.is_empty() {
        // Specification doesn't allow this
        // https://build.fhir.org/ig/HL7/CDA-ccda-2.1-sd/document_level_guidance.html
        todo!()
    }

    Ok(ClinicalDocument { record_targets })
}

fn p_record_target<'src, 'r>(
    ctx: &Context<'src>,
    arena: &'r Bump,
    opening: &BytesStart,
) -> PResult<Patient<'r>> {
    assert_eq!(
        str::from_utf8(opening.local_name().into_inner()).unwrap(),
        "recordTarget"
    );
    assert!(opening.attributes().next().is_none());

    let mut patient_role: Option<Patient> = None;

    for ev in ctx {
        use Event::*;
        match ev {
            Start(ref tag) => {
                match tag.name().local_name().into_inner() {
                    b"patientRole" => {
                        assert!(patient_role.is_none());
                        patient_role = Some(p_patient_role(ctx, arena, tag)?);
                    }
                    t => todo!("{:?}", str::from_utf8(t).unwrap()),
                };
            }
            Empty(_) => todo!(),
            Decl(_) => todo!(),
            PI(_) => todo!(),
            DocType(_) => todo!(),
            CData(_) | Text(_) | Comment(_) => {} // Meaningless
            _ => {}
        }
    }

    let Some(patient) = patient_role else {
        // Standard says patient role must be present
        todo!()
    };

    Ok(patient)
}

/// The source string lives 'a
/// the saved content in the bump allocator lives 'b
fn p_patient_role<'src, 'r>(
    ctx: &Context<'src>,
    arena: &'r Bump,
    opening: &BytesStart,
) -> PResult<Patient<'r>> {
    assert_eq!(
        str::from_utf8(opening.local_name().into_inner()).unwrap(),
        "patientRole"
    );
    let mut seen_patient_id = false;
    let mut seen_patient = false;
    let mut patient = Patient::default();

    for ev in ctx {
        use Event::*;
        match ev {
            Start(ref tag) => {
                let name = tag.name().local_name().into_inner();
                match name {
                    b"patient" => {
                        assert!(!seen_patient);
                        seen_patient = true;
                        p_patient(ctx, arena, &mut patient, tag)?;
                    }
                    _ => ctx.skip_rec(),
                };
            }
            Empty(ref tag) => {
                match tag.name().local_name().into_inner() {
                    b"id" => {
                        assert!(!seen_patient_id);
                        seen_patient_id = true;
                        p_patient_role__id(ctx, arena, &mut patient, tag)?;
                    }
                    b"telecom" => {
                        p_patient_role__telecom(ctx, arena, &mut patient, tag)?;
                    }
                    _ => {}
                };
            }
            Text(_) | CData(_) | Comment(_) => {} // Meaningless
            Decl(_bytes_decl) => todo!(),
            PI(_bytes_pi) => todo!(),
            DocType(_bytes_text) => todo!(),
            _ => panic!(),
        }
    }

    assert!(seen_patient_id);
    Ok(patient)
}

fn p_patient<'r>(
    ctx: &Context,
    arena: &'r Bump,
    patient: &mut Patient<'r>,
    opening: &BytesStart,
) -> PResult<()> {
    assert_eq!(
        str::from_utf8(opening.local_name().into_inner()).unwrap(),
        "patient"
    );

    let mut seen_admin_gender_code = false;

    for ev in ctx {
        use Event::*;
        match ev {
            Start(ref tag) => {
                let name = tag.name().local_name().into_inner();
                match name {
                    _ => ctx.skip_rec(),
                };
            }
            Empty(ref tag) => {
                match tag.name().local_name().into_inner() {
                    b"administrativeGenderCode" => {
                        assert!(!seen_admin_gender_code);
                        seen_admin_gender_code = true;
                        p_patient__admin_gender_code(ctx, arena, patient, tag)?;
                    }
                    _ => {}
                };
            }
            Text(_) | CData(_) | Comment(_) => {} // Meaningless
            Decl(_bytes_decl) => todo!(),
            PI(_bytes_pi) => todo!(),
            DocType(_bytes_text) => todo!(),
            _ => panic!(),
        }
    }

    Ok(())
}

fn p_patient_role__id<'src, 'r>(
    _ctx: &Context<'src>,
    arena: &'r Bump,
    patient: &mut Patient<'r>,
    opening: &BytesStart,
) -> PResult<()> {
    assert_eq!(
        str::from_utf8(opening.local_name().into_inner()).unwrap(),
        "id"
    );
    let (mut root, mut extension): (Option<&str>, Option<&str>) = (None, None);

    for attr in opening.attributes() {
        match attr {
            Err(_) => todo!(),
            Ok(attr) => match attr.key.local_name().into_inner() {
                b"root" => {
                    println!("patient_id_root");
                    let val: &str = arena.alloc_str(str::from_utf8(attr.value.deref()).unwrap());
                    assert!(root.is_none());
                    root = Some(val);
                    patient.id_root = Some(val);
                }
                b"extension" => {
                    let val: &str = arena.alloc_str(str::from_utf8(attr.value.deref()).unwrap());
                    assert!(extension.is_none());
                    extension = Some(val);
                    patient.id_extension = Some(val);
                }
                _ => todo!(),
            },
        }
    }

    Ok(())
}

fn p_patient_role__telecom<'r>(
    _ctx: &Context,
    arena: &'r Bump,
    patient: &mut Patient<'r>,
    opening: &BytesStart,
) -> PResult<()> {
    assert_eq!(
        str::from_utf8(opening.local_name().into_inner()).unwrap(),
        "telecom"
    );

    let mut system: Option<TelecomSystem> = None;
    let mut value: Option<&'r str> = None;
    let mut use_pt: Option<ContactPointUse> = None;
    let mut rank: Option<u16> = None;

    for attr in opening.attributes() {
        match attr {
            Err(_) => todo!(),
            Ok(attr) => match attr.key.local_name().into_inner() {
                b"use" => {
                    let val: &str = arena.alloc_str(str::from_utf8(attr.value.deref()).unwrap());
                    assert!(use_pt.is_none());
                    use_pt = Some(match val {
                        "MC" => ContactPointUse::Mobile,
                        "HP" => ContactPointUse::Home,
                        _ => todo!("We need to handle nonstandard values"),
                    });
                }
                b"value" => {
                    let val: &str = arena.alloc_str(str::from_utf8(attr.value.deref()).unwrap());
                    assert!(value.is_none() && system.is_none());

                    // TODO: More robust
                    let parts: Vec<&str> = val.split(":").take(2).collect();
                    system = Some(match parts[0] {
                        "tel" => TelecomSystem::Phone,
                        "mailto" => TelecomSystem::Email,
                        _ => todo!(),
                    });
                    value = Some(parts[1])
                }
                t => todo!("{:?}", t),
            },
        }
    }

    let Some(system) = system else {
        todo!("Spec says this has to be here")
    };
    let Some(value) = value else {
        todo!("Spec says this has to be here")
    };

    let result = TeleContactPoint {
        system,
        value,
        use_pt,
        rank,
    };

    patient.telecontacts.push(result);

    Ok(())
}

fn p_patient__admin_gender_code<'r>(
    _ctx: &Context,
    arena: &'r Bump,
    patient: &mut Patient<'r>,
    opening: &BytesStart,
) -> PResult<()> {
    assert_eq!(
        str::from_utf8(opening.local_name().into_inner()).unwrap(),
        "administrativeGenderCode"
    );

    let mut admin_gender: Option<AdministrativeGender> = None;

    for attr in opening.attributes() {
        match attr {
            Err(_) => todo!(),
            Ok(attr) => match attr.key.local_name().into_inner() {
                b"code" => {
                    let val: &str = arena.alloc_str(str::from_utf8(attr.value.deref()).unwrap());
                    assert!(admin_gender.is_none());

                    // TODO: Handle coding systems for genders?
                    // TODO: More robust
                    admin_gender = Some(match val {
                        "M" => AdministrativeGender::Male,
                        "F" => AdministrativeGender::Female,
                        _ => todo!("Handle other gender codes"),
                    })
                }
                _ => {} //TODO: Implement reading the gender system coding systems?
            },
        }
    }

    patient.administrativeGender = admin_gender;

    Ok(())
}
