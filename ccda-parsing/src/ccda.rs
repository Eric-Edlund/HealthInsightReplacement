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
use std::cell::RefCell;
use std::ops::Deref;

#[derive(Default, Debug, Eq, PartialEq)]
pub enum Sex {
    #[default]
    Unspecified,
    Male,
    Female,
}

#[derive(Default, Debug, Eq, PartialEq)]
pub struct Patient<'a> {
    pub id_root: Option<&'a str>,
    pub id_extension: Option<&'a str>,

    pub sex: Sex,
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
    // path: Vec<&'src str>,
}

// impl<'src> fmt::Debug for Context<'src> {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         // f.write_str(&" ".repeat(self.path.len()))?;
//         f.debug_struct("Path").field("path", &self.path).finish()
//     }
// }

impl<'src> Context<'src> {
    fn skip_rec(&self) {
        let mut depth = 1;
        let Some(tag) = self.last_given.borrow().clone() else {
            todo!()
        };
        let Event::Start(tag) = tag else { todo!() };
        let start_tag = str::from_utf8(tag.local_name().into_inner()).unwrap();
        println!("Skipping rec: {}", start_tag);
        while depth > 0 {
            let Ok(ev) = self.reader.borrow_mut().read_event() else {
                todo!()
            };
            match ev {
                Event::Start(ref t) => {
                    println!("<{:?}>", t);
                    depth += 1;
                }
                Event::End(ref t) => {
                    depth -= 1;
                    println!("</{:?}> -> {}", t, depth);
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
                    println!("Pulled {:?}", ev);
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
    start_bytes: &BytesStart,
) -> PResult<ClinicalDocument<'r>> {
    let mut record_targets = Vec::<Patient<'r>>::new();

    for ev in ctx {
        println!("Clinical Doc: {:?}", ev);
        match ev {
            Event::Start(open_tag) => {
                println!("{:?}", open_tag);
                match open_tag.local_name().into_inner() {
                    b"recordTarget" => {
                        let patient = p_record_target(ctx, arena, &open_tag)?;
                        record_targets.push(patient);
                    }
                    t => {
                        println!("Skipping {:?}", t);
                        ctx.skip_rec();
                    } // t => todo!("{:?}", str::from_utf8(t).unwrap()),
                };
            }
            Event::Empty(tag) => {
                match tag.local_name().into_inner() {
                    b"recordTarget" => {
                        let patient = p_record_target(ctx, arena, &tag)?;
                        record_targets.push(patient);
                    }
                    _ => {} // t => todo!("{:?}", str::from_utf8(t).unwrap()),
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
    start_bytes: &BytesStart,
) -> PResult<Patient<'r>> {
    assert_eq!(
        start_bytes.name().local_name().into_inner(),
        b"recordTarget"
    );
    assert!(start_bytes.attributes().next().is_none());

    let mut patient_role: Option<Patient> = None;

    for ev in ctx {
        use Event::*;
        match ev {
            Start(bytes_start) => {
                match bytes_start.name().local_name().into_inner() {
                    b"patientRole" => {
                        assert!(patient_role.is_none());
                        patient_role = Some(p_patient_role(ctx, arena, start_bytes)?);
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

    let Some(patient) = patient_role else { todo!() };

    Ok(patient)
}

/// The source string lives 'a
/// the saved content in the bump allocator lives 'b
fn p_patient_role<'src, 'r>(
    ctx: &Context<'src>,
    arena: &'r Bump,
    start_bytes: &BytesStart,
) -> PResult<Patient<'r>> {
    let mut seen_patient_role = false;
    let mut patient = Patient::default();

    // ctx.consume_using(C {
    //     start: |tag| {
    //         match tag.name().local_name().into_inner() {
    //             b"id" => {
    //                 assert!(!seen_patient_role);
    //                 seen_patient_role = true;
    //                 p_patient__id(ctx, arena, &mut patient, start_bytes)?;
    //                 Ok()
    //             }
    //             _ => None,
    //         };
    //     },
    // });
    //
    for ev in ctx {
        use Event::*;
        match ev {
            Start(bytes_start) => {
                match bytes_start.name().local_name().into_inner() {
                    b"id" => {
                        assert!(!seen_patient_role);
                        seen_patient_role = true;
                        p_patient__id(ctx, arena, &mut patient, start_bytes)?;
                    }
                    _ => ctx.skip_rec(),
                };
            }
            Empty(bytes_start) => {
                match bytes_start.name().local_name().into_inner() {
                    b"id" => {
                        assert!(!seen_patient_role);
                        seen_patient_role = true;
                        p_patient__id(ctx, arena, &mut patient, start_bytes)?;
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

    assert!(seen_patient_role);
    Ok(patient)
}

fn p_patient__id<'src, 'r>(
    _ctx: &Context<'src>,
    arena: &'r Bump,
    patient: &mut Patient<'r>,
    start_bytes: &BytesStart,
) -> PResult<()> {
    let (mut root, mut extension) = (None, None);

    for attr in start_bytes.attributes() {
        match attr {
            Err(_) => todo!(),
            Ok(attr) => {
                let val: &str = arena.alloc_str(str::from_utf8(attr.value.deref()).unwrap());
                match attr.key.local_name().into_inner() {
                    b"root" => {
                        assert!(root.is_none());
                        root = Some(val);
                        patient.id_root = Some(val);
                    }
                    b"extension" => {
                        assert!(extension.is_none());
                        extension = Some(val);
                        patient.id_extension = Some(val);
                    }
                    _ => todo!(),
                }
            }
        }
    }

    Ok(())
}
