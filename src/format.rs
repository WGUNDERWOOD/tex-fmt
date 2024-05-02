use crate::indent::*;
use crate::subs::*;
use crate::wrap::*;

pub fn format_file(file: &str, debug: bool) -> String {
    let mut new_file = remove_extra_newlines(file);
    new_file = begin_end_environments_new_line(&new_file);
    new_file = remove_tabs(&new_file);
    new_file = remove_trailing_spaces(&new_file);
    new_file = apply_indent(&new_file, debug);
    //if needs_wrap(&file){
    //new_file = wrap(&new_file);
    //new_file = apply_indent(&new_file, debug);
    //};
    new_file
}
