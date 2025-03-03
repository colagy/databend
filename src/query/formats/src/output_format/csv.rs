// Copyright 2022 Datafuse Labs.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use common_datablocks::DataBlock;
use common_datavalues::DataSchemaRef;
use common_datavalues::DataType;
use common_exception::Result;

use crate::field_encoder::write_csv_string;
use crate::field_encoder::FieldEncoderCSV;
use crate::field_encoder::FieldEncoderRowBased;
use crate::output_format::OutputFormat;
use crate::FileFormatOptionsExt;

pub type CSVOutputFormat = CSVOutputFormatBase<false, false>;
pub type CSVWithNamesOutputFormat = CSVOutputFormatBase<true, false>;
pub type CSVWithNamesAndTypesOutputFormat = CSVOutputFormatBase<true, true>;

pub struct CSVOutputFormatBase<const WITH_NAMES: bool, const WITH_TYPES: bool> {
    schema: DataSchemaRef,
    field_encoder: FieldEncoderCSV,
    field_delimiter: u8,
    record_delimiter: Vec<u8>,
    quote: u8,
}

impl<const WITH_NAMES: bool, const WITH_TYPES: bool> CSVOutputFormatBase<WITH_NAMES, WITH_TYPES> {
    pub fn create(schema: DataSchemaRef, options: &FileFormatOptionsExt) -> Self {
        let field_encoder = FieldEncoderCSV::create(options);
        Self {
            schema,
            field_encoder,
            field_delimiter: options.stage.field_delimiter.as_bytes()[0],
            record_delimiter: options.stage.record_delimiter.as_bytes().to_vec(),
            quote: options.quote.as_bytes()[0],
        }
    }

    fn serialize_strings(&self, values: Vec<String>) -> Vec<u8> {
        let mut buf = vec![];
        let fd = self.field_delimiter;

        for (col_index, v) in values.iter().enumerate() {
            if col_index != 0 {
                buf.push(fd);
            }
            write_csv_string(v.as_bytes(), &mut buf, self.quote);
        }

        buf.extend_from_slice(&self.record_delimiter);
        buf
    }
}

impl<const WITH_NAMES: bool, const WITH_TYPES: bool> OutputFormat
    for CSVOutputFormatBase<WITH_NAMES, WITH_TYPES>
{
    fn serialize_block(&mut self, block: &DataBlock) -> Result<Vec<u8>> {
        let rows_size = block.column(0).len();
        let mut buf = Vec::with_capacity(block.memory_size());
        let serializers = block.get_serializers()?;

        let fd = self.field_delimiter;
        let rd = &self.record_delimiter;

        for row_index in 0..rows_size {
            for (col_index, serializer) in serializers.iter().enumerate() {
                if col_index != 0 {
                    buf.push(fd);
                }
                self.field_encoder
                    .write_field(serializer, row_index, &mut buf, false);
            }
            buf.extend_from_slice(rd)
        }
        Ok(buf)
    }

    fn serialize_prefix(&self) -> Result<Vec<u8>> {
        let mut buf = vec![];
        if WITH_NAMES {
            let names = self
                .schema
                .fields()
                .iter()
                .map(|f| f.name().to_string())
                .collect::<Vec<_>>();
            buf.extend_from_slice(&self.serialize_strings(names));
            if WITH_TYPES {
                let types = self
                    .schema
                    .fields()
                    .iter()
                    .map(|f| f.data_type().name())
                    .collect::<Vec<_>>();
                buf.extend_from_slice(&self.serialize_strings(types));
            }
        }
        Ok(buf)
    }
    fn finalize(&mut self) -> Result<Vec<u8>> {
        Ok(vec![])
    }
}
