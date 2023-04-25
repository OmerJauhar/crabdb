use miette::Diagnostic;
use nom_supreme::error::{BaseErrorKind, ErrorTree, GenericErrorTree, StackContext};
use thiserror::Error;
#[derive(Error, Debug, Diagnostic)]
#[error("Parse Error")] // for custom error 
 //This code defines the FormattedError struct using the Error and Diagnostic derives.
// The Error macro is used to define a custom error type,
// while the Diagnostic derive is used to provide additional information about the error message.
pub struct FormattedError<'b> {
    // need 'b since Diagnostic derive uses 'a
    #[source_code]
    src: &'b str,
    // the src files is annotated with #[source_code] attribute 
    // which specifies that this field contains the code that caused the error 

    #[label("{kind}")]
    span: miette::SourceSpan,
    // will explain this later. TLDR: the parsing error
    kind: BaseErrorKind<&'b str, Box<dyn std::error::Error + Send + Sync + 'static>>,
    #[related]
    others: Vec<FormattedErrorContext<'b>>,
    // The span field is annotated with the #[label("{kind}")] attribute, which specifies that this field should be used to provide a label for the error message. The {kind} placeholder will be replaced with the kind field of the error message.
    //The kind field is used to specify the kind of error that occurred, and is annotated with the BaseErrorKind type from the nom_supreme crate.
    //The others field is used to specify additional error context, and is annotated with the #[related] attribute.
    
}
#[derive(Er while the Diagnostic derive is used to provide additional information about the error message.ror, Debug, Diagnostic)]
#[error("Parse Error Context")]
pub struct FormattedErrorContext<'b> {
    #[source_code]
    src: &'b str,
    #[label("{context}")]
    span: miette::SourceSpan,
    context: StackContext<&'b str>,
}