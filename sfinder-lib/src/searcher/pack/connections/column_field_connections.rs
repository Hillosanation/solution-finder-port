use super::column_field_connection::ColumnFieldConnection;

pub trait ColumnFieldConnections {
    fn get_connection_stream(&self) -> Box<dyn Iterator<Item = ColumnFieldConnection>>;
}
