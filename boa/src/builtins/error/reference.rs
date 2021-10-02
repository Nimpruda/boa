//! This module implements the global `ReferenceError` object.
//!
//! Indicates an error that occurs when de-referencing an invalid reference
//!
//! More information:
//!  - [MDN documentation][mdn]
//!  - [ECMAScript reference][spec]
//!
//! [spec]: https://tc39.es/ecma262/#sec-native-error-types-used-in-this-standard-referenceerror
//! [mdn]: https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/ReferenceError

use crate::{
    builtins::BuiltIn,
    context::StandardObjects,
    object::{internal_methods::get_prototype_from_constructor, ConstructorBuilder, ObjectData},
    profiler::BoaProfiler,
    property::Attribute,
    Context, JsResult, JsValue,
};

#[derive(Debug, Clone, Copy)]
pub(crate) struct ReferenceError;

impl BuiltIn for ReferenceError {
    const NAME: &'static str = "ReferenceError";

    const ATTRIBUTE: Attribute = Attribute::WRITABLE
        .union(Attribute::NON_ENUMERABLE)
        .union(Attribute::CONFIGURABLE);

    fn init(context: &mut Context) -> JsValue {
        let _timer = BoaProfiler::global().start_event(Self::NAME, "init");

        let error_prototype = context.standard_objects().error_object().prototype();
        let attribute = Attribute::WRITABLE | Attribute::NON_ENUMERABLE | Attribute::CONFIGURABLE;
        let reference_error_object = ConstructorBuilder::with_standard_object(
            context,
            Self::constructor,
            context.standard_objects().reference_error_object().clone(),
        )
        .name(Self::NAME)
        .length(Self::LENGTH)
        .inherit(error_prototype.into())
        .property("name", Self::NAME, attribute)
        .property("message", "", attribute)
        .build();

        reference_error_object.into()
    }
}

impl ReferenceError {
    /// The amount of arguments this function object takes.
    pub(crate) const LENGTH: usize = 1;

    /// Create a new error object.
    pub(crate) fn constructor(
        new_target: &JsValue,
        args: &[JsValue],
        context: &mut Context,
    ) -> JsResult<JsValue> {
        let prototype =
            get_prototype_from_constructor(new_target, StandardObjects::error_object, context)?;
        let obj = context.construct_object();
        obj.set_prototype_instance(prototype.into());
        let this = JsValue::new(obj);
        if let Some(message) = args.get(0) {
            if !message.is_undefined() {
                this.set_field("message", message.to_string(context)?, false, context)?;
            }
        }

        // This value is used by console.log and other routines to match Object type
        // to its Javascript Identifier (global constructor method name)
        this.set_data(ObjectData::error());
        Ok(this)
    }
}
