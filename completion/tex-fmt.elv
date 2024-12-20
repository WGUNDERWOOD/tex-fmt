
use builtin;
use str;

set edit:completion:arg-completer[tex-fmt] = {|@words|
    fn spaces {|n|
        builtin:repeat $n ' ' | str:join ''
    }
    fn cand {|text desc|
        edit:complex-candidate $text &display=$text' '(spaces (- 14 (wcswidth $text)))$desc
    }
    var command = 'tex-fmt'
    for word $words[1..-1] {
        if (str:has-prefix $word '-') {
            break
        }
        set command = $command';'$word
    }
    var completions = [
        &'tex-fmt'= {
            cand -l 'Line length for wrapping [default: 80]'
            cand --wraplen 'Line length for wrapping [default: 80]'
            cand -t 'Number of characters to use as tab size [default: 2]'
            cand --tabsize 'Number of characters to use as tab size [default: 2]'
            cand --config 'Path to configuration file'
            cand --completion 'Generate shell completion script'
            cand -c 'Check formatting, do not modify files'
            cand --check 'Check formatting, do not modify files'
            cand -p 'Print to stdout, do not modify files'
            cand --print 'Print to stdout, do not modify files'
            cand -n 'Do not wrap long lines'
            cand --nowrap 'Do not wrap long lines'
            cand --usetabs 'Use tabs instead of spaces for indentation'
            cand -s 'Process stdin as a single file, output to stdout'
            cand --stdin 'Process stdin as a single file, output to stdout'
            cand --noconfig 'Do not read any config file'
            cand -v 'Show info messages'
            cand --verbose 'Show info messages'
            cand -q 'Hide warning messages'
            cand --quiet 'Hide warning messages'
            cand --trace 'Show trace messages'
            cand --man 'Generate man page'
            cand --args 'Print arguments passed to tex-fmt and exit'
            cand -h 'Print help'
            cand --help 'Print help'
            cand -V 'Print version'
            cand --version 'Print version'
        }
    ]
    $completions[$command]
}
