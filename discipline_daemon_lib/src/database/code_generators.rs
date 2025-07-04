use super::*;

struct MlutiColumnPrimaryKeyConstraint {
  code: String,
}

impl MlutiColumnPrimaryKeyConstraint {
  fn new() -> Self {
    Self {
      code: String::new()
    }
  }

  fn write(&mut self, column_specification: &ColumnSpecification) {
    if self.code.is_empty() {
      self.code.push_str("PRIMARY KEY(");
    } else {
      self.code.push_str(", ");
    }

    self.code.push_str(column_specification.path.as_str());
  }

  fn finish(mut self) -> String {
    self.code.push_str(")");
    self.code
  }
}

pub fn generate_code_define_collection(
  code: &mut String,
  collection_specification: &CollectionSpecification,
) ->
  Result<(), GenericError>
{
  code.push_str("CREATE TABLE IF NOT EXISTS ");
  code.push_str(collection_specification.path.as_str());
  code.push_str(" (");

  let mut multi_column_primary_key_constraint = MlutiColumnPrimaryKeyConstraint::new();
  let mut did_write_a_column_definition = false;

  for column_specification in &collection_specification.collection_item_namespace.columns {
    if did_write_a_column_definition {
      code.push_str(", ");
    }

    code.push_str(column_specification.path.as_str());
    
    match column_specification.column_type {
      ColumnType::Primary => {
        if collection_specification.collection_item_namespace.primary_columns_number > 1 {
          multi_column_primary_key_constraint.write(&column_specification);
        } else {
          code.push_str(" PRIMARY KEY");
        }
      }
      ColumnType::Optional => {
        // noop
      }
      ColumnType::Required => {
        code.push_str(" NOT NULL");
      }
      ColumnType::UniqueOptional => {
        code.push_str(" UNIQUE");
      }
      ColumnType::UniqueRequired => {
        code.push_str(" UNIQUE NOT NULL");
      }
    }

    did_write_a_column_definition = true;
  }

  if collection_specification.collection_item_namespace.primary_columns_number > 0 {
    code.push_str(", ");
    code.push_str(&multi_column_primary_key_constraint.finish());
  }
  
  code.push_str(");");

  Ok(())
}

pub(super) fn generate_code_add_collection_item<Serializer>(
  code: &mut String,
  collection_specification: &CollectionSpecification,
  collection_item_serializer: &Serializer,
  new_collection_item: &Serializer::CompoundType,
) ->
  Result<(), GenericError>
where 
  Serializer: CompoundTypeSerializer
{
  let mut values_clause = String::new();
  serialize_compound_value_into(
    collection_item_serializer, 
    new_collection_item, 
    &mut values_clause
  )?; // TODO: do proper error handling

  code.push_str("INSERT INTO ");
  code.push_str(collection_specification.path.as_str());
  code.push_str(" ");
  code.push_str(&values_clause);
  code.push_str(";");

  Ok(())
}

pub(super) fn generate_code_delete_collection_item(
  code: &mut String,
  collection_specification: &CollectionSpecification,
  collection_item_matcher: &CollectionItemMatcher,
) ->
  Result<(), GenericError>
{
  code.push_str("DELETE FROM ");
  code.push_str(collection_specification.path.as_str());
  match &collection_item_matcher.inner {
    CollectionItemMatcherInner::NoWhereClause => {
      code.push_str(";");
    }
    CollectionItemMatcherInner::WhereClause(where_clause) => {
      code.push_str(" ");
      code.push_str(&where_clause);
      code.push_str(";");
    }
  }

  Ok(())
}

pub(super) fn generate_code_update_collection_item(
  code: &mut String,
  collection_specification: &CollectionSpecification,
  collection_item_matcher: &CollectionItemMatcher,
  collection_item_modifications: &CollectionItemChanges,
) -> 
  Result<(), GenericError>
{
  let Some(set_clause) = collection_item_modifications.finish() else {
    return Ok(());
  };

  code.push_str("UPDATE ");
  code.push_str(collection_specification.path.as_str());
  code.push_str(" ");
  code.push_str(&set_clause);
 
  match &collection_item_matcher.inner {
    CollectionItemMatcherInner::NoWhereClause => {
      code.push_str(";");
    }
    CollectionItemMatcherInner::WhereClause(where_clause) => {
      code.push_str(" ");
      code.push_str(&where_clause);
      code.push_str(";");
    }
  }

  Ok(())
}

pub(super) fn generate_code_find_all_collection_items(
  code: &mut String,
  collection_specification: &CollectionSpecification,
) -> 
  Result<(), GenericError>
{
  code.push_str("SELECT * FROM ");
  code.push_str(collection_specification.path.as_str());
  code.push_str(";");
  Ok(())
}

pub(super) fn generate_code_find_one_collection_item(
  code: &mut String,
  collection_specification: &CollectionSpecification,
  collection_item_matcher: &CollectionItemMatcher,
) -> 
  Result<(), GenericError>
{
  code.push_str("SELECT * FROM ");
  code.push_str(collection_specification.path.as_str());

  match &collection_item_matcher.inner {
    CollectionItemMatcherInner::NoWhereClause => {
      code.push_str(";");
    }
    CollectionItemMatcherInner::WhereClause(where_clause) => {
      code.push_str(" ");
      code.push_str(where_clause);
      code.push_str(";");
    }
  }

  Ok(())
}



// pub(super) fn generate_code_define_database_schema(
//   code: &mut String,
//   database_specifications_provider: &impl DatabaseSpecificationsProvider,
// ) ->
//   Result<(), GenericError>
// {
//   // TODO: Retrun an error if the providers adds zero collection specifications
//   let mut context = DatabaseSpecificationsProviderContext::new(code);
//   database_specifications_provider
//     .add_specifications(&mut context)
//     .map_err(|error| error.change_context("generate sql code that initializes the database schema, which are tables, triggers and views"))
// }