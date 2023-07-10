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
        
    if data.get("optional", False):
        return f'self.{arg["name"]}.map(|data| Tlv::new({name}, data))'
    else:
        return f'Tlv::new({name}, self.{arg["name"]})'

def ty_for_arg(arg, name):
    if name == "then":
        return f'{arg["type"]}'
        
    if data.get("optional", False):
        return f'Option<Tlv<{arg["type"]}>>'
    else:
        return f'Tlv<{arg["type"]}>'

PARSE_PATTERN = """
        let (%s, rem) = loop {
            let mut rem_inner = rem;
            let (tag, value, r) = take_do(rem_inner).ok_or(Error::Tlv)?;
            rem_inner = r;
            if tag == %s {
                break (value.try_into()?, rem_inner);
            }
        };
"""

def parse_for_resp(arg, name, outfile):
    tab = " "*8
    if name == "then":
        outfile.write(f'{tab}let {arg["name"]} = rem;\n')
        return

    outfile.write(PARSE_PATTERN % (arg["name"], name))



if len(sys.argv) != 3:
    print("Usage: ./generate_commands.py <toml data> <target file>")
    exit(1)

outfile = open(sys.argv[2], "w")
data = toml.load(sys.argv[1])


outfile.write("// Generated Automatically by `generate_commands.py DO NOT MODIFY MANUALLY\n\n")

outfile.write("use super::policies::*;\n")
outfile.write("use super::*;\n")
outfile.write("use iso7816::command::{CommandBuilder, ExpectedLen};\n")
outfile.write("use iso7816::tlv::{Tlv, take_do};\n")

for command, v in data.items():
    outfile.write("\n")
    name = camel_case(command) 
    payload_has_lifetime = v.get("contains_arbitrary_payload", True)
    response_has_lifetime = v.get("contains_arbitrary_response", True)
    payload_lifetime = ""
    if payload_has_lifetime:
        payload_lifetime = "<'data>"

    response_lifetime = ""
    if response_has_lifetime:
        response_lifetime = "<'data>"

    outfile.write("#[derive(Clone, Debug)]")
    outfile.write(f'pub struct {name}{payload_lifetime} {{')

    if "payload" in v:
        outfile.write("\n")

    for arg in v["payload"].values():
        outfile.write(f'    pub {arg["name"]}: {arg["type"]},\n')
    outfile.write("}\n\n")

    if payload_has_lifetime:
        outfile.write(f'impl<\'data> {name}<\'data> {{\n')
    else:
        outfile.write(f'impl {name} {{\n')

    tup_ty = ", ".join([ty_for_arg(arg,name) for name, arg in v["payload"].items()])
    tup_val = ", ".join([data_for_arg(arg,name) for name, arg in v["payload"].items()])
    if len(v["payload"].values()) != 1:
        tup_ty = f'({tup_ty})'
        tup_val = f'({tup_val})'
    cla = v["cla"]
    ins = v["ins"]
    p1 = v["p1"]
    p2 = v["p2"]
    le = v["le"]
    outfile.write(f'    fn data(&self) -> {tup_ty} {{\n')
    outfile.write(f'        {tup_val}\n')
    outfile.write("    }\n")
    outfile.write(f'    fn command(&self) -> CommandBuilder<{tup_ty}> {{\n')
    outfile.write(f'        CommandBuilder::new({cla}, {ins}, {p1}, {p2}, self.data(), {le})\n')
    outfile.write("    }\n")

    outfile.write("}\n")
    outfile.write("\n")

    outfile.write(f'impl{payload_lifetime} DataSource for {name}{payload_lifetime} {{\n')
    outfile.write('    fn len(&self) -> usize {\n')
    outfile.write('        self.command().len()\n')
    outfile.write('    }\n')
    outfile.write('    fn is_empty(&self) -> bool {\n')
    outfile.write('        self.command().is_empty()\n')
    outfile.write('    }\n')
    outfile.write("}\n")

    bound = "<W: Writer>"
    if payload_has_lifetime:
        bound = "<'data, W: Writer>"
        

    outfile.write(f'impl{bound} DataStream<W> for {name}{payload_lifetime} {{\n')
    outfile.write('    fn to_writer(&self, writer: &mut W) -> Result<(), <W as iso7816::command::Writer>::Error> {\n')
    outfile.write('        self.command().to_writer(writer)\n')
    outfile.write('    }\n')
    outfile.write("}\n")
   
    outfile.write("#[derive(Clone, Debug)]")
    outfile.write(f'pub struct {name}Response{response_lifetime} {{')

    if "response" in v:
        outfile.write("\n")

    for arg in v["response"].values():
        outfile.write(f'    pub {arg["name"]}: {arg["type"]},\n')
    outfile.write("}\n")

    outfile.write(f'\nimpl<\'data> TryFrom<&\'data [u8]> for {name}Response{response_lifetime} {{')
    outfile.write("    type Error = Error;\n")
    outfile.write("    fn try_from(rem: &'data [u8]) -> Result<Self, Error> {\n")
    for arg_name, arg in v["response"].items():
         parse_for_resp(arg, arg_name, outfile)
    outfile.write("        let _ = rem;\n")
    outfile.write(f'        Ok(Self {{ {", ".join([arg["name"] for arg in v["response"].values()])} }})\n')
    outfile.write("    }\n")
    outfile.write("}\n")

    outfile.write(f'impl{bound} Se050Command<W> for {name}{payload_lifetime} {{\n')
    if response_has_lifetime:
        outfile.write(f'    type Response<\'rdata> = {name}Response<\'rdata>;\n')
    else:
        outfile.write(f'    type Response<\'rdata> = {name}Response;\n')
    outfile.write("}\n")


outfile.flush()
