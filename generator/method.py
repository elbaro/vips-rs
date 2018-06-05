# Example input

# int
# vips_mosaic (VipsImage *ref,
#              VipsImage *sec,
#              VipsImage **out,
#              VipsDirection direction,
#              int xref,
#              int yref,
#              int xsec,
#              int ysec,
#              ...);

# Optional arguments:

#     bandno : gint, band to search for features

#     hwindow : gint, half window size

#     harea : gint, half search size

#     mblend : gint, maximum blend size

import sys

spec = sys.stdin.read()
cnt = 0
name = None
args = []
has_optional = False
has_out = False
optionals = []

for line in spec.splitlines():
    line = line.strip()
    if len(line)==0: continue
    cnt += 1

    if cnt == 1:
        ret = line
        continue
    elif cnt == 2:
        tokens = line.split(' ')
        name = tokens[0][5:]
        stars = len(tokens[2]) - len(tokens[2].lstrip('*'))
        args.append(('&'*stars + tokens[1][1:], tokens[2].lstrip('*').rstrip(',')))
    elif line == '...);':
        has_optional = True
    elif line == 'Optional arguments:':
        has_optional = True
    elif line == 'VipsImage **out,':
        has_out = True
        args.append(('&&VipsImage', 'out'))
    elif has_optional:
        tokens = line.split() # search : search to improve tie-points
        optionals.append(tokens[0])
    else:
        tokens = line.split(' ')
        stars = len(tokens[1]) - len(tokens[1].lstrip('*'))
        args.append(('&'*stars + tokens[0], tokens[1].lstrip('*').rstrip(',')))

template = '''pub fn %(name)s(&self%(args)s) %(ret)s{
    let mut out_ptr: *mut ffi::VipsImage = null_mut();
    let ret = unsafe {
        ffi::vips_%(name)s(
            self.c as *mut ffi::VipsImage,
%(params)s            null() as *const c_char)
    };
    result_with_ret(out_ptr, ret)
}'''

d = {
    'name': name,
    'ret': '',
    'args': '',
    'params': '',
}

if has_out:
    d['ret'] = '-> Result<VipsImage, Box<Error>> '
else:
    d['ret'] = '-> Result<(), Box<Error>> '

for arg in args:

    if arg[1]=='out':
        d['params'] += '            &mut out_ptr,\n'
    elif arg[0]=='&VipsImage':
        d['params'] += '            %s.c as *mut ffi::VipsImage,\n' % arg[1]
    else:
        d['params'] += '            %s,\n' % arg[1]


    if arg[1]=='out': continue

    if arg[0]=='int':
        arg=('i32',*arg[1:])

    d['args'] += ', %s: %s' % (arg[1],arg[0])

if has_optional:
    for arg in optionals:
        d['args'] += ', %s: Option<>' % arg
        d['params'] += '            "%s\0".as_ptr(),\n' % arg
        d['params'] += '            %s.unwrap_or(),\n' % arg

print(template % d)
