complete -c tex-fmt -s l -l wraplen -d 'Line length for wrapping [default: 80]' -r
complete -c tex-fmt -s t -l tabsize -d 'Number of characters to use as tab size [default: 2]' -r
complete -c tex-fmt -l config -d 'Path to configuration file' -r -F
complete -c tex-fmt -l completion -d 'Generate shell completion script' -r -f -a "{bash\t'',elvish\t'',fish\t'',powershell\t'',zsh\t''}"
complete -c tex-fmt -s c -l check -d 'Check formatting, do not modify files'
complete -c tex-fmt -s p -l print -d 'Print to stdout, do not modify files'
complete -c tex-fmt -s n -l nowrap -d 'Do not wrap long lines'
complete -c tex-fmt -l usetabs -d 'Use tabs instead of spaces for indentation'
complete -c tex-fmt -s s -l stdin -d 'Process stdin as a single file, output to stdout'
complete -c tex-fmt -l noconfig -d 'Do not read any config file'
complete -c tex-fmt -s v -l verbose -d 'Show info messages'
complete -c tex-fmt -s q -l quiet -d 'Hide warning messages'
complete -c tex-fmt -l trace -d 'Show trace messages'
complete -c tex-fmt -l man -d 'Generate man page'
complete -c tex-fmt -l args -d 'Print arguments passed to tex-fmt and exit'
complete -c tex-fmt -s h -l help -d 'Print help'
complete -c tex-fmt -s V -l version -d 'Print version'
