//! Methods to enforce constraints on integers in a resolved Leo program.

use crate::{
    constraints::{ConstrainedProgram, ConstrainedValue},
    errors::IntegerError,
    types::{InputModel, InputValue, Integer, Type},
    IntegerType,
};

use snarkos_models::{
    curves::{Group, Field, PrimeField},
    gadgets::{r1cs::ConstraintSystem, utilities::boolean::Boolean},
};

impl<G: Group, F: Field + PrimeField, CS: ConstraintSystem<G>> ConstrainedProgram<G, F, CS> {
    pub(crate) fn get_integer_constant(integer: Integer) -> ConstrainedValue<G, F> {
        ConstrainedValue::Integer(integer)
    }

    pub(crate) fn evaluate_integer_eq(
        left: Integer,
        right: Integer,
    ) -> Result<ConstrainedValue<G, F>, IntegerError> {
        Ok(ConstrainedValue::Boolean(Boolean::Constant(
            match (left, right) {
                (Integer::U8(left_u8), Integer::U8(right_u8)) => left_u8.eq(&right_u8),
                (Integer::U16(left_u16), Integer::U16(right_u16)) => left_u16.eq(&right_u16),
                (Integer::U32(left_u32), Integer::U32(right_u32)) => left_u32.eq(&right_u32),
                (Integer::U64(left_u64), Integer::U64(right_u64)) => left_u64.eq(&right_u64),
                (Integer::U128(left_u128), Integer::U128(right_u128)) => left_u128.eq(&right_u128),
                (left, right) => {
                    return Err(IntegerError::CannotEvaluate(format!(
                        "{} == {}",
                        left, right
                    )))
                }
            },
        )))
    }

    pub(crate) fn integer_from_parameter(
        &mut self,
        cs: &mut CS,
        integer_model: InputModel<G, F>,
        integer_value: Option<InputValue<G, F>>,
    ) -> Result<ConstrainedValue<G, F>, IntegerError> {
        let integer_type = match &integer_model._type {
            Type::IntegerType(integer_type) => integer_type,
            _type => return Err(IntegerError::InvalidType(_type.to_string())),
        };

        // Check that the parameter value is the correct type
        let integer_option = match integer_value {
            Some(parameter) => {
                if let InputValue::Integer(integer) = parameter {
                    Some(integer)
                } else {
                    return Err(IntegerError::InvalidInteger(
                        integer_model._type.to_string(),
                        parameter.to_string(),
                    ));
                }
            }
            None => None,
        };

        match integer_type {
            IntegerType::U8 => self.u8_from_input(cs, integer_model, integer_option),
            IntegerType::U16 => self.u16_from_input(cs, integer_model, integer_option),
            IntegerType::U32 => self.u32_from_input(cs, integer_model, integer_option),
            IntegerType::U64 => self.u64_from_input(cs, integer_model, integer_option),
            IntegerType::U128 => self.u128_from_integer(cs, integer_model, integer_option),
        }
    }

    pub(crate) fn enforce_integer_eq(
        cs: &mut CS,
        left: Integer,
        right: Integer,
    ) -> Result<(), IntegerError> {
        match (left, right) {
            (Integer::U8(left_u8), Integer::U8(right_u8)) => {
                Self::enforce_u8_eq(cs, left_u8, right_u8)
            }
            (Integer::U16(left_u16), Integer::U16(right_u16)) => {
                Self::enforce_u16_eq(cs, left_u16, right_u16)
            }
            (Integer::U32(left_u32), Integer::U32(right_u32)) => {
                Self::enforce_u32_eq(cs, left_u32, right_u32)
            }
            (Integer::U64(left_u64), Integer::U64(right_u64)) => {
                Self::enforce_u64_eq(cs, left_u64, right_u64)
            }
            (Integer::U128(left_u128), Integer::U128(right_u128)) => {
                Self::enforce_u128_eq(cs, left_u128, right_u128)
            }
            (left, right) => {
                return Err(IntegerError::CannotEnforce(format!(
                    "{} == {}",
                    left, right
                )))
            }
        }
    }

    pub(crate) fn enforce_integer_add(
        cs: &mut CS,
        left: Integer,
        right: Integer,
    ) -> Result<ConstrainedValue<G, F>, IntegerError> {
        Ok(ConstrainedValue::Integer(match (left, right) {
            (Integer::U8(left_u8), Integer::U8(right_u8)) => {
                Integer::U8(Self::enforce_u8_add(cs, left_u8, right_u8)?)
            }
            (Integer::U16(left_u16), Integer::U16(right_u16)) => {
                Integer::U16(Self::enforce_u16_add(cs, left_u16, right_u16)?)
            }
            (Integer::U32(left_u32), Integer::U32(right_u32)) => {
                Integer::U32(Self::enforce_u32_add(cs, left_u32, right_u32)?)
            }
            (Integer::U64(left_u64), Integer::U64(right_u64)) => {
                Integer::U64(Self::enforce_u64_add(cs, left_u64, right_u64)?)
            }
            (Integer::U128(left_u128), Integer::U128(right_u128)) => {
                Integer::U128(Self::enforce_u128_add(cs, left_u128, right_u128)?)
            }
            (left, right) => {
                return Err(IntegerError::CannotEnforce(format!("{} + {}", left, right)))
            }
        }))
    }
    pub(crate) fn enforce_integer_sub(
        cs: &mut CS,
        left: Integer,
        right: Integer,
    ) -> Result<ConstrainedValue<G, F>, IntegerError> {
        Ok(ConstrainedValue::Integer(match (left, right) {
            (Integer::U8(left_u8), Integer::U8(right_u8)) => {
                Integer::U8(Self::enforce_u8_sub(cs, left_u8, right_u8)?)
            }
            (Integer::U16(left_u16), Integer::U16(right_u16)) => {
                Integer::U16(Self::enforce_u16_sub(cs, left_u16, right_u16)?)
            }
            (Integer::U32(left_u32), Integer::U32(right_u32)) => {
                Integer::U32(Self::enforce_u32_sub(cs, left_u32, right_u32)?)
            }
            (Integer::U64(left_u64), Integer::U64(right_u64)) => {
                Integer::U64(Self::enforce_u64_sub(cs, left_u64, right_u64)?)
            }
            (Integer::U128(left_u128), Integer::U128(right_u128)) => {
                Integer::U128(Self::enforce_u128_sub(cs, left_u128, right_u128)?)
            }
            (left, right) => {
                return Err(IntegerError::CannotEnforce(format!("{} - {}", left, right)))
            }
        }))
    }
    pub(crate) fn enforce_integer_mul(
        cs: &mut CS,
        left: Integer,
        right: Integer,
    ) -> Result<ConstrainedValue<G, F>, IntegerError> {
        Ok(ConstrainedValue::Integer(match (left, right) {
            (Integer::U8(left_u8), Integer::U8(right_u8)) => {
                Integer::U8(Self::enforce_u8_mul(cs, left_u8, right_u8)?)
            }
            (Integer::U16(left_u16), Integer::U16(right_u16)) => {
                Integer::U16(Self::enforce_u16_mul(cs, left_u16, right_u16)?)
            }
            (Integer::U32(left_u32), Integer::U32(right_u32)) => {
                Integer::U32(Self::enforce_u32_mul(cs, left_u32, right_u32)?)
            }
            (Integer::U64(left_u64), Integer::U64(right_u64)) => {
                Integer::U64(Self::enforce_u64_mul(cs, left_u64, right_u64)?)
            }
            (Integer::U128(left_u128), Integer::U128(right_u128)) => {
                Integer::U128(Self::enforce_u128_mul(cs, left_u128, right_u128)?)
            }
            (left, right) => {
                return Err(IntegerError::CannotEnforce(format!("{} * {}", left, right)))
            }
        }))
    }
    pub(crate) fn enforce_integer_div(
        cs: &mut CS,
        left: Integer,
        right: Integer,
    ) -> Result<ConstrainedValue<G, F>, IntegerError> {
        Ok(ConstrainedValue::Integer(match (left, right) {
            (Integer::U8(left_u8), Integer::U8(right_u8)) => {
                Integer::U8(Self::enforce_u8_div(cs, left_u8, right_u8)?)
            }
            (Integer::U16(left_u16), Integer::U16(right_u16)) => {
                Integer::U16(Self::enforce_u16_div(cs, left_u16, right_u16)?)
            }
            (Integer::U32(left_u32), Integer::U32(right_u32)) => {
                Integer::U32(Self::enforce_u32_div(cs, left_u32, right_u32)?)
            }
            (Integer::U64(left_u64), Integer::U64(right_u64)) => {
                Integer::U64(Self::enforce_u64_div(cs, left_u64, right_u64)?)
            }
            (Integer::U128(left_u128), Integer::U128(right_u128)) => {
                Integer::U128(Self::enforce_u128_div(cs, left_u128, right_u128)?)
            }
            (left, right) => {
                return Err(IntegerError::CannotEnforce(format!("{} / {}", left, right)))
            }
        }))
    }
    pub(crate) fn enforce_integer_pow(
        cs: &mut CS,
        left: Integer,
        right: Integer,
    ) -> Result<ConstrainedValue<G, F>, IntegerError> {
        Ok(ConstrainedValue::Integer(match (left, right) {
            (Integer::U8(left_u8), Integer::U8(right_u8)) => {
                Integer::U8(Self::enforce_u8_pow(cs, left_u8, right_u8)?)
            }
            (Integer::U16(left_u16), Integer::U16(right_u16)) => {
                Integer::U16(Self::enforce_u16_pow(cs, left_u16, right_u16)?)
            }
            (Integer::U32(left_u32), Integer::U32(right_u32)) => {
                Integer::U32(Self::enforce_u32_pow(cs, left_u32, right_u32)?)
            }
            (Integer::U64(left_u64), Integer::U64(right_u64)) => {
                Integer::U64(Self::enforce_u64_pow(cs, left_u64, right_u64)?)
            }
            (Integer::U128(left_u128), Integer::U128(right_u128)) => {
                Integer::U128(Self::enforce_u128_pow(cs, left_u128, right_u128)?)
            }
            (left, right) => {
                return Err(IntegerError::CannotEnforce(format!(
                    "{} ** {}",
                    left, right
                )))
            }
        }))
    }
}
