// Auto generated 2016-03-15 11:32:56.533000

use neovim::*;
use rmp::Value;
use rpc::*;

pub struct Buffer {
    code_data: Value,
}

impl Buffer {
    pub fn new(code_data: Value) -> Buffer {
        Buffer { code_data: code_data }
    }

    pub fn line_count(&self, neovim: &mut Neovim) -> Result<u64, String> {
        neovim.session
              .call("buffer_line_count", &call_args![self.code_data.clone()])
              .map(map_result)
              .map_err(map_generic_error)
    }
    pub fn get_line(&self, neovim: &mut Neovim, index: u64) -> Result<String, String> {
        neovim.session
              .call("buffer_get_line",
                    &call_args![self.code_data.clone(), index])
              .map(map_result)
              .map_err(map_generic_error)
    }
    pub fn set_line(&self, neovim: &mut Neovim, index: u64, line: &str) -> Result<(), String> {
        neovim.session
              .call("buffer_set_line",
                    &call_args![self.code_data.clone(), index, line])
              .map(map_result)
              .map_err(map_generic_error)
    }
    pub fn del_line(&self, neovim: &mut Neovim, index: u64) -> Result<(), String> {
        neovim.session
              .call("buffer_del_line",
                    &call_args![self.code_data.clone(), index])
              .map(map_result)
              .map_err(map_generic_error)
    }
    pub fn get_line_slice(&self,
                          neovim: &mut Neovim,
                          start: u64,
                          end: u64,
                          include_start: bool,
                          include_end: bool)
                          -> Result<Vec<String>, String> {
        neovim.session
              .call("buffer_get_line_slice",
                    &call_args![self.code_data.clone(), start, end, include_start, include_end])
              .map(map_result)
              .map_err(map_generic_error)
    }
    pub fn set_line_slice(&self,
                          neovim: &mut Neovim,
                          start: u64,
                          end: u64,
                          include_start: bool,
                          include_end: bool,
                          replacement: Vec<String>)
                          -> Result<(), String> {
        neovim.session
              .call("buffer_set_line_slice",
                    &call_args![self.code_data.clone(),
                                start,
                                end,
                                include_start,
                                include_end,
                                replacement])
              .map(map_result)
              .map_err(map_generic_error)
    }
    pub fn get_var(&self, neovim: &mut Neovim, name: &str) -> Result<Value, String> {
        neovim.session
              .call("buffer_get_var", &call_args![self.code_data.clone(), name])
              .map(map_result)
              .map_err(map_generic_error)
    }
    pub fn set_var(&self, neovim: &mut Neovim, name: &str, value: Value) -> Result<Value, String> {
        neovim.session
              .call("buffer_set_var",
                    &call_args![self.code_data.clone(), name, value])
              .map(map_result)
              .map_err(map_generic_error)
    }
    pub fn get_option(&self, neovim: &mut Neovim, name: &str) -> Result<Value, String> {
        neovim.session
              .call("buffer_get_option",
                    &call_args![self.code_data.clone(), name])
              .map(map_result)
              .map_err(map_generic_error)
    }
    pub fn set_option(&self, neovim: &mut Neovim, name: &str, value: Value) -> Result<(), String> {
        neovim.session
              .call("buffer_set_option",
                    &call_args![self.code_data.clone(), name, value])
              .map(map_result)
              .map_err(map_generic_error)
    }
    pub fn get_number(&self, neovim: &mut Neovim) -> Result<u64, String> {
        neovim.session
              .call("buffer_get_number", &call_args![self.code_data.clone()])
              .map(map_result)
              .map_err(map_generic_error)
    }
    pub fn get_name(&self, neovim: &mut Neovim) -> Result<String, String> {
        neovim.session
              .call("buffer_get_name", &call_args![self.code_data.clone()])
              .map(map_result)
              .map_err(map_generic_error)
    }
    pub fn set_name(&self, neovim: &mut Neovim, name: &str) -> Result<(), String> {
        neovim.session
              .call("buffer_set_name", &call_args![self.code_data.clone(), name])
              .map(map_result)
              .map_err(map_generic_error)
    }
    pub fn is_valid(&self, neovim: &mut Neovim) -> Result<bool, String> {
        neovim.session
              .call("buffer_is_valid", &call_args![self.code_data.clone()])
              .map(map_result)
              .map_err(map_generic_error)
    }
    pub fn insert(&self, neovim: &mut Neovim, lnum: u64, lines: Vec<String>) -> Result<(), String> {
        neovim.session
              .call("buffer_insert",
                    &call_args![self.code_data.clone(), lnum, lines])
              .map(map_result)
              .map_err(map_generic_error)
    }
    pub fn get_mark(&self, neovim: &mut Neovim, name: &str) -> Result<(u64, u64), String> {
        neovim.session
              .call("buffer_get_mark", &call_args![self.code_data.clone(), name])
              .map(map_result)
              .map_err(map_generic_error)
    }
    pub fn add_highlight(&self,
                         neovim: &mut Neovim,
                         src_id: u64,
                         hl_group: &str,
                         line: u64,
                         col_start: u64,
                         col_end: u64)
                         -> Result<u64, String> {
        neovim.session
              .call("buffer_add_highlight",
                    &call_args![self.code_data.clone(), src_id, hl_group, line, col_start, col_end])
              .map(map_result)
              .map_err(map_generic_error)
    }
    pub fn clear_highlight(&self,
                           neovim: &mut Neovim,
                           src_id: u64,
                           line_start: u64,
                           line_end: u64)
                           -> Result<(), String> {
        neovim.session
              .call("buffer_clear_highlight",
                    &call_args![self.code_data.clone(), src_id, line_start, line_end])
              .map(map_result)
              .map_err(map_generic_error)
    }
}

pub struct Window {
    code_data: Value,
}

impl Window {
    pub fn new(code_data: Value) -> Window {
        Window { code_data: code_data }
    }

    pub fn get_buffer(&self, neovim: &mut Neovim) -> Result<Buffer, String> {
        neovim.session
              .call("window_get_buffer", &call_args![self.code_data.clone()])
              .map(map_result)
              .map_err(map_generic_error)
    }
    pub fn get_cursor(&self, neovim: &mut Neovim) -> Result<(u64, u64), String> {
        neovim.session
              .call("window_get_cursor", &call_args![self.code_data.clone()])
              .map(map_result)
              .map_err(map_generic_error)
    }
    pub fn set_cursor(&self, neovim: &mut Neovim, pos: (u64, u64)) -> Result<(), String> {
        neovim.session
              .call("window_set_cursor",
                    &call_args![self.code_data.clone(), pos])
              .map(map_result)
              .map_err(map_generic_error)
    }
    pub fn get_height(&self, neovim: &mut Neovim) -> Result<u64, String> {
        neovim.session
              .call("window_get_height", &call_args![self.code_data.clone()])
              .map(map_result)
              .map_err(map_generic_error)
    }
    pub fn set_height(&self, neovim: &mut Neovim, height: u64) -> Result<(), String> {
        neovim.session
              .call("window_set_height",
                    &call_args![self.code_data.clone(), height])
              .map(map_result)
              .map_err(map_generic_error)
    }
    pub fn get_width(&self, neovim: &mut Neovim) -> Result<u64, String> {
        neovim.session
              .call("window_get_width", &call_args![self.code_data.clone()])
              .map(map_result)
              .map_err(map_generic_error)
    }
    pub fn set_width(&self, neovim: &mut Neovim, width: u64) -> Result<(), String> {
        neovim.session
              .call("window_set_width",
                    &call_args![self.code_data.clone(), width])
              .map(map_result)
              .map_err(map_generic_error)
    }
    pub fn get_var(&self, neovim: &mut Neovim, name: &str) -> Result<Value, String> {
        neovim.session
              .call("window_get_var", &call_args![self.code_data.clone(), name])
              .map(map_result)
              .map_err(map_generic_error)
    }
    pub fn set_var(&self, neovim: &mut Neovim, name: &str, value: Value) -> Result<Value, String> {
        neovim.session
              .call("window_set_var",
                    &call_args![self.code_data.clone(), name, value])
              .map(map_result)
              .map_err(map_generic_error)
    }
    pub fn get_option(&self, neovim: &mut Neovim, name: &str) -> Result<Value, String> {
        neovim.session
              .call("window_get_option",
                    &call_args![self.code_data.clone(), name])
              .map(map_result)
              .map_err(map_generic_error)
    }
    pub fn set_option(&self, neovim: &mut Neovim, name: &str, value: Value) -> Result<(), String> {
        neovim.session
              .call("window_set_option",
                    &call_args![self.code_data.clone(), name, value])
              .map(map_result)
              .map_err(map_generic_error)
    }
    pub fn get_position(&self, neovim: &mut Neovim) -> Result<(u64, u64), String> {
        neovim.session
              .call("window_get_position", &call_args![self.code_data.clone()])
              .map(map_result)
              .map_err(map_generic_error)
    }
    pub fn get_tabpage(&self, neovim: &mut Neovim) -> Result<Tabpage, String> {
        neovim.session
              .call("window_get_tabpage", &call_args![self.code_data.clone()])
              .map(map_result)
              .map_err(map_generic_error)
    }
    pub fn is_valid(&self, neovim: &mut Neovim) -> Result<bool, String> {
        neovim.session
              .call("window_is_valid", &call_args![self.code_data.clone()])
              .map(map_result)
              .map_err(map_generic_error)
    }
}

pub struct Tabpage {
    code_data: Value,
}

impl Tabpage {
    pub fn new(code_data: Value) -> Tabpage {
        Tabpage { code_data: code_data }
    }

    pub fn get_windows(&self, neovim: &mut Neovim) -> Result<Vec<Window>, String> {
        neovim.session
              .call("tabpage_get_windows", &call_args![self.code_data.clone()])
              .map(map_result)
              .map_err(map_generic_error)
    }
    pub fn get_var(&self, neovim: &mut Neovim, name: &str) -> Result<Value, String> {
        neovim.session
              .call("tabpage_get_var", &call_args![self.code_data.clone(), name])
              .map(map_result)
              .map_err(map_generic_error)
    }
    pub fn set_var(&self, neovim: &mut Neovim, name: &str, value: Value) -> Result<Value, String> {
        neovim.session
              .call("tabpage_set_var",
                    &call_args![self.code_data.clone(), name, value])
              .map(map_result)
              .map_err(map_generic_error)
    }
    pub fn get_window(&self, neovim: &mut Neovim) -> Result<Window, String> {
        neovim.session
              .call("tabpage_get_window", &call_args![self.code_data.clone()])
              .map(map_result)
              .map_err(map_generic_error)
    }
    pub fn is_valid(&self, neovim: &mut Neovim) -> Result<bool, String> {
        neovim.session
              .call("tabpage_is_valid", &call_args![self.code_data.clone()])
              .map(map_result)
              .map_err(map_generic_error)
    }
}


impl FromVal<Value> for Buffer {
    fn from_val(val: Value) -> Self {
        Buffer::new(val)
    }
}

impl<'a> IntoVal<Value> for &'a Buffer {
    fn into_val(self) -> Value {
        self.code_data.clone()
    }
}
impl FromVal<Value> for Window {
    fn from_val(val: Value) -> Self {
        Window::new(val)
    }
}

impl<'a> IntoVal<Value> for &'a Window {
    fn into_val(self) -> Value {
        self.code_data.clone()
    }
}
impl FromVal<Value> for Tabpage {
    fn from_val(val: Value) -> Self {
        Tabpage::new(val)
    }
}

impl<'a> IntoVal<Value> for &'a Tabpage {
    fn into_val(self) -> Value {
        self.code_data.clone()
    }
}

pub trait NeovimApi {
    fn command(&mut self, str: &str) -> Result<(), String>;
    fn feedkeys(&mut self, keys: &str, mode: &str, escape_csi: bool) -> Result<(), String>;
    fn input(&mut self, keys: &str) -> Result<u64, String>;
    fn replace_termcodes(&mut self,
                         str: &str,
                         from_part: bool,
                         do_lt: bool,
                         special: bool)
                         -> Result<String, String>;
    fn command_output(&mut self, str: &str) -> Result<String, String>;
    fn eval(&mut self, str: &str) -> Result<Value, String>;
    fn call_function(&mut self, fname: &str, args: Vec<Value>) -> Result<Value, String>;
    fn strwidth(&mut self, str: &str) -> Result<u64, String>;
    fn list_runtime_paths(&mut self) -> Result<Vec<String>, String>;
    fn change_directory(&mut self, dir: &str) -> Result<(), String>;
    fn get_current_line(&mut self) -> Result<String, String>;
    fn set_current_line(&mut self, line: &str) -> Result<(), String>;
    fn del_current_line(&mut self) -> Result<(), String>;
    fn get_var(&mut self, name: &str) -> Result<Value, String>;
    fn set_var(&mut self, name: &str, value: Value) -> Result<Value, String>;
    fn get_vvar(&mut self, name: &str) -> Result<Value, String>;
    fn get_option(&mut self, name: &str) -> Result<Value, String>;
    fn set_option(&mut self, name: &str, value: Value) -> Result<(), String>;
    fn out_write(&mut self, str: &str) -> Result<(), String>;
    fn err_write(&mut self, str: &str) -> Result<(), String>;
    fn report_error(&mut self, str: &str) -> Result<(), String>;
    fn get_buffers(&mut self) -> Result<Vec<Buffer>, String>;
    fn get_current_buffer(&mut self) -> Result<Buffer, String>;
    fn set_current_buffer(&mut self, buffer: &Buffer) -> Result<(), String>;
    fn get_windows(&mut self) -> Result<Vec<Window>, String>;
    fn get_current_window(&mut self) -> Result<Window, String>;
    fn set_current_window(&mut self, window: &Window) -> Result<(), String>;
    fn get_tabpages(&mut self) -> Result<Vec<Tabpage>, String>;
    fn get_current_tabpage(&mut self) -> Result<Tabpage, String>;
    fn set_current_tabpage(&mut self, tabpage: &Tabpage) -> Result<(), String>;
    fn subscribe(&mut self, event: &str) -> Result<(), String>;
    fn unsubscribe(&mut self, event: &str) -> Result<(), String>;
    fn name_to_color(&mut self, name: &str) -> Result<u64, String>;
    fn get_api_info(&mut self) -> Result<Vec<Value>, String>;
    }

impl NeovimApi for Neovim {
    fn command(&mut self, str: &str) -> Result<(), String> {
        self.session
            .call("vim_command", &call_args![str])
            .map(map_result)
            .map_err(map_generic_error)
    }

    fn feedkeys(&mut self, keys: &str, mode: &str, escape_csi: bool) -> Result<(), String> {
        self.session
            .call("vim_feedkeys", &call_args![keys, mode, escape_csi])
            .map(map_result)
            .map_err(map_generic_error)
    }

    fn input(&mut self, keys: &str) -> Result<u64, String> {
        self.session
            .call("vim_input", &call_args![keys])
            .map(map_result)
            .map_err(map_generic_error)
    }

    fn replace_termcodes(&mut self,
                         str: &str,
                         from_part: bool,
                         do_lt: bool,
                         special: bool)
                         -> Result<String, String> {
        self.session
            .call("vim_replace_termcodes",
                  &call_args![str, from_part, do_lt, special])
            .map(map_result)
            .map_err(map_generic_error)
    }

    fn command_output(&mut self, str: &str) -> Result<String, String> {
        self.session
            .call("vim_command_output", &call_args![str])
            .map(map_result)
            .map_err(map_generic_error)
    }

    fn eval(&mut self, str: &str) -> Result<Value, String> {
        self.session
            .call("vim_eval", &call_args![str])
            .map(map_result)
            .map_err(map_generic_error)
    }

    fn call_function(&mut self, fname: &str, args: Vec<Value>) -> Result<Value, String> {
        self.session
            .call("vim_call_function", &call_args![fname, args])
            .map(map_result)
            .map_err(map_generic_error)
    }

    fn strwidth(&mut self, str: &str) -> Result<u64, String> {
        self.session
            .call("vim_strwidth", &call_args![str])
            .map(map_result)
            .map_err(map_generic_error)
    }

    fn list_runtime_paths(&mut self) -> Result<Vec<String>, String> {
        self.session
            .call("vim_list_runtime_paths", &call_args![])
            .map(map_result)
            .map_err(map_generic_error)
    }

    fn change_directory(&mut self, dir: &str) -> Result<(), String> {
        self.session
            .call("vim_change_directory", &call_args![dir])
            .map(map_result)
            .map_err(map_generic_error)
    }

    fn get_current_line(&mut self) -> Result<String, String> {
        self.session
            .call("vim_get_current_line", &call_args![])
            .map(map_result)
            .map_err(map_generic_error)
    }

    fn set_current_line(&mut self, line: &str) -> Result<(), String> {
        self.session
            .call("vim_set_current_line", &call_args![line])
            .map(map_result)
            .map_err(map_generic_error)
    }

    fn del_current_line(&mut self) -> Result<(), String> {
        self.session
            .call("vim_del_current_line", &call_args![])
            .map(map_result)
            .map_err(map_generic_error)
    }

    fn get_var(&mut self, name: &str) -> Result<Value, String> {
        self.session
            .call("vim_get_var", &call_args![name])
            .map(map_result)
            .map_err(map_generic_error)
    }

    fn set_var(&mut self, name: &str, value: Value) -> Result<Value, String> {
        self.session
            .call("vim_set_var", &call_args![name, value])
            .map(map_result)
            .map_err(map_generic_error)
    }

    fn get_vvar(&mut self, name: &str) -> Result<Value, String> {
        self.session
            .call("vim_get_vvar", &call_args![name])
            .map(map_result)
            .map_err(map_generic_error)
    }

    fn get_option(&mut self, name: &str) -> Result<Value, String> {
        self.session
            .call("vim_get_option", &call_args![name])
            .map(map_result)
            .map_err(map_generic_error)
    }

    fn set_option(&mut self, name: &str, value: Value) -> Result<(), String> {
        self.session
            .call("vim_set_option", &call_args![name, value])
            .map(map_result)
            .map_err(map_generic_error)
    }

    fn out_write(&mut self, str: &str) -> Result<(), String> {
        self.session
            .call("vim_out_write", &call_args![str])
            .map(map_result)
            .map_err(map_generic_error)
    }

    fn err_write(&mut self, str: &str) -> Result<(), String> {
        self.session
            .call("vim_err_write", &call_args![str])
            .map(map_result)
            .map_err(map_generic_error)
    }

    fn report_error(&mut self, str: &str) -> Result<(), String> {
        self.session
            .call("vim_report_error", &call_args![str])
            .map(map_result)
            .map_err(map_generic_error)
    }

    fn get_buffers(&mut self) -> Result<Vec<Buffer>, String> {
        self.session
            .call("vim_get_buffers", &call_args![])
            .map(map_result)
            .map_err(map_generic_error)
    }

    fn get_current_buffer(&mut self) -> Result<Buffer, String> {
        self.session
            .call("vim_get_current_buffer", &call_args![])
            .map(map_result)
            .map_err(map_generic_error)
    }

    fn set_current_buffer(&mut self, buffer: &Buffer) -> Result<(), String> {
        self.session
            .call("vim_set_current_buffer", &call_args![buffer])
            .map(map_result)
            .map_err(map_generic_error)
    }

    fn get_windows(&mut self) -> Result<Vec<Window>, String> {
        self.session
            .call("vim_get_windows", &call_args![])
            .map(map_result)
            .map_err(map_generic_error)
    }

    fn get_current_window(&mut self) -> Result<Window, String> {
        self.session
            .call("vim_get_current_window", &call_args![])
            .map(map_result)
            .map_err(map_generic_error)
    }

    fn set_current_window(&mut self, window: &Window) -> Result<(), String> {
        self.session
            .call("vim_set_current_window", &call_args![window])
            .map(map_result)
            .map_err(map_generic_error)
    }

    fn get_tabpages(&mut self) -> Result<Vec<Tabpage>, String> {
        self.session
            .call("vim_get_tabpages", &call_args![])
            .map(map_result)
            .map_err(map_generic_error)
    }

    fn get_current_tabpage(&mut self) -> Result<Tabpage, String> {
        self.session
            .call("vim_get_current_tabpage", &call_args![])
            .map(map_result)
            .map_err(map_generic_error)
    }

    fn set_current_tabpage(&mut self, tabpage: &Tabpage) -> Result<(), String> {
        self.session
            .call("vim_set_current_tabpage", &call_args![tabpage])
            .map(map_result)
            .map_err(map_generic_error)
    }

    fn subscribe(&mut self, event: &str) -> Result<(), String> {
        self.session
            .call("vim_subscribe", &call_args![event])
            .map(map_result)
            .map_err(map_generic_error)
    }

    fn unsubscribe(&mut self, event: &str) -> Result<(), String> {
        self.session
            .call("vim_unsubscribe", &call_args![event])
            .map(map_result)
            .map_err(map_generic_error)
    }

    fn name_to_color(&mut self, name: &str) -> Result<u64, String> {
        self.session
            .call("vim_name_to_color", &call_args![name])
            .map(map_result)
            .map_err(map_generic_error)
    }

    fn get_api_info(&mut self) -> Result<Vec<Value>, String> {
        self.session
            .call("vim_get_api_info", &call_args![])
            .map(map_result)
            .map_err(map_generic_error)
    }
}
