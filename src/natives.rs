use liquid::model::Scalar;
use log::error;
use samp::error::AmxError;
use samp::error::AmxResult;
use samp::native;
use samp::prelude::*;

use crate::internals::insert_template;
use crate::internals::ArgumentPairType;

impl super::PawnTemplates {
    // native Template:CreateTemplate(const template[]);
    #[native(name = "CreateTemplate")]
    pub fn create_template(&mut self, _amx: &Amx, template: AmxString) -> AmxResult<i32> {
        let parser = liquid::ParserBuilder::with_stdlib().build().unwrap();

        let t = match parser.parse(&template.to_string()) {
            Ok(v) => v,
            Err(e) => {
                error!("{}", e);
                return Ok(0);
            }
        };

        let index = insert_template(self, t);
        Ok(index.as_cell())
    }

    // native RenderTemplate(Template:id, dest[], len, ...);
    #[native(raw, name = "RenderTemplate")]
    pub fn render_template(&mut self, _amx: &Amx, mut args: samp::args::Args) -> AmxResult<bool> {
        let varargc = args.count() - 3;
        let pairs = match varargc == 0 || varargc % 3 == 0 {
            true => varargc / 3,
            false => {
                error!("Invalid number of arguments passed to RenderTemplate. Expected a multiple of 3, got {}", varargc);
                return Ok(false);
            }
        };

        let template_id: i32 = args.next::<i32>().ok_or(AmxError::Params)?;
        let dest = args.next::<UnsizedBuffer>().ok_or(AmxError::Params)?;
        let size = args.next::<i32>().ok_or(AmxError::Params)? as usize;

        let template = match self.pool.get(template_id as usize - 1) {
            Some(t) => t,
            None => {
                error!("Template with id {} not found in pool", template_id);
                return Ok(false);
            }
        };

        let mut variables = liquid::model::Object::new();

        for _ in 0..pairs {
            let pair_type_ref = args.next::<Ref<i32>>().ok_or(AmxError::Params)?;
            let pair_type = *pair_type_ref;

            let pair_name = args
                .next::<AmxString>()
                .ok_or(AmxError::Params)?
                .to_string();

            match ArgumentPairType::from_i32(pair_type) {
                ArgumentPairType::String => {
                    let value = args
                        .next::<AmxString>()
                        .ok_or(AmxError::Params)?
                        .to_string();

                    variables.insert(
                        pair_name.into(),
                        liquid::model::Value::Scalar(Scalar::new(value)),
                    )
                }
                ArgumentPairType::Int => {
                    let value_ref = args.next::<Ref<i32>>().ok_or(AmxError::Params)?;
                    let value = *value_ref;

                    variables.insert(
                        pair_name.into(),
                        liquid::model::Value::Scalar(Scalar::new(value)),
                    )
                }
                ArgumentPairType::Float => {
                    let value_ref = args.next::<Ref<f32>>().ok_or(AmxError::Params)?;
                    let value = *value_ref;

                    variables.insert(
                        pair_name.into(),
                        liquid::model::Value::Scalar(Scalar::new(value)),
                    )
                }
                _ => {
                    return Ok(false);
                }
            };
        }

        let output = match template.render(&variables) {
            Ok(v) => v,
            Err(e) => {
                error!("{}", e);
                return Ok(false);
            }
        };

        let output_bytes = output.as_bytes();
        if output_bytes.len() >= size {
            error!(
                "rendered output too large for buffer. Output size: {}, Buffer size: {}",
                output_bytes.len(),
                size
            );
            return Ok(false);
        }

        let mut dest = dest.into_sized_buffer(size + 1);
        let err = samp::cell::string::put_in_buffer(&mut dest, &output);

        Ok(if err.is_ok() { true } else { false })
    }

    // native DeleteTemplate(Template:id);
    #[native(name = "DeleteTemplate")]
    pub fn delete_template(&mut self, _amx: &Amx, template_id: i32) -> AmxResult<bool> {
        match self.pool.get(template_id as usize - 1) {
            Some(_t) => {
                self.pool.remove(template_id as usize - 1);
                Ok(true)
            }
            None => return Ok(false),
        }
    }
}
