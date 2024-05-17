enum Error {
    kind: ErrorKind,
    message: String,
    linum: Option<usize>,
    filename: String,
}

enum ErrorKind {
    FileExtensionError,
    FormatFileInfo,
    IndentFileInfo,
    IndentLineInfo,
    IndentNegativeError,
    IndentZeroError,
    IgnoreEndError,
    IgnoreStartError,
    WrapFileInfo,
    WrapLineInfo,
    WrapLineError,
}
