
using namespace System.Management.Automation
using namespace System.Management.Automation.Language

Register-ArgumentCompleter -Native -CommandName 'tex-fmt' -ScriptBlock {
    param($wordToComplete, $commandAst, $cursorPosition)

    $commandElements = $commandAst.CommandElements
    $command = @(
        'tex-fmt'
        for ($i = 1; $i -lt $commandElements.Count; $i++) {
            $element = $commandElements[$i]
            if ($element -isnot [StringConstantExpressionAst] -or
                $element.StringConstantType -ne [StringConstantType]::BareWord -or
                $element.Value.StartsWith('-') -or
                $element.Value -eq $wordToComplete) {
                break
        }
        $element.Value
    }) -join ';'

    $completions = @(switch ($command) {
        'tex-fmt' {
            [CompletionResult]::new('-l', '-l', [CompletionResultType]::ParameterName, 'Line length for wrapping [default: 80]')
            [CompletionResult]::new('--wraplen', '--wraplen', [CompletionResultType]::ParameterName, 'Line length for wrapping [default: 80]')
            [CompletionResult]::new('-t', '-t', [CompletionResultType]::ParameterName, 'Number of characters to use as tab size [default: 2]')
            [CompletionResult]::new('--tabsize', '--tabsize', [CompletionResultType]::ParameterName, 'Number of characters to use as tab size [default: 2]')
            [CompletionResult]::new('--config', '--config', [CompletionResultType]::ParameterName, 'Path to configuration file')
            [CompletionResult]::new('--completion', '--completion', [CompletionResultType]::ParameterName, 'Generate shell completion script')
            [CompletionResult]::new('-c', '-c', [CompletionResultType]::ParameterName, 'Check formatting, do not modify files')
            [CompletionResult]::new('--check', '--check', [CompletionResultType]::ParameterName, 'Check formatting, do not modify files')
            [CompletionResult]::new('-p', '-p', [CompletionResultType]::ParameterName, 'Print to stdout, do not modify files')
            [CompletionResult]::new('--print', '--print', [CompletionResultType]::ParameterName, 'Print to stdout, do not modify files')
            [CompletionResult]::new('-n', '-n', [CompletionResultType]::ParameterName, 'Do not wrap long lines')
            [CompletionResult]::new('--nowrap', '--nowrap', [CompletionResultType]::ParameterName, 'Do not wrap long lines')
            [CompletionResult]::new('--usetabs', '--usetabs', [CompletionResultType]::ParameterName, 'Use tabs instead of spaces for indentation')
            [CompletionResult]::new('-s', '-s', [CompletionResultType]::ParameterName, 'Process stdin as a single file, output to stdout')
            [CompletionResult]::new('--stdin', '--stdin', [CompletionResultType]::ParameterName, 'Process stdin as a single file, output to stdout')
            [CompletionResult]::new('--noconfig', '--noconfig', [CompletionResultType]::ParameterName, 'Do not read any config file')
            [CompletionResult]::new('-v', '-v', [CompletionResultType]::ParameterName, 'Show info messages')
            [CompletionResult]::new('--verbose', '--verbose', [CompletionResultType]::ParameterName, 'Show info messages')
            [CompletionResult]::new('-q', '-q', [CompletionResultType]::ParameterName, 'Hide warning messages')
            [CompletionResult]::new('--quiet', '--quiet', [CompletionResultType]::ParameterName, 'Hide warning messages')
            [CompletionResult]::new('--trace', '--trace', [CompletionResultType]::ParameterName, 'Show trace messages')
            [CompletionResult]::new('--man', '--man', [CompletionResultType]::ParameterName, 'Generate man page')
            [CompletionResult]::new('--args', '--args', [CompletionResultType]::ParameterName, 'Print arguments passed to tex-fmt and exit')
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('-V', '-V ', [CompletionResultType]::ParameterName, 'Print version')
            [CompletionResult]::new('--version', '--version', [CompletionResultType]::ParameterName, 'Print version')
            break
        }
    })

    $completions.Where{ $_.CompletionText -like "$wordToComplete*" } |
        Sort-Object -Property ListItemText
}
