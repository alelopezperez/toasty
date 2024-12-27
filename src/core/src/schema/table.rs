mod builder;

mod pk;
pub use pk::TablePrimaryKey;

use super::*;

use std::fmt;

/// A database table
#[derive(Debug)]
pub struct Table {
    /// Uniquely identifies a table
    pub id: TableId,

    /// Name of the table
    pub name: String,

    /// The table's columns
    pub columns: Vec<Column>,

    pub primary_key: TablePrimaryKey,

    pub indices: Vec<Index>,
}

/// Uniquely identifies a table
#[derive(PartialEq, Eq, Clone, Copy, Hash)]
pub struct TableId(pub usize);

impl Table {
    pub fn primary_key_column(&self, i: usize) -> &Column {
        &self.columns[self.primary_key.columns[i].index]
    }

    pub fn primary_key_columns(&self) -> impl ExactSizeIterator<Item = &Column> + '_ {
        self.primary_key
            .columns
            .iter()
            .map(|column_id| &self.columns[column_id.index])
    }

    pub fn column(&self, id: impl Into<ColumnId>) -> &Column {
        &self.columns[id.into().index]
    }

    /// The path must have exactly one step
    pub fn resolve(&self, projection: &stmt::Projection) -> &Column {
        let [first, rest @ ..] = projection.as_slice() else {
            panic!("need at most one path step")
        };
        assert!(rest.is_empty());

        &self.columns[*first]
    }

    pub(crate) fn from_ast(ctx: &mut schema::Context, ast: &ast::Table) -> crate::Result<Table> {
        let name = ast.ident.to_string();
        let id = ctx.register_table(&name);
        Ok(Table::new(id, name))
    }

    fn new(id: TableId, name: String) -> Table {
        Table {
            id,
            name,
            columns: vec![],
            primary_key: TablePrimaryKey {
                columns: vec![],
                index: IndexId {
                    table: id,
                    index: 0,
                },
            },
            indices: vec![Index {
                id: IndexId {
                    table: id,
                    index: 0,
                },
                name: String::new(),
                on: id,
                columns: vec![],
                unique: true,
                primary_key: true,
            }],
        }
    }

    pub(crate) fn from_model(ctx: &mut schema::Context, model: &mut Model) -> crate::Result<Table> {
        let name = name_from_model(&model.name);
        let id = ctx.register_table(&name);

        // Update the model to point to this table
        model.lowering.table = id;

        let mut table = Table::new(id, name);

        table.lower_models(&mut [model]);

        Ok(table)
    }
}

impl TableId {
    pub(crate) fn placeholder() -> TableId {
        TableId(usize::MAX)
    }
}

impl fmt::Debug for TableId {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(fmt, "TableId({})", self.0)
    }
}

fn name_from_model(model_name: &Name) -> String {
    std_util::str::pluralize(&model_name.snake_case())
}
