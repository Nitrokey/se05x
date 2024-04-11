# Copyright (C) 2023 Nitrokey GmbH
# SPDX-License-Identifier: LGPL-3.0-only

import toml
import sys

def capitilize_first(name):
    name

def camel_case(name):
    parts = name.split("_")
    return "".join([part.title() for part in parts])

def data_for_arg(arg, name):
    if name == "then":
        return f'self.{arg["name"]}'

    if "value" in arg:
        return f'Tlv::new({name}, {arg["value"]})'
        
    if arg.get("optional", False):
        return f'self.{arg["name"]}.map(|data| Tlv::new({name}, data))'
    else:
        return f'Tlv::new({name}, self.{arg["name"]})'

def struct_ty_for_arg(arg, name):
    if name == "then":
        return f'{arg.get("type", DEFAULT_TYPE)}'
        
    if arg.get("optional", False):
        return f'Option<{arg.get("type", DEFAULT_TYPE)}>'
    else:
        return f'{arg.get("type", DEFAULT_TYPE)}'

def ty_for_arg(arg, name):
    if name == "then":
        return f'{arg.get("type", DEFAULT_TYPE)}'
        
    if arg.get("optional", False):
        return f'Option<Tlv<{arg.get("type", DEFAULT_TYPE)}>>'
    else:
        return f'Tlv<{arg.get("type", DEFAULT_TYPE)}>'

def ty_for_resp(arg):
    if arg.get("optional", False):
        return f'Option<{arg.get("type", DEFAULT_TYPE)}>'
    else:
        return f'{arg.get("type", DEFAULT_TYPE)}'

PARSE_PATTERN = """        let (%s, rem) = take_do_until(%s, rem)?;"""

PARSE_PATTERN_OPTIONAL = """        let (%s, rem) = take_opt_do_until(%s, %s, rem)?;"""

DEFAULT_TYPE = "&'data [u8]"

def parse_for_resp(arg, name, outfile, full_response):
    tab = " "*8
    ty = arg.get("type", None)
    conversion = "" 
    if ty is not None:
        conversion = ".try_into()?"

    if name == "then":
        outfile.write(f'{tab}let {arg["name"]} = rem{conversion};\n')
        return

    if arg.get("optional", False):
        next = [k for k in full_response.keys() if k != "then"]
        next = f"&[{','.join(next)}]"
        outfile.write(PARSE_PATTERN_OPTIONAL % (arg["name"], name, next))
    else:
        outfile.write(PARSE_PATTERN % (arg["name"], name))

def flatten(items):
    for arg_name, arg in items:
        if type(arg) is list:
            for elem in arg:
                yield arg_name, elem
        else:
            yield arg_name, arg

if len(sys.argv) != 3:
    print("Usage: ./generate_commands.py <toml data> <target file>")
    exit(1)

outfile = open(sys.argv[2], "w")
data = toml.load(sys.argv[1])

# REUSE-IgnoreStart
outfile.write("// Copyright (C) 2023 Nitrokey GmbH\n")
outfile.write("// SPDX-License-Identifier: LGPL-3.0-only\n\n")
# REUSE-IgnoreEnd

outfile.write("// Generated Automatically by `generate_commands.py DO NOT MODIFY MANUALLY\n\n")

outfile.write("use super::policies::*;\n")
outfile.write("use super::*;\n")

for command, v in data.items():
    name = camel_case(command) 

    outfile.write("\n")
    outfile.write(f'// ************* {name} ************* //\n')
    outfile.write("\n")

    cla = v["cla"]
    ins = v["ins"]
    p1 = v["p1"]
    p1_val = p1
    p2 = v["p2"]
    p2_val = p2
    le = v.get("le", 0)

    payload_has_lifetime = False
    for _, a in flatten(v["payload"].items()):
        if "type" not in a:
            payload_has_lifetime = True
            break
        if "'data" in a["type"]:
            payload_has_lifetime = True
            break

    response_has_lifetime = False
    if "response" in v:
        for a in v["response"].values():
            if "type" not in a:
                response_has_lifetime = True
                break
            if "'data" in a["type"]:
                response_has_lifetime = True
                break

    payload_lifetime = ""
    if payload_has_lifetime:
        payload_lifetime = "<'data>"

    response_lifetime = ""
    if response_has_lifetime:
        response_lifetime = "<'data>"


    outfile.write("#[derive(Clone, Debug)]\n")
    outfile.write("#[cfg_attr(feature = \"builder\", derive(typed_builder::TypedBuilder))]\n")
    outfile.write(f'pub struct {name}{payload_lifetime} {{\n')

    pre_ins = ""

    if v.get("maybe_transient", False):
        outfile.write("    #[cfg_attr(feature = \"builder\", builder(default))]\n")
        outfile.write("    pub transient: bool,\n")
        pre_ins = f'        let ins = if self.transient {{ {ins} | INS_TRANSIENT }} else {{ {ins} }};\n'
        ins = "ins"
    if v.get("maybe_auth", False):
        outfile.write("    #[cfg_attr(feature = \"builder\", builder(default))]\n")
        outfile.write("    pub is_auth: bool,\n")
        pre_ins += f'        let ins = if self.is_auth {{ {ins} | INS_AUTH_OBJECT }} else {{ {ins} }};\n'
        ins = "ins"
    if not isinstance(p1, str):
        outfile.write(f'    pub {p1["name"]}: {p1["type"]},\n')
        pre_ins += f'        let p1: u8 = self.{p1["name"]}.into();\n'
        p1_val = "p1"
    if not isinstance(p2, str):
        outfile.write(f'    pub {p2["name"]}: {p2["type"]},\n')
        pre_ins += f'        let p2: u8 = self.{p2["name"]}.into();\n'
        p2_val = "p2"

    if "maybe_p1_mask" in v:
        a = v["maybe_p1_mask"]
        outfile.write("    #[cfg_attr(feature = \"builder\", builder(default, setter(strip_option)))]\n")
        outfile.write(f'    pub {a["name"]}: Option<{a["type"]}>,\n')
        pre_ins += f'        let p1: u8 = self.{a["name"]}.map(|v| v | {p1_val} ).unwrap_or({p1});\n'
        p1_val = "p1"
    if "maybe_p2_mask" in v:
        a = v["maybe_p2_mask"]
        outfile.write("    #[cfg_attr(feature = \"builder\", builder(default, setter(strip_option)))]\n")
        outfile.write(f'    pub {a["name"]}: Option<{a["type"]}>,\n')
        pre_ins += f'        let p2: u8 = self.{a["name"]}.map(|v| v | {p2_val} ).unwrap_or({p2});\n'
        p2_val = "p2"

    arg_count = 0
    for arg_name, arg in flatten(v["payload"].items()):
        arg_count += 1
        if "value" in arg:
            continue
        if "comment" in arg:
            outfile.write(f'    /// {arg["comment"]}\n')
            outfile.write(f'    ///\n')
        if arg_name != "then":
            outfile.write(f'    /// Serialized to TLV tag [`{arg_name}`]()\n')
        else:
            outfile.write(f'    /// Serialized to remaining data\n')

        if "optional" in arg and arg["optional"] == True:
            outfile.write("    #[cfg_attr(feature = \"builder\", builder(default, setter(strip_option)))]\n")
        elif "default" in arg:
            outfile.write(f'    #[cfg_attr(feature = "builder", builder(default={arg["default"]}))]\n')
            
        outfile.write(f'    pub {arg["name"]}: {struct_ty_for_arg(arg,arg_name)},\n')
    outfile.write("}\n\n")

    slice_val_pre = " ".join([f'        let {arg["name"]} = &{data_for_arg(arg, name)};' for name, arg in flatten(v["payload"].items()) ])
    slice_val_inner = ", ".join([arg["name"] for name, arg in flatten(v["payload"].items())])
    slice_val = "&[" + slice_val_inner + "]"

    command_builder = f'CommandBuilder::new({cla}, {ins}, {p1_val}, {p2_val}, __data, {le})'

    outfile.write(f'impl{payload_lifetime} DataSource for {name}{payload_lifetime} {{\n')
    outfile.write('    fn len(&self) -> usize {\n')
    outfile.write(f'        {slice_val_pre}')
    outfile.write(f'        let __data: &[&dyn DataSource] = {slice_val};\n')
    if pre_ins != "": 
        outfile.write(f'{pre_ins}\n')
    outfile.write(f'       let command = {command_builder};\n')
    outfile.write('        command.len()\n')
    outfile.write('    }\n')
    outfile.write('    fn is_empty(&self) -> bool {\n')
    outfile.write('        // Command always has a header\n')
    outfile.write('        false\n')
    outfile.write('    }\n')
    outfile.write("}\n")

    bound = "<W: Writer>"
    if payload_has_lifetime:
        bound = "<'data, W: Writer>"
        

    outfile.write(f'impl{bound} DataStream<W> for {name}{payload_lifetime} {{\n')
    outfile.write('    fn to_writer(&self, writer: &mut W) -> Result<(), <W as iso7816::command::Writer>::Error> {\n')
    outfile.write(f'        {slice_val_pre}')
    outfile.write(f'        let __data: &[&dyn DataStream<W>] = {slice_val};\n')
    if pre_ins != "": 
        outfile.write(f'{pre_ins}\n')
    outfile.write(f'       let command = {command_builder};\n')
    outfile.write('        command.to_writer(writer)\n')
    outfile.write('    }\n')
    outfile.write("}\n")
   
    if "response" in v:
        outfile.write("#[derive(Clone, Debug)]\n")
        outfile.write(f'pub struct {name}Response{response_lifetime} {{\n')

        for arg_name, arg in v["response"].items():
            if "comment" in arg:
                outfile.write(f'    /// {arg["comment"]}\n')
                outfile.write(f'    ///\n')
            if arg_name != "then":
                outfile.write(f'    /// Parsed from TLV tag [`{arg_name}`]()\n')
            else:
                outfile.write(f'    /// Parsed from remaining data\n')
            outfile.write(f'    pub {arg["name"]}: {ty_for_resp(arg)},\n')
        outfile.write("}\n")

        outfile.write(f'\nimpl<\'data> Se05XResponse<\'data> for {name}Response{response_lifetime} {{\n')
        outfile.write("    #[inline(never)]\n")
        outfile.write("    fn from_response(rem: &'data [u8]) -> Result<Self, Error> {\n")
        for arg_name, arg in v["response"].items():
            parse_for_resp(arg, arg_name, outfile, v["response"])
        outfile.write("        let _ = rem;\n")
        outfile.write(f'        Ok(Self {{ {", ".join([arg["name"] for arg in v["response"].values()])} }})\n')
        outfile.write("    }\n")
        outfile.write("}\n")

    outfile.write("\n")
    outfile.write(f'impl{bound} Se05XCommand<W> for {name}{payload_lifetime} {{\n')
    if "response" not in v: 
        outfile.write(f'    type Response<\'rdata> = ();\n')
    elif response_has_lifetime:
        outfile.write(f'    type Response<\'rdata> = {name}Response<\'rdata>;\n')
    else:
        outfile.write(f'    type Response<\'rdata> = {name}Response;\n')
    outfile.write("}\n")


outfile.flush()
