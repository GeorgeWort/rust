//! Errors emitted by codegen_ssa

use crate::assert_module_sources::CguReuse;
use crate::back::command::Command;
use crate::fluent_generated as fluent;
use rustc_errors::{
    codes::*, Diag, DiagArgValue, DiagCtxtHandle, Diagnostic, EmissionGuarantee, IntoDiagArg, Level,
};
use rustc_macros::Diagnostic;
use rustc_middle::ty::layout::LayoutError;
use rustc_middle::ty::Ty;
use rustc_span::{Span, Symbol};
use rustc_type_ir::FloatTy;
use std::borrow::Cow;
use std::io::Error;
use std::path::{Path, PathBuf};
use std::process::ExitStatus;

#[derive(Diagnostic)]
#[diag(codegen_ssa_incorrect_cgu_reuse_type)]
pub struct IncorrectCguReuseType<'a> {
    #[primary_span]
    pub span: Span,
    pub cgu_user_name: &'a str,
    pub actual_reuse: CguReuse,
    pub expected_reuse: CguReuse,
    pub at_least: u8,
}

#[derive(Diagnostic)]
#[diag(codegen_ssa_cgu_not_recorded)]
pub struct CguNotRecorded<'a> {
    pub cgu_user_name: &'a str,
    pub cgu_name: &'a str,
}

#[derive(Diagnostic)]
#[diag(codegen_ssa_unknown_reuse_kind)]
pub struct UnknownReuseKind {
    #[primary_span]
    pub span: Span,
    pub kind: Symbol,
}

#[derive(Diagnostic)]
#[diag(codegen_ssa_missing_query_depgraph)]
pub struct MissingQueryDepGraph {
    #[primary_span]
    pub span: Span,
}

#[derive(Diagnostic)]
#[diag(codegen_ssa_malformed_cgu_name)]
pub struct MalformedCguName {
    #[primary_span]
    pub span: Span,
    pub user_path: String,
    pub crate_name: String,
}

#[derive(Diagnostic)]
#[diag(codegen_ssa_no_module_named)]
pub struct NoModuleNamed<'a> {
    #[primary_span]
    pub span: Span,
    pub user_path: &'a str,
    pub cgu_name: Symbol,
    pub cgu_names: String,
}

#[derive(Diagnostic)]
#[diag(codegen_ssa_field_associated_value_expected)]
pub struct FieldAssociatedValueExpected {
    #[primary_span]
    pub span: Span,
    pub name: Symbol,
}

#[derive(Diagnostic)]
#[diag(codegen_ssa_no_field)]
pub struct NoField {
    #[primary_span]
    pub span: Span,
    pub name: Symbol,
}

#[derive(Diagnostic)]
#[diag(codegen_ssa_lib_def_write_failure)]
pub struct LibDefWriteFailure {
    pub error: Error,
}

#[derive(Diagnostic)]
#[diag(codegen_ssa_version_script_write_failure)]
pub struct VersionScriptWriteFailure {
    pub error: Error,
}

#[derive(Diagnostic)]
#[diag(codegen_ssa_symbol_file_write_failure)]
pub struct SymbolFileWriteFailure {
    pub error: Error,
}

#[derive(Diagnostic)]
#[diag(codegen_ssa_ld64_unimplemented_modifier)]
pub struct Ld64UnimplementedModifier;

#[derive(Diagnostic)]
#[diag(codegen_ssa_linker_unsupported_modifier)]
pub struct LinkerUnsupportedModifier;

#[derive(Diagnostic)]
#[diag(codegen_ssa_L4Bender_exporting_symbols_unimplemented)]
pub struct L4BenderExportingSymbolsUnimplemented;

#[derive(Diagnostic)]
#[diag(codegen_ssa_no_natvis_directory)]
pub struct NoNatvisDirectory {
    pub error: Error,
}

#[derive(Diagnostic)]
#[diag(codegen_ssa_no_saved_object_file)]
pub struct NoSavedObjectFile<'a> {
    pub cgu_name: &'a str,
}

#[derive(Diagnostic)]
#[diag(codegen_ssa_copy_path_buf)]
pub struct CopyPathBuf {
    pub source_file: PathBuf,
    pub output_path: PathBuf,
    pub error: Error,
}

// Reports Paths using `Debug` implementation rather than Path's `Display` implementation.
#[derive(Diagnostic)]
#[diag(codegen_ssa_copy_path)]
pub struct CopyPath<'a> {
    from: DebugArgPath<'a>,
    to: DebugArgPath<'a>,
    error: Error,
}

impl<'a> CopyPath<'a> {
    pub fn new(from: &'a Path, to: &'a Path, error: Error) -> CopyPath<'a> {
        CopyPath { from: DebugArgPath(from), to: DebugArgPath(to), error }
    }
}

struct DebugArgPath<'a>(pub &'a Path);

impl IntoDiagArg for DebugArgPath<'_> {
    fn into_diag_arg(self) -> rustc_errors::DiagArgValue {
        DiagArgValue::Str(Cow::Owned(format!("{:?}", self.0)))
    }
}

#[derive(Diagnostic)]
#[diag(codegen_ssa_binary_output_to_tty)]
pub struct BinaryOutputToTty {
    pub shorthand: &'static str,
}

#[derive(Diagnostic)]
#[diag(codegen_ssa_ignoring_emit_path)]
pub struct IgnoringEmitPath {
    pub extension: String,
}

#[derive(Diagnostic)]
#[diag(codegen_ssa_ignoring_output)]
pub struct IgnoringOutput {
    pub extension: String,
}

#[derive(Diagnostic)]
#[diag(codegen_ssa_create_temp_dir)]
pub struct CreateTempDir {
    pub error: Error,
}

#[derive(Diagnostic)]
#[diag(codegen_ssa_add_native_library)]
pub struct AddNativeLibrary {
    pub library_path: PathBuf,
    pub error: Error,
}

#[derive(Diagnostic)]
#[diag(codegen_ssa_multiple_external_func_decl)]
pub struct MultipleExternalFuncDecl<'a> {
    #[primary_span]
    pub span: Span,
    pub function: Symbol,
    pub library_name: &'a str,
}

#[derive(Diagnostic)]
pub enum LinkRlibError {
    #[diag(codegen_ssa_rlib_missing_format)]
    MissingFormat,

    #[diag(codegen_ssa_rlib_only_rmeta_found)]
    OnlyRmetaFound { crate_name: Symbol },

    #[diag(codegen_ssa_rlib_not_found)]
    NotFound { crate_name: Symbol },

    #[diag(codegen_ssa_rlib_incompatible_dependency_formats)]
    IncompatibleDependencyFormats { ty1: String, ty2: String, list1: String, list2: String },
}

pub struct ThorinErrorWrapper(pub thorin::Error);

impl<G: EmissionGuarantee> Diagnostic<'_, G> for ThorinErrorWrapper {
    fn into_diag(self, dcx: DiagCtxtHandle<'_>, level: Level) -> Diag<'_, G> {
        let build = |msg| Diag::new(dcx, level, msg);
        match self.0 {
            thorin::Error::ReadInput(_) => build(fluent::codegen_ssa_thorin_read_input_failure),
            thorin::Error::ParseFileKind(_) => {
                build(fluent::codegen_ssa_thorin_parse_input_file_kind)
            }
            thorin::Error::ParseObjectFile(_) => {
                build(fluent::codegen_ssa_thorin_parse_input_object_file)
            }
            thorin::Error::ParseArchiveFile(_) => {
                build(fluent::codegen_ssa_thorin_parse_input_archive_file)
            }
            thorin::Error::ParseArchiveMember(_) => {
                build(fluent::codegen_ssa_thorin_parse_archive_member)
            }
            thorin::Error::InvalidInputKind => build(fluent::codegen_ssa_thorin_invalid_input_kind),
            thorin::Error::DecompressData(_) => build(fluent::codegen_ssa_thorin_decompress_data),
            thorin::Error::NamelessSection(_, offset) => {
                build(fluent::codegen_ssa_thorin_section_without_name)
                    .with_arg("offset", format!("0x{offset:08x}"))
            }
            thorin::Error::RelocationWithInvalidSymbol(section, offset) => {
                build(fluent::codegen_ssa_thorin_relocation_with_invalid_symbol)
                    .with_arg("section", section)
                    .with_arg("offset", format!("0x{offset:08x}"))
            }
            thorin::Error::MultipleRelocations(section, offset) => {
                build(fluent::codegen_ssa_thorin_multiple_relocations)
                    .with_arg("section", section)
                    .with_arg("offset", format!("0x{offset:08x}"))
            }
            thorin::Error::UnsupportedRelocation(section, offset) => {
                build(fluent::codegen_ssa_thorin_unsupported_relocation)
                    .with_arg("section", section)
                    .with_arg("offset", format!("0x{offset:08x}"))
            }
            thorin::Error::MissingDwoName(id) => build(fluent::codegen_ssa_thorin_missing_dwo_name)
                .with_arg("id", format!("0x{id:08x}")),
            thorin::Error::NoCompilationUnits => {
                build(fluent::codegen_ssa_thorin_no_compilation_units)
            }
            thorin::Error::NoDie => build(fluent::codegen_ssa_thorin_no_die),
            thorin::Error::TopLevelDieNotUnit => {
                build(fluent::codegen_ssa_thorin_top_level_die_not_unit)
            }
            thorin::Error::MissingRequiredSection(section) => {
                build(fluent::codegen_ssa_thorin_missing_required_section)
                    .with_arg("section", section)
            }
            thorin::Error::ParseUnitAbbreviations(_) => {
                build(fluent::codegen_ssa_thorin_parse_unit_abbreviations)
            }
            thorin::Error::ParseUnitAttribute(_) => {
                build(fluent::codegen_ssa_thorin_parse_unit_attribute)
            }
            thorin::Error::ParseUnitHeader(_) => {
                build(fluent::codegen_ssa_thorin_parse_unit_header)
            }
            thorin::Error::ParseUnit(_) => build(fluent::codegen_ssa_thorin_parse_unit),
            thorin::Error::IncompatibleIndexVersion(section, format, actual) => {
                build(fluent::codegen_ssa_thorin_incompatible_index_version)
                    .with_arg("section", section)
                    .with_arg("actual", actual)
                    .with_arg("format", format)
            }
            thorin::Error::OffsetAtIndex(_, index) => {
                build(fluent::codegen_ssa_thorin_offset_at_index).with_arg("index", index)
            }
            thorin::Error::StrAtOffset(_, offset) => {
                build(fluent::codegen_ssa_thorin_str_at_offset)
                    .with_arg("offset", format!("0x{offset:08x}"))
            }
            thorin::Error::ParseIndex(_, section) => {
                build(fluent::codegen_ssa_thorin_parse_index).with_arg("section", section)
            }
            thorin::Error::UnitNotInIndex(unit) => {
                build(fluent::codegen_ssa_thorin_unit_not_in_index)
                    .with_arg("unit", format!("0x{unit:08x}"))
            }
            thorin::Error::RowNotInIndex(_, row) => {
                build(fluent::codegen_ssa_thorin_row_not_in_index).with_arg("row", row)
            }
            thorin::Error::SectionNotInRow => build(fluent::codegen_ssa_thorin_section_not_in_row),
            thorin::Error::EmptyUnit(unit) => build(fluent::codegen_ssa_thorin_empty_unit)
                .with_arg("unit", format!("0x{unit:08x}")),
            thorin::Error::MultipleDebugInfoSection => {
                build(fluent::codegen_ssa_thorin_multiple_debug_info_section)
            }
            thorin::Error::MultipleDebugTypesSection => {
                build(fluent::codegen_ssa_thorin_multiple_debug_types_section)
            }
            thorin::Error::NotSplitUnit => build(fluent::codegen_ssa_thorin_not_split_unit),
            thorin::Error::DuplicateUnit(unit) => build(fluent::codegen_ssa_thorin_duplicate_unit)
                .with_arg("unit", format!("0x{unit:08x}")),
            thorin::Error::MissingReferencedUnit(unit) => {
                build(fluent::codegen_ssa_thorin_missing_referenced_unit)
                    .with_arg("unit", format!("0x{unit:08x}"))
            }
            thorin::Error::NoOutputObjectCreated => {
                build(fluent::codegen_ssa_thorin_not_output_object_created)
            }
            thorin::Error::MixedInputEncodings => {
                build(fluent::codegen_ssa_thorin_mixed_input_encodings)
            }
            thorin::Error::Io(e) => {
                build(fluent::codegen_ssa_thorin_io).with_arg("error", format!("{e}"))
            }
            thorin::Error::ObjectRead(e) => {
                build(fluent::codegen_ssa_thorin_object_read).with_arg("error", format!("{e}"))
            }
            thorin::Error::ObjectWrite(e) => {
                build(fluent::codegen_ssa_thorin_object_write).with_arg("error", format!("{e}"))
            }
            thorin::Error::GimliRead(e) => {
                build(fluent::codegen_ssa_thorin_gimli_read).with_arg("error", format!("{e}"))
            }
            thorin::Error::GimliWrite(e) => {
                build(fluent::codegen_ssa_thorin_gimli_write).with_arg("error", format!("{e}"))
            }
            _ => unimplemented!("Untranslated thorin error"),
        }
    }
}

pub struct LinkingFailed<'a> {
    pub linker_path: &'a PathBuf,
    pub exit_status: ExitStatus,
    pub command: &'a Command,
    pub escaped_output: String,
}

impl<G: EmissionGuarantee> Diagnostic<'_, G> for LinkingFailed<'_> {
    fn into_diag(self, dcx: DiagCtxtHandle<'_>, level: Level) -> Diag<'_, G> {
        let mut diag = Diag::new(dcx, level, fluent::codegen_ssa_linking_failed);
        diag.arg("linker_path", format!("{}", self.linker_path.display()));
        diag.arg("exit_status", format!("{}", self.exit_status));

        let contains_undefined_ref = self.escaped_output.contains("undefined reference to");

        diag.note(format!("{:?}", self.command)).note(self.escaped_output);

        // Trying to match an error from OS linkers
        // which by now we have no way to translate.
        if contains_undefined_ref {
            diag.note(fluent::codegen_ssa_extern_funcs_not_found)
                .note(fluent::codegen_ssa_specify_libraries_to_link);

            if rustc_session::utils::was_invoked_from_cargo() {
                diag.note(fluent::codegen_ssa_use_cargo_directive);
            }
        }
        diag
    }
}

#[derive(Diagnostic)]
#[diag(codegen_ssa_link_exe_unexpected_error)]
pub struct LinkExeUnexpectedError;

#[derive(Diagnostic)]
#[diag(codegen_ssa_repair_vs_build_tools)]
pub struct RepairVSBuildTools;

#[derive(Diagnostic)]
#[diag(codegen_ssa_missing_cpp_build_tool_component)]
pub struct MissingCppBuildToolComponent;

#[derive(Diagnostic)]
#[diag(codegen_ssa_select_cpp_build_tool_workload)]
pub struct SelectCppBuildToolWorkload;

#[derive(Diagnostic)]
#[diag(codegen_ssa_visual_studio_not_installed)]
pub struct VisualStudioNotInstalled;

#[derive(Diagnostic)]
#[diag(codegen_ssa_linker_not_found)]
#[note]
pub struct LinkerNotFound {
    pub linker_path: PathBuf,
    pub error: Error,
}

#[derive(Diagnostic)]
#[diag(codegen_ssa_unable_to_exe_linker)]
#[note]
#[note(codegen_ssa_command_note)]
pub struct UnableToExeLinker {
    pub linker_path: PathBuf,
    pub error: Error,
    pub command_formatted: String,
}

#[derive(Diagnostic)]
#[diag(codegen_ssa_msvc_missing_linker)]
pub struct MsvcMissingLinker;

#[derive(Diagnostic)]
#[diag(codegen_ssa_self_contained_linker_missing)]
pub struct SelfContainedLinkerMissing;

#[derive(Diagnostic)]
#[diag(codegen_ssa_check_installed_visual_studio)]
pub struct CheckInstalledVisualStudio;

#[derive(Diagnostic)]
#[diag(codegen_ssa_insufficient_vs_code_product)]
pub struct InsufficientVSCodeProduct;

#[derive(Diagnostic)]
#[diag(codegen_ssa_processing_dymutil_failed)]
#[note]
pub struct ProcessingDymutilFailed {
    pub status: ExitStatus,
    pub output: String,
}

#[derive(Diagnostic)]
#[diag(codegen_ssa_unable_to_run_dsymutil)]
pub struct UnableToRunDsymutil {
    pub error: Error,
}

#[derive(Diagnostic)]
#[diag(codegen_ssa_stripping_debug_info_failed)]
#[note]
pub struct StrippingDebugInfoFailed<'a> {
    pub util: &'a str,
    pub status: ExitStatus,
    pub output: String,
}

#[derive(Diagnostic)]
#[diag(codegen_ssa_unable_to_run)]
pub struct UnableToRun<'a> {
    pub util: &'a str,
    pub error: Error,
}

#[derive(Diagnostic)]
#[diag(codegen_ssa_linker_file_stem)]
pub struct LinkerFileStem;

#[derive(Diagnostic)]
#[diag(codegen_ssa_static_library_native_artifacts)]
pub struct StaticLibraryNativeArtifacts;

#[derive(Diagnostic)]
#[diag(codegen_ssa_static_library_native_artifacts_to_file)]
pub struct StaticLibraryNativeArtifactsToFile<'a> {
    pub path: &'a Path,
}

#[derive(Diagnostic)]
#[diag(codegen_ssa_link_script_unavailable)]
pub struct LinkScriptUnavailable;

#[derive(Diagnostic)]
#[diag(codegen_ssa_link_script_write_failure)]
pub struct LinkScriptWriteFailure {
    pub path: PathBuf,
    pub error: Error,
}

#[derive(Diagnostic)]
#[diag(codegen_ssa_failed_to_write)]
pub struct FailedToWrite {
    pub path: PathBuf,
    pub error: Error,
}

#[derive(Diagnostic)]
#[diag(codegen_ssa_unable_to_write_debugger_visualizer)]
pub struct UnableToWriteDebuggerVisualizer {
    pub path: PathBuf,
    pub error: Error,
}

#[derive(Diagnostic)]
#[diag(codegen_ssa_rlib_archive_build_failure)]
pub struct RlibArchiveBuildFailure {
    pub error: Error,
}

#[derive(Diagnostic)]
pub enum ExtractBundledLibsError<'a> {
    #[diag(codegen_ssa_extract_bundled_libs_open_file)]
    OpenFile { rlib: &'a Path, error: Box<dyn std::error::Error> },

    #[diag(codegen_ssa_extract_bundled_libs_mmap_file)]
    MmapFile { rlib: &'a Path, error: Box<dyn std::error::Error> },

    #[diag(codegen_ssa_extract_bundled_libs_parse_archive)]
    ParseArchive { rlib: &'a Path, error: Box<dyn std::error::Error> },

    #[diag(codegen_ssa_extract_bundled_libs_read_entry)]
    ReadEntry { rlib: &'a Path, error: Box<dyn std::error::Error> },

    #[diag(codegen_ssa_extract_bundled_libs_archive_member)]
    ArchiveMember { rlib: &'a Path, error: Box<dyn std::error::Error> },

    #[diag(codegen_ssa_extract_bundled_libs_convert_name)]
    ConvertName { rlib: &'a Path, error: Box<dyn std::error::Error> },

    #[diag(codegen_ssa_extract_bundled_libs_write_file)]
    WriteFile { rlib: &'a Path, error: Box<dyn std::error::Error> },

    #[diag(codegen_ssa_extract_bundled_libs_write_file)]
    ExtractSection { rlib: &'a Path, error: Box<dyn std::error::Error> },
}

#[derive(Diagnostic)]
#[diag(codegen_ssa_unsupported_arch)]
pub struct UnsupportedArch<'a> {
    pub arch: &'a str,
    pub os: &'a str,
}

#[derive(Diagnostic)]
pub enum AppleSdkRootError<'a> {
    #[diag(codegen_ssa_apple_sdk_error_sdk_path)]
    SdkPath { sdk_name: &'a str, error: Error },
}

#[derive(Diagnostic)]
#[diag(codegen_ssa_read_file)]
pub struct ReadFileError {
    pub message: std::io::Error,
}

#[derive(Diagnostic)]
#[diag(codegen_ssa_unsupported_link_self_contained)]
pub struct UnsupportedLinkSelfContained;

#[derive(Diagnostic)]
#[diag(codegen_ssa_archive_build_failure)]
// Public for rustc_codegen_llvm::back::archive
pub struct ArchiveBuildFailure {
    pub error: std::io::Error,
}

#[derive(Diagnostic)]
#[diag(codegen_ssa_unknown_archive_kind)]
// Public for rustc_codegen_llvm::back::archive
pub struct UnknownArchiveKind<'a> {
    pub kind: &'a str,
}

#[derive(Diagnostic)]
#[diag(codegen_ssa_expected_used_symbol)]
pub struct ExpectedUsedSymbol {
    #[primary_span]
    pub span: Span,
}

#[derive(Diagnostic)]
#[diag(codegen_ssa_multiple_main_functions)]
#[help]
pub struct MultipleMainFunctions {
    #[primary_span]
    pub span: Span,
}

#[derive(Diagnostic)]
#[diag(codegen_ssa_metadata_object_file_write)]
pub struct MetadataObjectFileWrite {
    pub error: Error,
}

#[derive(Diagnostic)]
#[diag(codegen_ssa_invalid_windows_subsystem)]
pub struct InvalidWindowsSubsystem {
    pub subsystem: Symbol,
}

#[derive(Diagnostic)]
#[diag(codegen_ssa_shuffle_indices_evaluation)]
pub struct ShuffleIndicesEvaluation {
    #[primary_span]
    pub span: Span,
}

#[derive(Diagnostic)]
#[diag(codegen_ssa_const_vector_evaluation)]
pub struct ConstVectorEvaluation {
    #[primary_span]
    pub span: Span,
}

#[derive(Diagnostic)]
#[diag(codegen_ssa_intrinsic_const_vector_arg)]
pub struct IntrinsicConstVectorArg {
    #[primary_span]
    pub span: Span,
}

#[derive(Diagnostic)]
#[diag(codegen_ssa_missing_memory_ordering)]
pub struct MissingMemoryOrdering;

#[derive(Diagnostic)]
#[diag(codegen_ssa_unknown_atomic_ordering)]
pub struct UnknownAtomicOrdering;

#[derive(Diagnostic)]
#[diag(codegen_ssa_atomic_compare_exchange)]
pub struct AtomicCompareExchange;

#[derive(Diagnostic)]
#[diag(codegen_ssa_unknown_atomic_operation)]
pub struct UnknownAtomicOperation;

#[derive(Diagnostic)]
pub enum InvalidMonomorphization<'tcx> {
    #[diag(codegen_ssa_invalid_monomorphization_basic_integer_type, code = E0511)]
    BasicIntegerType {
        #[primary_span]
        span: Span,
        name: Symbol,
        ty: Ty<'tcx>,
    },

    #[diag(codegen_ssa_invalid_monomorphization_basic_float_type, code = E0511)]
    BasicFloatType {
        #[primary_span]
        span: Span,
        name: Symbol,
        ty: Ty<'tcx>,
    },

    #[diag(codegen_ssa_invalid_monomorphization_float_to_int_unchecked, code = E0511)]
    FloatToIntUnchecked {
        #[primary_span]
        span: Span,
        ty: Ty<'tcx>,
    },

    #[diag(codegen_ssa_invalid_monomorphization_floating_point_vector, code = E0511)]
    FloatingPointVector {
        #[primary_span]
        span: Span,
        name: Symbol,
        f_ty: FloatTy,
        in_ty: Ty<'tcx>,
    },

    #[diag(codegen_ssa_invalid_monomorphization_floating_point_type, code = E0511)]
    FloatingPointType {
        #[primary_span]
        span: Span,
        name: Symbol,
        in_ty: Ty<'tcx>,
    },

    #[diag(codegen_ssa_invalid_monomorphization_unrecognized_intrinsic, code = E0511)]
    UnrecognizedIntrinsic {
        #[primary_span]
        span: Span,
        name: Symbol,
    },

    #[diag(codegen_ssa_invalid_monomorphization_simd_argument, code = E0511)]
    SimdArgument {
        #[primary_span]
        span: Span,
        name: Symbol,
        ty: Ty<'tcx>,
    },

    #[diag(codegen_ssa_invalid_monomorphization_simd_input, code = E0511)]
    SimdInput {
        #[primary_span]
        span: Span,
        name: Symbol,
        ty: Ty<'tcx>,
    },

    #[diag(codegen_ssa_invalid_monomorphization_simd_first, code = E0511)]
    SimdFirst {
        #[primary_span]
        span: Span,
        name: Symbol,
        ty: Ty<'tcx>,
    },

    #[diag(codegen_ssa_invalid_monomorphization_simd_second, code = E0511)]
    SimdSecond {
        #[primary_span]
        span: Span,
        name: Symbol,
        ty: Ty<'tcx>,
    },

    #[diag(codegen_ssa_invalid_monomorphization_simd_third, code = E0511)]
    SimdThird {
        #[primary_span]
        span: Span,
        name: Symbol,
        ty: Ty<'tcx>,
    },

    #[diag(codegen_ssa_invalid_monomorphization_simd_return, code = E0511)]
    SimdReturn {
        #[primary_span]
        span: Span,
        name: Symbol,
        ty: Ty<'tcx>,
    },

    #[diag(codegen_ssa_invalid_monomorphization_invalid_bitmask, code = E0511)]
    InvalidBitmask {
        #[primary_span]
        span: Span,
        name: Symbol,
        mask_ty: Ty<'tcx>,
        expected_int_bits: u64,
        expected_bytes: u64,
    },

    #[diag(codegen_ssa_invalid_monomorphization_return_length_input_type, code = E0511)]
    ReturnLengthInputType {
        #[primary_span]
        span: Span,
        name: Symbol,
        in_len: u64,
        in_ty: Ty<'tcx>,
        ret_ty: Ty<'tcx>,
        out_len: u64,
    },

    #[diag(codegen_ssa_invalid_monomorphization_second_argument_length, code = E0511)]
    SecondArgumentLength {
        #[primary_span]
        span: Span,
        name: Symbol,
        in_len: u64,
        in_ty: Ty<'tcx>,
        arg_ty: Ty<'tcx>,
        out_len: u64,
    },

    #[diag(codegen_ssa_invalid_monomorphization_third_argument_length, code = E0511)]
    ThirdArgumentLength {
        #[primary_span]
        span: Span,
        name: Symbol,
        in_len: u64,
        in_ty: Ty<'tcx>,
        arg_ty: Ty<'tcx>,
        out_len: u64,
    },

    #[diag(codegen_ssa_invalid_monomorphization_return_integer_type, code = E0511)]
    ReturnIntegerType {
        #[primary_span]
        span: Span,
        name: Symbol,
        ret_ty: Ty<'tcx>,
        out_ty: Ty<'tcx>,
    },

    #[diag(codegen_ssa_invalid_monomorphization_simd_shuffle, code = E0511)]
    SimdShuffle {
        #[primary_span]
        span: Span,
        name: Symbol,
        ty: Ty<'tcx>,
    },

    #[diag(codegen_ssa_invalid_monomorphization_return_length, code = E0511)]
    ReturnLength {
        #[primary_span]
        span: Span,
        name: Symbol,
        in_len: u64,
        ret_ty: Ty<'tcx>,
        out_len: u64,
    },

    #[diag(codegen_ssa_invalid_monomorphization_return_element, code = E0511)]
    ReturnElement {
        #[primary_span]
        span: Span,
        name: Symbol,
        in_elem: Ty<'tcx>,
        in_ty: Ty<'tcx>,
        ret_ty: Ty<'tcx>,
        out_ty: Ty<'tcx>,
    },

    #[diag(codegen_ssa_invalid_monomorphization_simd_index_out_of_bounds, code = E0511)]
    SimdIndexOutOfBounds {
        #[primary_span]
        span: Span,
        name: Symbol,
        arg_idx: u64,
        total_len: u128,
    },

    #[diag(codegen_ssa_invalid_monomorphization_inserted_type, code = E0511)]
    InsertedType {
        #[primary_span]
        span: Span,
        name: Symbol,
        in_elem: Ty<'tcx>,
        in_ty: Ty<'tcx>,
        out_ty: Ty<'tcx>,
    },

    #[diag(codegen_ssa_invalid_monomorphization_return_type, code = E0511)]
    ReturnType {
        #[primary_span]
        span: Span,
        name: Symbol,
        in_elem: Ty<'tcx>,
        in_ty: Ty<'tcx>,
        ret_ty: Ty<'tcx>,
    },

    #[diag(codegen_ssa_invalid_monomorphization_expected_return_type, code = E0511)]
    ExpectedReturnType {
        #[primary_span]
        span: Span,
        name: Symbol,
        in_ty: Ty<'tcx>,
        ret_ty: Ty<'tcx>,
    },

    #[diag(codegen_ssa_invalid_monomorphization_mismatched_lengths, code = E0511)]
    MismatchedLengths {
        #[primary_span]
        span: Span,
        name: Symbol,
        m_len: u64,
        v_len: u64,
    },

    #[diag(codegen_ssa_invalid_monomorphization_mask_type, code = E0511)]
    MaskType {
        #[primary_span]
        span: Span,
        name: Symbol,
        ty: Ty<'tcx>,
    },

    #[diag(codegen_ssa_invalid_monomorphization_vector_argument, code = E0511)]
    VectorArgument {
        #[primary_span]
        span: Span,
        name: Symbol,
        in_ty: Ty<'tcx>,
        in_elem: Ty<'tcx>,
    },

    #[diag(codegen_ssa_invalid_monomorphization_cannot_return, code = E0511)]
    CannotReturn {
        #[primary_span]
        span: Span,
        name: Symbol,
        ret_ty: Ty<'tcx>,
        expected_int_bits: u64,
        expected_bytes: u64,
    },

    #[diag(codegen_ssa_invalid_monomorphization_expected_element_type, code = E0511)]
    ExpectedElementType {
        #[primary_span]
        span: Span,
        name: Symbol,
        expected_element: Ty<'tcx>,
        second_arg: Ty<'tcx>,
        in_elem: Ty<'tcx>,
        in_ty: Ty<'tcx>,
        mutability: ExpectedPointerMutability,
    },

    #[diag(codegen_ssa_invalid_monomorphization_third_arg_element_type, code = E0511)]
    ThirdArgElementType {
        #[primary_span]
        span: Span,
        name: Symbol,
        expected_element: Ty<'tcx>,
        third_arg: Ty<'tcx>,
    },

    #[diag(codegen_ssa_invalid_monomorphization_unsupported_symbol_of_size, code = E0511)]
    UnsupportedSymbolOfSize {
        #[primary_span]
        span: Span,
        name: Symbol,
        symbol: Symbol,
        in_ty: Ty<'tcx>,
        in_elem: Ty<'tcx>,
        size: u64,
        ret_ty: Ty<'tcx>,
    },

    #[diag(codegen_ssa_invalid_monomorphization_unsupported_symbol, code = E0511)]
    UnsupportedSymbol {
        #[primary_span]
        span: Span,
        name: Symbol,
        symbol: Symbol,
        in_ty: Ty<'tcx>,
        in_elem: Ty<'tcx>,
        ret_ty: Ty<'tcx>,
    },

    #[diag(codegen_ssa_invalid_monomorphization_cast_fat_pointer, code = E0511)]
    CastFatPointer {
        #[primary_span]
        span: Span,
        name: Symbol,
        ty: Ty<'tcx>,
    },

    #[diag(codegen_ssa_invalid_monomorphization_expected_pointer, code = E0511)]
    ExpectedPointer {
        #[primary_span]
        span: Span,
        name: Symbol,
        ty: Ty<'tcx>,
    },

    #[diag(codegen_ssa_invalid_monomorphization_expected_usize, code = E0511)]
    ExpectedUsize {
        #[primary_span]
        span: Span,
        name: Symbol,
        ty: Ty<'tcx>,
    },

    #[diag(codegen_ssa_invalid_monomorphization_unsupported_cast, code = E0511)]
    UnsupportedCast {
        #[primary_span]
        span: Span,
        name: Symbol,
        in_ty: Ty<'tcx>,
        in_elem: Ty<'tcx>,
        ret_ty: Ty<'tcx>,
        out_elem: Ty<'tcx>,
    },

    #[diag(codegen_ssa_invalid_monomorphization_unsupported_operation, code = E0511)]
    UnsupportedOperation {
        #[primary_span]
        span: Span,
        name: Symbol,
        in_ty: Ty<'tcx>,
        in_elem: Ty<'tcx>,
    },

    #[diag(codegen_ssa_invalid_monomorphization_expected_vector_element_type, code = E0511)]
    ExpectedVectorElementType {
        #[primary_span]
        span: Span,
        name: Symbol,
        expected_element: Ty<'tcx>,
        vector_type: Ty<'tcx>,
    },
}

pub enum ExpectedPointerMutability {
    Mut,
    Not,
}

impl IntoDiagArg for ExpectedPointerMutability {
    fn into_diag_arg(self) -> DiagArgValue {
        match self {
            ExpectedPointerMutability::Mut => DiagArgValue::Str(Cow::Borrowed("*mut")),
            ExpectedPointerMutability::Not => DiagArgValue::Str(Cow::Borrowed("*_")),
        }
    }
}

#[derive(Diagnostic)]
#[diag(codegen_ssa_invalid_no_sanitize)]
#[note]
pub struct InvalidNoSanitize {
    #[primary_span]
    pub span: Span,
}

#[derive(Diagnostic)]
#[diag(codegen_ssa_invalid_link_ordinal_nargs)]
#[note]
pub struct InvalidLinkOrdinalNargs {
    #[primary_span]
    pub span: Span,
}

#[derive(Diagnostic)]
#[diag(codegen_ssa_illegal_link_ordinal_format)]
#[note]
pub struct InvalidLinkOrdinalFormat {
    #[primary_span]
    pub span: Span,
}

#[derive(Diagnostic)]
#[diag(codegen_ssa_target_feature_safe_trait)]
pub struct TargetFeatureSafeTrait {
    #[primary_span]
    #[label]
    pub span: Span,
    #[label(codegen_ssa_label_def)]
    pub def: Span,
}

#[derive(Diagnostic)]
#[diag(codegen_ssa_failed_to_get_layout)]
pub struct FailedToGetLayout<'tcx> {
    #[primary_span]
    pub span: Span,
    pub ty: Ty<'tcx>,
    pub err: LayoutError<'tcx>,
}

#[derive(Diagnostic)]
#[diag(codegen_ssa_error_creating_remark_dir)]
pub struct ErrorCreatingRemarkDir {
    pub error: std::io::Error,
}

#[derive(Diagnostic)]
#[diag(codegen_ssa_compiler_builtins_cannot_call)]
pub struct CompilerBuiltinsCannotCall {
    pub caller: String,
    pub callee: String,
}
