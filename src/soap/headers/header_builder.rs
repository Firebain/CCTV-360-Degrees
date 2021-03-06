use crate::xml::EventWriter;

pub trait HeaderBuilder {
    fn build_header(&self, writer: &mut EventWriter);
}
