use mysql::PooledConn;

use crate::helper::connection::establish_connection_v2;
use crate::repository::insulin_repository::{
    create_blood_sugar_log_table, create_insulin_assign_table, create_insulin_item_table,
    create_insulin_usage_table,
};

pub fn init_create_table_v2() {
    let mut conn: PooledConn = establish_connection_v2().expect("Failed to connect to database");
    create_insulin_assign_table(&mut conn);
    create_insulin_item_table(&mut conn);
    create_insulin_usage_table(&mut conn);
    create_blood_sugar_log_table(&mut conn);
}
