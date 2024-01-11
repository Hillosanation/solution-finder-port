use crate::searcher::pack::mino_field::mino_field::MinoField;

pub trait MinoFields {
    // TODO: Is boxed iterator necessary, compared to an generic associated type?
    // TODO: Is boxed MinoField necessary, or is substituting it with RecursiveMinoField possible?
    fn stream(&self) -> Box<dyn Iterator<Item = Box<dyn MinoField + '_>> + '_>;
}
