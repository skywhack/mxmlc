use lazy_static::lazy_static;
use maplit::hashmap;
use crate::ns::*;

lazy_static! {
    pub static ref DATA: HashMap<i32, String> = hashmap! {
        // WhackDiagnosticKind::K.id() => ".".into(),
        WhackDiagnosticKind::EntityIsNotAType.id() => "Entity is not a type.".into(),
        WhackDiagnosticKind::ImplicitCoercionToUnrelatedType.id() => "Implicit coercion of a value of type {1} to an unrelated type {2}.".into(),
        WhackDiagnosticKind::EntityIsReadOnly.id() => "Entity is read-only.".into(),
        WhackDiagnosticKind::EntityIsWriteOnly.id() => "Entity is write-only.".into(),
        WhackDiagnosticKind::EntityMustNotBeDeleted.id() => "Entity must not be deleted.".into(),
        WhackDiagnosticKind::UndefinedProperty.id() => "Access of possibly undefined property {1}.".into(),
        WhackDiagnosticKind::AmbiguousReference.id() => "Ambiguous reference to {1}.".into(),
        WhackDiagnosticKind::AccessOfVoid.id() => "Accessing property of void.".into(),
        WhackDiagnosticKind::AccessOfNullable.id() => "Accessing property of nullable data type.".into(),
        WhackDiagnosticKind::CouldNotExpandInlineConstant.id() => "Could not expand inline constant.".into(),
        WhackDiagnosticKind::ReachedMaximumCycles.id() => "Reached maximum cycles.".into(),
        WhackDiagnosticKind::NullNotExpectedHere.id() => "Null not expected here.".into(),
        WhackDiagnosticKind::CouldNotParseNumber.id() => "Could not parse {1}.".into(),
        WhackDiagnosticKind::NoMatchingEnumMember.id() => "Found no member {1} in {2}.".into(),
        WhackDiagnosticKind::UnexpectedThis.id() => "Unexpected this.".into(),
        WhackDiagnosticKind::ArrayLengthNotEqualsTupleLength.id() => "Array length is not equals length of tuple {1}.".into(),
        WhackDiagnosticKind::UnexpectedElision.id() => "Unexpected elision.".into(),
        WhackDiagnosticKind::UnexpectedArray.id() => "Unexpected array.".into(),
        WhackDiagnosticKind::UnexpectedRest.id() => "Unexpected rest.".into(),
        WhackDiagnosticKind::UnexpectedObject.id() => "Unexpected object.".into(),
        WhackDiagnosticKind::DynamicOptionNotSupported.id() => "Dynamic option name is not supported.".into(),
        WhackDiagnosticKind::UnknownOptionForClass.id() => "Unknown option {1} for {2}.".into(),
        WhackDiagnosticKind::MustSpecifyOption.id() => "Must specify option {1}.".into(),
        WhackDiagnosticKind::UnexpectedFieldName.id() => "Unexpected field name.".into(),
        WhackDiagnosticKind::UnexpectedNewBase.id() => "Unexpected new base.".into(),
        WhackDiagnosticKind::IncorrectNumArguments.id() => "Incorrect number of arguments. Expected {1}".into(),
        WhackDiagnosticKind::IncorrectNumArgumentsNoMoreThan.id() => "Incorrect number of arguments. Expected no more than {1}".into(),
        WhackDiagnosticKind::UndefinedPropertyWithStaticType.id() => "Access of possibly undefined property {1} through a reference with static type {2}.".into(),
        WhackDiagnosticKind::InapplicableFilter.id() => "Attempt to filter through a reference with static type {1}.".into(),
        WhackDiagnosticKind::InapplicableDescendants.id() => "Attempt to search descendants through a reference with static type {1}.".into(),
        WhackDiagnosticKind::ASuperExpCanBeUsedOnlyIn.id() => "A super expression can be used only in class instance methods.".into(),
        WhackDiagnosticKind::ASuperExpCanOnlyBeUsedInSubclasses.id() => "A super expression can be used only in subclasses of Object.".into(),
        WhackDiagnosticKind::CallOnArrayType.id() => "A call on the Array type is equivalent to a new expression.".into(),
        WhackDiagnosticKind::CallOnNonFunction.id() => "Call on non Function object.".into(),
        WhackDiagnosticKind::NonParameterizedType.id() => "Applying types on non parameterized type.".into(),
        WhackDiagnosticKind::AwaitOperandMustBeAPromise.id() => "Await operand must be a Promise.".into(),
        WhackDiagnosticKind::OperandMustBeNumber.id() => "Operand must be a Number.".into(),
        WhackDiagnosticKind::ReferenceIsAlreadyNonNullable.id() => "Reference is already non nullable.".into(),
        WhackDiagnosticKind::YieldIsNotSupported.id() => "Yield operator is currently not supported.".into(),
        WhackDiagnosticKind::UnrelatedMathOperation.id() => "Unrelated mathematical operation using type {1}.".into(),
        WhackDiagnosticKind::ComparisonBetweenUnrelatedTypes.id() => "Comparison between a value of type {1} and an unrelated type {2}.".into(),
        WhackDiagnosticKind::UnrelatedTernaryOperands.id() => "Unrelated ternary operands of types {1} and {2}.".into(),
        WhackDiagnosticKind::SystemNamespaceNotFound.id() => "System namespace not found.".into(),
        WhackDiagnosticKind::RestParameterMustBeArray.id() => "Rest parameter must be an Array.".into(),
        WhackDiagnosticKind::AConflictExistsWithDefinition.id() => "A conflict exists with definition {1} in namespace {2}.".into(),
        WhackDiagnosticKind::DuplicateVariableDefinition.id() => "Duplicate variable definition: {1}.".into(),
        WhackDiagnosticKind::DuplicateClassDefinition.id() => "Duplicate variable definition: {1}.".into(),
        WhackDiagnosticKind::DuplicateInterfaceDefinition.id() => "Duplicate interface definition: {1}.".into(),
        WhackDiagnosticKind::DuplicateFunctionDefinition.id() => "Duplicate function definition: {1}.".into(),
        WhackDiagnosticKind::UnexpectedFieldNameInDestructuring.id() => "Unexpected field name in destructuring.".into(),
        WhackDiagnosticKind::EntityIsNotAConstant.id() => "Entity is not a constant.".into(),
        WhackDiagnosticKind::ReturnValueHasNoTypeDeclaration.id() => "Return value has no type declaration.".into(),
        WhackDiagnosticKind::ReturnTypeDeclarationMustBePromise.id() => "Return type declaration must be Promise.".into(),
        WhackDiagnosticKind::ReturnTypeInferenceIsNotImplemented.id() => "Return type inference is not implemented in the present. Using an untyped type.".into(),
        WhackDiagnosticKind::NanComparison.id() => "Comparison involving NaN. Use isNaN() or !isNaN() instead.".into(),
        WhackDiagnosticKind::NotABooleanConstant.id() => "Not a Boolean constant.".into(),
        WhackDiagnosticKind::EmptyPackage.id() => "Package {1} is empty.".into(),
        WhackDiagnosticKind::ImportOfUndefined.id() => "Import of undefined property {1}.".into(),
        WhackDiagnosticKind::NotANamespaceConstant.id() => "Not a Namespace constant.".into(),
        WhackDiagnosticKind::CannotResolveConfigConstant.id() => "Can not resolve configuration constant: '{1}'".into(),
        WhackDiagnosticKind::ConcatenatingSelfReferentialPackage.id() => "Concatenating a self referential package.".into(),
        WhackDiagnosticKind::CallOnDateType.id() => "'Date(...)' is not a cast. It ignores its arguments and returns a String value equal to 'new Date().toString()'. To cast a value to type Date use 'x as Date' instead of 'Date(x)'.".into(),
        WhackDiagnosticKind::AccessControlNamespaceNotAllowedHere.id() => "Access control namespace not allowed here.".into(),
        WhackDiagnosticKind::CannotUseDestructuringHere.id() => "Cannot use destructuring here.".into(),
        WhackDiagnosticKind::ShadowingDefinitionInBaseClass.id() => "Shadowing definition in base class: $1.".into(),
        WhackDiagnosticKind::VariableHasNoTypeAnnotation.id() => "Variable has no type annotation.".into(),
        WhackDiagnosticKind::ConstantMustContainInitializer.id() => "Constant must contain initializer.".into(),
        WhackDiagnosticKind::ExternalFunctionMustBeNativeOrAbstract.id() => "External function must be marked native or abstract.".into(),
        WhackDiagnosticKind::IncompatibleOverride.id() => "Incompatible override signature: expected $1, but specified $2.".into(),
        WhackDiagnosticKind::MustOverrideAMethod.id() => "Must override a method.".into(),
        WhackDiagnosticKind::OverridingFinalMethod.id() => "Cannot override a method marked final.".into(),
        WhackDiagnosticKind::RedefiningConstructor.id() => "Redefining constructor.".into(),
        WhackDiagnosticKind::ConstructorMustContainSuperStatement.id() => "Constructor must contain super statement.".into(),
        WhackDiagnosticKind::GetterMustTakeNoParameters.id() => "Getter must take no parameters.".into(),
        WhackDiagnosticKind::SetterMustTakeOneParameter.id() => "Setter must take one parameter.".into(),
        WhackDiagnosticKind::GetterMustReturnDataType.id() => "Getter must return data type $1.".into(),
        WhackDiagnosticKind::SetterMustTakeDataType.id() => "Setter must take data type $1.".into(),
        WhackDiagnosticKind::SetterMustReturnVoid.id() => "Setter must return void.".into(),
        WhackDiagnosticKind::ExternalClassMustSetSlots.id() => "External class must assign a number to the 'slots' key of the 'Whack::External' meta-data.".into(),
        WhackDiagnosticKind::NotAClass.id() => "Not a class.".into(),
        WhackDiagnosticKind::CannotExtendFinalClass.id() => "Cannot extend final class $1.".into(),
        WhackDiagnosticKind::ExtendingSelfReferentialClass.id() => "Extending self-referential class.".into(),
        WhackDiagnosticKind::NotAnInterface.id() => "Not an interface.".into(),
        WhackDiagnosticKind::OptionsClassMustExtendObject.id() => "Options class must extend Object.".into(),
        WhackDiagnosticKind::MalformedEventMetadata.id() => "Malformed Event meta-data.".into(),
        WhackDiagnosticKind::AbstractMethodMustBeOverriden.id() => "Abstract method must be overriden: $1.".into(),
        WhackDiagnosticKind::AbstractGetterMustBeOverriden.id() => "Abstract getter must be overriden: $1.".into(),
        WhackDiagnosticKind::AbstractSetterMustBeOverriden.id() => "Abstract setter must be overriden: $1.".into(),
        WhackDiagnosticKind::ClassMustDefineAConstructor.id() => "Class must define a constructor.".into(),
        WhackDiagnosticKind::MethodNotImplemented.id() => "Method not implemented: $1.".into(),
        WhackDiagnosticKind::GetterNotImplemented.id() => "Getter not implemented: $1.".into(),
        WhackDiagnosticKind::SetterNotImplemented.id() => "Setter not implemented: $1.".into(),
        WhackDiagnosticKind::IncompatibleMethodSignature.id() => "Incompatible method signature for $1: expected $2.".into(),
        WhackDiagnosticKind::IncompatibleGetterSignature.id() => "Incompatible getter signature for $1: expected $2.".into(),
        WhackDiagnosticKind::IncompatibleSetterSignature.id() => "Incompatible setter signature for $1: expected $2.".into(),
        WhackDiagnosticKind::PropertyMustBeMethod.id() => "Property must be method: $1.".into(),
        WhackDiagnosticKind::PropertyMustBeVirtual.id() => "Property must be virtual variable: $1.".into(),
        WhackDiagnosticKind::IllegalEnumConstInit.id() => "Illegal enum constant initialiser.".into(),
        WhackDiagnosticKind::DuplicateEnumString.id() => "Duplicate enum string: $1.".into(),
        WhackDiagnosticKind::DuplicateEnumValue.id() => "Duplicate enum value: $1.".into(),
        WhackDiagnosticKind::DuplicateEnumConstant.id() => "Duplicate enum constant: $1.".into(),
        WhackDiagnosticKind::ExtendingSelfReferentialInterface.id() => "Extending self-referential interface.".into(),
        WhackDiagnosticKind::CannotIterateType.id() => "Cannot iterate data type $1.".into(),
        WhackDiagnosticKind::ExpectedToIterateType.id() => "Expected to iterate item of data type $1.".into(),
        WhackDiagnosticKind::IllegalReturnStatement.id() => "Illegal return statement.".into(),
        WhackDiagnosticKind::ReturnValueMustBeSpecified.id() => "Return value must be specified.".into(),
        WhackDiagnosticKind::DxnsStatementIsNotSupported.id() => "Default XML namespace statement is not supported in HTML5 environment.".into(),
        WhackDiagnosticKind::ReturnNotAllowedInPackageInit.id() => "The return statement cannot be used in package initialization code.".into(),
        WhackDiagnosticKind::ReturnNotAllowedInGlobalInit.id() => "The return statement cannot be used in global initialization code.".into(),
        // WhackDiagnosticKind::K.id() => ".".into(),
    };
}