use serde_json::Value;

use crate::core::convert::StmtConvert;

use crate::engine::runtime::RbatisEngine;

/// Abstract syntax tree node
pub trait RbatisAST: Send + Sync {
    fn eval(&self, convert: &crate::core::db::DriverType, env: &mut Value, engine: &RbatisEngine, arg_result: &mut Vec<Value>) -> Result<String, crate::core::Error>;
}