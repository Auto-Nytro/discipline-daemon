use rusqlite::Row;
use super::*;

pub trait CompoundValueDeserializer {
  type CompoundValue;

  fn deserialize(
    &self, 
    context: &CompoundValueDeserializerContext,
  ) -> Result<Self::CompoundValue, GenericError>;
}

pub struct CompoundValueDeserializerContext<'a>(&'a Row<'a>);

impl<'a> CompoundValueDeserializerContext<'a> {
  fn get_column_value(&self, column: &Field) -> Result<ScalarValue, GenericError> {
    self.0.get_ref(column.path().to_sql_identifier_str())
      .map_err(|error| {
        GenericError::new("retrieving the value of a sqlite column")
          .add_error("sqlite wrapper returned error")
          .add_attachment("column identifier", column.path().to_displayable_string())
          .add_attachment("sqlite error", error.to_string())
      })
      .map(
        ScalarValue::new
      )
  }

  // TODO: rename to deserializable_scalar_field
  pub fn deserializable_scalar<Value>(
    &self, 
    column: &Field,
  ) 
    -> Result<Value, GenericError>
  where 
    Value: FromScalarValue
  {
    self
      .get_column_value(column)
      .and_then(Value::deserialize)
  }

  pub fn deserialize_compound<Deserializer>(
    &self, 
    deserializer: &Deserializer,
  ) -> 
    Result<Deserializer::CompoundValue, GenericError>
  where
    Deserializer: CompoundValueDeserializer
  {
    deserializer.deserialize(self)
  }
}

pub(super) fn deserialize_compound_value<Deserializer>(
  row: &Row, 
  deserializer: &Deserializer,
) -> 
  Result<Deserializer::CompoundValue, GenericError> 
where 
  Deserializer: CompoundValueDeserializer
{
  CompoundValueDeserializerContext(row).deserialize_compound(deserializer)
}
