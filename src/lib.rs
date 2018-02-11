//! Generate lttng-ust tracepoints for rust code

extern crate bindgen;
extern crate cc;

pub mod generator;

pub use generator::Generator;

// use std::io::{Error, ErrorKind};

/// A tracepoint provider.
/// You usually only need to create one of these
pub struct Provider {
    name: String,
    classes: Vec<EventClass>,
}

impl Provider {
    /// Create a new tracepoint provider
    pub fn new<S: Into<String>>(name: S) -> Provider {
        // TODO: validate name
        Provider {
            name: name.into(),
            classes: Vec::new(),
        }
    }

    /// Create a new class of tracepoint event.
    pub fn create_class<S: Into<String>>(&mut self, class_name: S) -> &mut EventClass {
        self.classes.push(EventClass::new(class_name.into()));
        let cls_len = self.classes.len();
        &mut self.classes[cls_len - 1]
    }
}

/// Represents a class of events that we would like to trace
pub struct EventClass {
    /// The name of this class
    class_name: String,
    /// The provider for this class of tracepoint events.
    fields: Vec<Field>,
    /// The set of instances
    instances: Vec<EventInstance>,
}

impl EventClass {
    /// Create a new tracepoint event class
    fn new(class_name: String) -> Self {
        EventClass {
            class_name,
            fields: Vec::new(),
            instances: Vec::new(),
        }
    }

    pub fn add_field<S: Into<String>>(&mut self, field_name: S, ty: CTFType) -> &mut Self {
        self.fields.push(Field::new(
            field_name.into(), ty
        ));
        self
    }

    pub fn instantiate<S: Into<String>>(&mut self, instance_name: S) -> &mut Self {
        self.instances.push(EventInstance::new(
            instance_name.into()
        ));
        self
    }
}

/// A field in a tracing event
pub struct Field {
    ctf_type: CTFType,
    field_name: String,
}

impl Field {
    fn new(field_name: String, ctf_type: CTFType) -> Self {
        Self {
            ctf_type, field_name,
        }
    }
}

pub struct EventInstance {
    name: String,
}

impl EventInstance {
    fn new(name: String) -> Self {
        EventInstance {
            name,
        }
    }
}

/// Represents the log level for a given tracepoint
pub enum LogLevel {
    /// Corresponds to the `TRACE_EMERG` log level
    Emergency,
    /// Corresponds to the `TRACE_ALERT` log level
    Alert,
    /// Corresponds to the `TRACE_CRIT` log level
    Critical,
    /// Corresponds to the `TRACE_ERR` log level
    Error,
    /// Corresponds to the `TRACE_WARNING` log level
    Warning,
    /// Corresponds to the `TRACE_NOTICE` log level
    Notice,
    /// Corresponds to the `TRACE_INFO` log level
    Info,
    /// Corresponds to the `TRACE_DEBUG_SYSTEM` log level
    DebugSystem,
    /// Corresponds to the `TRACE_DEBUG_PROGRAM` log level
    DebugProgram,
    /// Corresponds to the `TRACE_DEBUG_PROCESS` log level
    DebugProcess,
    /// Corresponds to the `TRACE_DEBUG_MODULE` log level
    DebugModule,
    /// Corresponds to the `TRACE_DEBUG_UNIT` log level
    DebugUnit,
    /// Corresponds to the `TRACE_DEBUG_FUNCTION` log level
    DebugFunction,
    /// Corresponds to the `TRACE_DEBUG_LINE` log level
    DebugLine,
    /// Corresponds to the `TRACE_DEBUG` log level
    Debug
}

/// Represents a C integer type
pub enum CIntegerType {
    I8, I16, I32, I64,
    U8, U16, U32, U64,
}

/// Represents a C float type
pub enum CFloatType {
    Single, Double
}

/// Represents a CTF type
pub enum CTFType {
    /// A standard base-10 integer.
    /// Maps to `ctf_integer`.
    Integer(CIntegerType),
    /// A standard base-10 integer which is available to event filters, but is not persisted to the
    /// event itself.
    /// Maps to `ctf_integer_nowrite`.
    IntegerNoWrite(CIntegerType),
    /// Integer to be printed in hex format.
    /// Maps to `ctf_integer_hex`.
    IntegerHex(CIntegerType),
    /// Integer in network (BE) byte order.
    /// Maps to `ctf_integer_network`.
    IntegerNetwork(CIntegerType),
    /// Integer in network (BE) byte order, to be printed in hex.
    /// Maps to `ctf_niteger_network_hex`.
    IntegerNetworkHex(CIntegerType),
    /// IEEE single- or double- precision float.
    /// Maps to `ctf_float`.
    Float(CFloatType),
    /// IEEE single- or double- precision float which is available to event filters,
    /// but is not persisted to the event itself.
    /// Maps to `ctf_float_nowrite`.
    FloatNoWrite(CFloatType),
    /// A null-terminated string.
    /// Unless you're working with already-terminated `OsStrings`, you probably want to use a
    /// [Text](CTFType::Text) instead.
    /// Maps to `ctf_string`.
    String,
    /// A null-terminated string which is available to event filters, but is not persisted.
    /// Unless you're working with already-terminated `OsStrings`, you probably want to use a
    /// [TextNoWrite](CTFType::TextNoWrite) instead.
    /// Maps to `ctf_string_nowrite`.
    StringNoWrite,
    /// A statically sized array of integers
    /// Maps to `ctf_array`.
    Array(CIntegerType, i32),
    /// A statically sized array of integers which is available to event filters, but is not
    /// persisted.
    /// Maps to `ctf_array_nowrite`.
    ArrayNoWrite(CIntegerType, i32),
    /// Dynamically sized array of integers
    /// Maps to `ctf_sequence`.
    Sequence(CIntegerType),
    /// A dynamically sized array of integers which is available to event filters, but is not
    /// persisted.
    /// Maps to `ctf_sequence_nowrite`.
    SequenceNoWrite(CIntegerType),
    /// Dynamically-sized array, displayed as text
    /// Maps to `ctf_sequence_text`.
    Text,
    /// Dynamically-sized array, displayed as text, but is not persisted.
    /// Maps to `ctf_sequence_text_nowrite`.
    TextNoWrite,
    /// Enumeration value.
    /// TODO: some sort of proc-macro skulduggery is probably required here.
    /// Maps to `ctf_enum`.
    Enum,
    /// Enumeration value. that is available to event filters but is not persisted
    /// TODO: some sort of proc-macro skulduggery is probably required here.
    /// Maps to `ctf_enum_nowrite`.
    EnumNoWrite,
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(add_numbers(2, 3), 5);
    }
}
