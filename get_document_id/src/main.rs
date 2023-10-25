#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum ErrorOffice {
    OfficeClose(u32),
    OfficeNotFound(u32),
    OfficeFull(u32),
}

#[derive(PartialEq, Eq, Clone, Copy)]
pub struct OfficeOne {
    pub next_office: Result<OfficeTwo, ErrorOffice>,
}

#[derive(PartialEq, Eq, Clone, Copy)]
pub struct OfficeTwo {
    pub next_office: Result<OfficeThree, ErrorOffice>,
}

#[derive(PartialEq, Eq, Clone, Copy)]
pub struct OfficeThree {
    pub next_office: Result<OfficeFour, ErrorOffice>,
}

#[derive(PartialEq, Eq, Clone, Copy)]
pub struct OfficeFour {
    pub document_id: Result<u32, ErrorOffice>,
}

impl OfficeOne {
    pub fn get_document_id(&self) -> Result<u32, ErrorOffice> {
        self.next_office?.next_office?.next_office?.document_id
    }
}


fn main() {
    let office_ok = OfficeOne {
        next_office: Ok(OfficeTwo {
            next_office: Ok(OfficeThree {
                next_office: Ok(OfficeFour {
                    document_id: Ok(13),
                }),
            }),
        }),
    };
    let office_closed = {
        OfficeOne {
            next_office: Ok(OfficeTwo {
                next_office: Err(ErrorOffice::OfficeClose(23)),
            }),
        }
    };

    match office_ok.get_document_id() {
        Ok(id) => println!("Found a document with id {}", id),
        Err(err) => println!("Error: {:?}", err),
    };
    match office_closed.get_document_id() {
        Ok(id) => println!("Found a document with id {}", id),
        Err(err) => println!("Error: {:?}", err),
    };
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_document_id_ok() {
        let office = OfficeOne {
            next_office: Ok(OfficeTwo {
                next_office: Ok(OfficeThree {
                    next_office: Ok(OfficeFour {
                        document_id: Ok(13),
                    }),
                }),
            }),
        };

        assert_eq!(Ok(13), office.get_document_id());
    }
    #[test]
    fn test_get_document_id_closed() {
        let office = {
            OfficeOne {
                next_office: Ok(OfficeTwo {
                    next_office: Err(ErrorOffice::OfficeClose(2)),
                }),
            }
        };

        assert_eq!(Err(ErrorOffice::OfficeClose(2)), office.get_document_id());
    }
    #[test]
    fn test_get_document_id_not_found() {
        let office = {
            OfficeOne {
                next_office: Ok(OfficeTwo {
                    next_office: Err(ErrorOffice::OfficeNotFound(2)),
                }),
            }
        };

        assert_eq!(
            Err(ErrorOffice::OfficeNotFound(2)),
            office.get_document_id()
        );
    }
    #[test]
    fn test_get_document_id_full() {
        let office = {
            OfficeOne {
                next_office: Ok(OfficeTwo {
                    next_office: Err(ErrorOffice::OfficeFull(2)),
                }),
            }
        };

        assert_eq!(Err(ErrorOffice::OfficeFull(2)), office.get_document_id());
    }
}