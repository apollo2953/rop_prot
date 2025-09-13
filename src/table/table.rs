use crate::menu::menu::MenuItem;

pub struct Table {
    is_flexible: bool,
    client_limit: u8,
    table_id: u8,
    is_open: bool,
    receipt: Vec<MenuItem>,
}
impl Table {
    pub fn create_table(is_flexible: bool, client_limit: u8, table_id: u8) -> Self {
        Self {
            is_flexible,
            client_limit,
            table_id,
            is_open: false,
            receipt: vec![],
        }
    }
    pub fn open_table(mut self) {
        self.is_open = true;
    }
    pub fn close_table(mut self) {
        if !self.is_open {
            self.is_open = false;
        }
    }
}
