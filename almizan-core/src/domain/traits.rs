#[allow(dead_code)]
pub trait Attributable {
    fn source(&self) -> String;
}

#[allow(dead_code)]
pub trait DigitalProvenance {
    fn digitized_by(&self) -> String;
    fn signature(&self) -> String;
}
