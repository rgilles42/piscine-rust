use get_document_id::*;

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
