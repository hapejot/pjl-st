use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use crate::memory::{SmalltalkClass, Value};

use super::{
    VirtualMachine, VirtualMachineState,
    primitives::{
        execution_value0, execution_value1, execution_value2, integer_add, integer_bit_and,
        integer_bit_shift, integer_div, integer_eq, integer_ge, integer_gt, integer_lt,
        integer_mul, integer_sub, meta_class_new,
    },
};

impl VirtualMachine {
    pub fn new() -> Self {
        Self::try_new().expect("Failed to initialize virtual machine")
    }

    pub fn try_new() -> Result<Self, String> {
        let object_class = SmalltalkClass::new("Object", None, vec![])?;
        let meta_class = SmalltalkClass::new(
            "Behavior",
            Some(object_class.clone()),
            vec![
                "superClass",
                "methodDictionary",
                "instanceSpec",
                "subClasses",
                "instanceVariables",
            ],
        )?;
        let class_description =
            SmalltalkClass::new("ClassDescription", Some(meta_class.clone()), vec![])?;
        let class_class = SmalltalkClass::new(
            "Class",
            Some(class_description.clone()),
            vec![
                "name",
                "comment",
                "category",
                "environment",
                "classVariables",
                "sharedPools",
                "pragmaHandlers",
            ],
        )?;
        object_class.set_meta(class_class.clone())?;

        let magnitude_class = SmalltalkClass::new("Magnitude", Some(object_class.clone()), vec![])?;
        let number_class = SmalltalkClass::new("Number", Some(magnitude_class.clone()), vec![])?;
        let integer_class = SmalltalkClass::new("Integer", Some(number_class.clone()), vec![])?;
        let float_class = SmalltalkClass::new("Float", Some(number_class.clone()), vec![])?;
        let execution_class = SmalltalkClass::new("Execution", Some(object_class.clone()), vec![])?;

        let nil_class = SmalltalkClass::new("Nil", Some(object_class.clone()), vec![])?;
        let undefined_class =
            SmalltalkClass::new("UndefinedObject", Some(object_class.clone()), vec![])?;

        integer_class.insert_method("+", Value::NativeMethod(integer_add))?;
        integer_class.insert_method("-", Value::NativeMethod(integer_sub))?;
        integer_class.insert_method("*", Value::NativeMethod(integer_mul))?;
        integer_class.insert_method("/", Value::NativeMethod(integer_div))?;
        integer_class.insert_method(">", Value::NativeMethod(integer_gt))?;
        integer_class.insert_method("<", Value::NativeMethod(integer_lt))?;
        integer_class.insert_method(">=", Value::NativeMethod(integer_ge))?;
        integer_class.insert_method("bitAnd:", Value::NativeMethod(integer_bit_and))?;
        integer_class.insert_method("bitShift:", Value::NativeMethod(integer_bit_shift))?;
        integer_class.insert_method("=", Value::NativeMethod(integer_eq))?;
        execution_class.insert_method("value", Value::NativeMethod(execution_value0))?;
        execution_class.insert_method("value:", Value::NativeMethod(execution_value1))?;
        execution_class.insert_method("value:value:", Value::NativeMethod(execution_value2))?;
        // integer_methods.insert("-", Value::Method(Arc::new(CompiledMethod::default())));
        // integer_methods.insert("*", Value::Method(Arc::new(CompiledMethod::default())));
        // integer_methods.insert("/", Value::Method(Arc::new(CompiledMethod::default())));
        // -------------------------------------------------------
        // initialize smalltalk dictionary

        let mut st = HashMap::new();

        st.insert("Object", object_class.clone().into());
        st.insert("Behavior", meta_class.clone().into());
        st.insert("ClassDescription", class_description.clone().into());
        st.insert("Class", class_class.clone().into());

        st.insert("Magnitude", magnitude_class.clone().into());
        st.insert("Integer", integer_class.clone().into());
        st.insert("Number", number_class.clone().into());
        st.insert("Execution", execution_class.clone().into());
        st.insert("Nil", nil_class.clone().into());
        st.insert("nil", Value::Nil);
        st.insert("UndefinedObject", undefined_class.clone().into());
        meta_class.insert_method("basicNew", Value::NativeMethod(meta_class_new))?;
        // object_class.set_meta(meta_class.clone());
        st.insert("Behavior", meta_class.clone().into());

        st.insert("true", Value::Boolean(true));
        st.insert("false", Value::Boolean(false));

        Ok(Self {
            state: Arc::new(VirtualMachineState {
                integer_class: integer_class.into(),
                float_class: float_class.into(),
                meta_class: meta_class.into(),
                execution_class: execution_class.into(),
                st: Value::Dictionary(Arc::new(Mutex::new(st))),
                debugger: Mutex::new(None),
            }),
        })
    }
}
