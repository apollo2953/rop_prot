use redb::{Database, Error, ReadableDatabase, ReadableTable, TableDefinition, Value};

const USER_TABLE: TableDefinition<String, String> = TableDefinition::new("user");
// ^
// | we initialize the table here
fn write_to_user(name: String, value: serde_json::Value) -> Result<(), Error> {
    let db = Database::create("user.redb")?; // <- creates the database, if it doesn't exist 
    let write_txn = db.begin_write()?; // <- begins the write operation
    {
        let mut table = write_txn.open_table(USER_TABLE)?; // <- writes in the defined table format
        let val = value.to_string();
        table.insert(name, val)?;
    }
    write_txn.commit()?; // <- commits.
    Ok(())
}

fn read_user_val(name: String) -> Result<serde_json::Value, Error> {
    let db = Database::open("user.redb")?;
    let read_txn = db.begin_read()?; // <- begins the write operation
    let table = read_txn.open_table(USER_TABLE)?; // <- opens it in the pre-defined table format
    let json: serde_json::Value = serde_json::from_str(
        table
            .get(name)? // <- finds the key, inputted as the String
            .unwrap() // <- gets rid of the access guard
            .value() // <- gets the value
            .as_str(),
    )
    .unwrap(); // <- we turn the value back into a json for ease of use
    // TODO: change unwrap to an error safe option
    Ok(json)
}
