use crate::sfinder_core::column_field::column_small_field::ColumnSmallField;

pub trait ColumnFieldConnections {
    fn get_connection_stream(&self) -> Box<dyn Iterator<Item = ColumnFieldConnection>>;
}

struct ColumnFieldConnection {
    pub column_field: ColumnSmallField,
    pub inner_field: ColumnSmallField,
    pub outer_field: ColumnSmallField,
}
