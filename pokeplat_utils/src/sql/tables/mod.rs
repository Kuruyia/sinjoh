use anyhow::Result;
use rusqlite::Connection;

mod area_data;
mod area_lights;
mod area_map_props;
mod land_data;
mod map_headers;
mod map_matrices;
mod map_prop_animation_lists;
mod map_prop_material_shapes;

pub(super) trait PopulateSql {
    fn create_sql_tables(&self, conn: &Connection) -> Result<()>;
    fn populate_sql_tables(&self, conn: &mut Connection) -> Result<()>;

    fn create_and_populate_sql_tables(&self, conn: &mut Connection) -> Result<()> {
        self.create_sql_tables(conn)?;
        self.populate_sql_tables(conn)
    }
}
