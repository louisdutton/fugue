#!/usr/bin/env fish
# Fish completion script for Fugue editor

complete -c fugue -s h -l help -d "Prints help information"
complete -c fugue -l tutor -d "Loads the tutorial"
complete -c fugue -l health -xa "(__fugue_langs_ops)" -d "Checks for errors"
complete -c fugue -l health -xka all -d "Prints all diagnostic informations"
complete -c fugue -l health -xka all-languages -d "Lists all languages"
complete -c fugue -l health -xka languages -d "Lists user configured languages"
complete -c fugue -l health -xka clipboard -d "Prints system clipboard provider"
complete -c fugue -s g -l grammar -x -a "fetch build" -d "Fetch or build tree-sitter grammars"
complete -c fugue -s v -o vv -o vvv -d "Increases logging verbosity"
complete -c fugue -s V -l version -d "Prints version information"
complete -c fugue -l vsplit -d "Splits all given files vertically"
complete -c fugue -l hsplit -d "Splits all given files horizontally"
complete -c fugue -s c -l config -r -d "Specifies a file to use for config"
complete -c fugue -l log -r -d "Specifies a file to use for logging"
complete -c fugue -s w -l working-dir -d "Specify initial working directory" -xa "(__fish_complete_directories)"

function __fugue_langs_ops
    fugue --health all-languages | tail -n '+2' | string replace -fr '^(\S+) .*' '$1'
end
