use std::collections::HashMap;

use serde_json::{json, Value};

use crate::core::convert::StmtConvert;
use crate::core::db::DriverType;

use crate::ast::ast::RbatisAST;
use crate::ast::node::bind_node::BindNode;
use crate::ast::node::choose_node::ChooseNode;
use crate::ast::node::foreach_node::ForEachNode;
use crate::ast::node::if_node::IfNode;
use crate::ast::node::node_type::NodeType::NWhen;
use crate::ast::node::otherwise_node::OtherwiseNode;
use crate::ast::node::set_node::SetNode;
use crate::ast::node::string_node::StringNode;
use crate::ast::node::trim_node::TrimNode;
use crate::ast::node::when_node::WhenNode;
use crate::ast::node::where_node::WhereNode;
use crate::engine::runtime::RbatisEngine;

use super::node_type::NodeType;

pub trait SqlNodePrint {
    fn print(&self, deep: i32) -> String;
}


//执行子所有节点
pub fn do_child_nodes(convert: &crate::core::db::DriverType, child_nodes: &Vec<NodeType>, env: &mut Value, engine: &RbatisEngine, arg_array: &mut Vec<Value>) -> Result<String, crate::core::Error> {
    let mut s = String::new();
    for item in child_nodes {
        let item_result = item.eval(convert, env, engine, arg_array)?;
        s = s + item_result.as_str();
    }
    return Result::Ok(s);
}


pub fn print_child(arg: &Vec<impl SqlNodePrint>, deep: i32) -> String {
    let mut result = String::new();
    for x in arg {
        let item = x.print(deep);
        result = result + "" + item.as_str();
    }
    return result;
}

pub fn create_deep(deep: i32) -> String {
    let mut s = "\n".to_string();
    for index in 0..deep {
        s = s + "  ";
    }
    return s;
}


#[test]
fn test_string_node() {
    let mut engine = RbatisEngine::new();
    let mut john = json!({
        "name": "John Doe",
    });
    let str_node = NodeType::NString(StringNode::new("select * from ${name} where name = #{name}"));
    let mut arg_array = vec![];

    let result = str_node.eval(&DriverType::Mysql, &mut john, &mut engine, &mut arg_array).unwrap();
    println!("{}", result);
}